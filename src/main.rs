use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use midir::{Ignore, MidiInput};
use std::collections::HashMap;
use std::error::Error;
use std::f32::consts::PI;
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::time::Instant;

struct Voice {
    frequency: f32,
    phase: f32,
    amplitude: f32,
    start_time: Instant,
    is_active: bool,
}

struct Synth {
    voices: HashMap<u8, Voice>,
    sample_rate: f32,
    attack_time: f32,
    release_time: f32,
}

impl Synth {
    fn new(sample_rate: f32) -> Self {
        Synth {
            voices: HashMap::new(),
            sample_rate,
            attack_time: 0.01, // 10ms attack
            release_time: 0.1, // 100ms release
        }
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        let freq = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);
        self.voices.insert(
            note,
            Voice {
                frequency: freq,
                phase: 0.0,
                amplitude: velocity as f32 / 127.0,
                start_time: Instant::now(),
                is_active: true,
            },
        );
    }

    fn note_off(&mut self, note: u8) {
        if let Some(voice) = self.voices.get_mut(&note) {
            voice.is_active = false;
            voice.start_time = Instant::now(); // Reset start time for release phase
        }
    }

    fn generate_sample(&mut self) -> f32 {
        let now = Instant::now();
        let mut sum = 0.0;
        let mut active_voices = 0;

        self.voices.retain(|_, voice| {
            let elapsed = now.duration_since(voice.start_time).as_secs_f32();
            let envelope = if voice.is_active {
                // Note is still pressed, apply attack
                (elapsed / self.attack_time).min(1.0)
            } else {
                // Note is released, apply exponential release
                (-elapsed / self.release_time).exp()
            };

            if envelope > 0.001 {
                // Add a small threshold to remove very quiet voices
                sum += (voice.phase * 2.0 * PI).sin() * voice.amplitude * envelope;
                voice.phase += voice.frequency / self.sample_rate;
                if voice.phase >= 1.0 {
                    voice.phase -= 1.0;
                }
                active_voices += 1;
                true // Keep this voice
            } else {
                false // Remove this voice
            }
        });

        // Normalize
        let output = if active_voices > 0 {
            sum / (active_voices as f32).sqrt()
        } else {
            0.0
        };

        // Soft clipping
        if output.abs() <= 0.8 {
            output
        } else {
            output.signum() * (0.8 + 0.2 * (1.0 - (-5.0 * (output.abs() - 0.8)).exp()))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config()?;

    println!("Default output device: {}", device.name()?);
    println!("Default output config: {:?}", config);

    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let synth = Arc::new(Mutex::new(Synth::new(sample_rate)));
    let synth_clone = synth.clone();

    let mut next_value = move || synth_clone.lock().unwrap().generate_sample();

    let stream = match config.sample_format() {
        SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            |err| eprintln!("An error occurred on stream: {}", err),
            None,
        )?,
        _ => return Err("Unsupported sample format".into()),
    };

    stream.play()?;

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_port = &midi_in.ports()[0];
    let port_name = midi_in.port_name(in_port)?;

    let synth_clone = synth.clone();
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if message.len() == 3 {
                let mut synth = synth_clone.lock().unwrap();
                match message[0] {
                    144 => synth.note_on(message[1], message[2]),
                    128 => synth.note_off(message[1]),
                    _ => {}
                }
            }
        },
        (),
    )?;

    println!(
        "\nConnection open, reading from port '{}'. Press Enter to exit...",
        port_name
    );
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    println!("Closing connection");
    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value = next_sample();
        for sample in frame.iter_mut() {
            *sample = cpal::Sample::from_sample(value);
        }
    }
}
