use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Stream, StreamConfig, SupportedStreamConfig};
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::synth::Synth;

pub fn get_audio_device() -> Result<(Device, SupportedStreamConfig), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No output device available")?;
    let config = device.default_output_config()?;

    println!("Default output device: {}", device.name()?);
    println!("Default output config: {:?}", config);

    Ok((device, config))
}

pub fn setup_audio_stream(
    device: &Device,
    config: &SupportedStreamConfig,
    synth: Arc<Mutex<Synth>>,
) -> Result<Stream, Box<dyn Error>> {
    let config: StreamConfig = config.clone().into();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, config.channels as usize, &mut || {
                synth.lock().unwrap().generate_sample()
            })
        },
        |err| eprintln!("An error occurred on stream: {}", err),
        None,
    )?;

    Ok(stream)
}

fn write_data(output: &mut [f32], channels: usize, next_sample: &mut dyn FnMut() -> f32) {
    for frame in output.chunks_mut(channels) {
        let value = next_sample();
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
