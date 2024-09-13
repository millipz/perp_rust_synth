mod audio;
mod envelope;
mod midi;
mod oscillator;
mod synth;
mod voice;

use cpal::traits::StreamTrait;
use midir::MidiInput;
use std::error::Error;
use std::io::stdin;
use std::sync::{Arc, Mutex};

use synth::Synth;

/// The main entry point of the program.
///
/// This function sets up the audio device, MIDI input, and synthesizer,
/// and runs the main loop of the program.
///
/// # Errors
///
/// If there is an error setting up the audio device, MIDI input, or synthesizer,
/// a `Box<dyn Error>` instance is returned.
fn main() -> Result<(), Box<dyn Error>> {
    // Get the default audio device and its configuration.
    let (device, config) = audio::get_audio_device()?;

    // Get the sample rate of the audio device.
    let sample_rate = config.sample_rate().0 as f32;

    // Create a new synthesizer instance with the given sample rate.
    let synth = Arc::new(Mutex::new(Synth::new(sample_rate)));

    // Set up the audio stream with the given device, configuration, and synthesizer.
    let stream = audio::setup_audio_stream(&device, &config, synth.clone())?;

    // Start playing the audio stream.
    stream.play()?;

    // Create a new MIDI input instance.
    let midi_in = MidiInput::new("midir reading input")?;

    // Set up the MIDI input connection with the given MIDI input and synthesizer.
    let _conn_in = midi::setup_midi_input(midi_in, synth.clone())?;

    // Print a message to the console indicating that the synthesizer is running.
    println!("Synth is running. Press Enter to exit...");

    // Read a line of input from the console.
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    // Print a message to the console indicating that the connection is closing.
    println!("Closing connection");

    // Return an empty result to indicate that the program executed successfully.
    Ok(())
}
