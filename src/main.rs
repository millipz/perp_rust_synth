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

fn main() -> Result<(), Box<dyn Error>> {
    let (device, config) = audio::get_audio_device()?;
    let sample_rate = config.sample_rate().0 as f32;

    let synth = Arc::new(Mutex::new(Synth::new(sample_rate)));
    let stream = audio::setup_audio_stream(&device, &config, synth.clone())?;
    stream.play()?;

    let midi_in = MidiInput::new("midir reading input")?;
    let _conn_in = midi::setup_midi_input(midi_in, synth.clone())?;

    println!("Synth is running. Press Enter to exit...");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    println!("Closing connection");
    Ok(())
}
