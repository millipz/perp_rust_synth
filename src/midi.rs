use midir::{Ignore, MidiInput, MidiInputConnection};
use std::error::Error;
use std::sync::{Arc, Mutex};

use super::synth::Synth;

/// Sets up a MIDI input connection to receive MIDI messages.
///
/// This function takes a `MidiInput` instance and a shared `Synth` instance,
/// and returns a `MidiInputConnection` instance that can be used to receive
/// MIDI messages.
///
/// # Errors
///
/// If there is an error setting up the MIDI input connection, a `Box<dyn Error>`
/// instance is returned.
pub fn setup_midi_input(
    mut midi_in: MidiInput,
    synth: Arc<Mutex<Synth>>,
) -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    // Ignore all MIDI messages that are not note on or note off messages.
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = in_ports.get(0).ok_or("No MIDI input port found")?;
    let port_name = midi_in.port_name(in_port)?;

    let conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            println!("Received MIDI message: {:?}", message); // Add this line
            if message.len() == 3 {
                let mut synth = synth.lock().unwrap();
                match message[0] {
                    144 => {
                        println!("Note On: {}, Velocity: {}", message[1], message[2]); // Add this line
                        synth.note_on(message[1], message[2])
                    }
                    128 => {
                        println!("Note Off: {}", message[1]); // Add this line
                        synth.note_off(message[1])
                    }
                    _ => {}
                }
            }
        },
        (),
    )?;

    println!("Connection open, reading from port '{}'", port_name);

    Ok(conn_in)
}
