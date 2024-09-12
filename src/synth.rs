use crate::envelope::EnvelopeParams;
use crate::oscillator::Oscillator;
use crate::voice::Voice;
use std::collections::HashMap;

pub struct Synth {
    voices: HashMap<u8, Voice>,
    sample_rate: f32,
    envelope_params: EnvelopeParams,
    oscillator: Oscillator,
    sample_count: usize,
}

impl Synth {
    pub fn new(sample_rate: f32) -> Self {
        Synth {
            voices: HashMap::new(),
            sample_rate,
            envelope_params: EnvelopeParams::default(),
            oscillator: Oscillator::new(sample_rate),
            sample_count: 0,
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let freq = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);

        // Only create a new voice if one doesn't already exist for this note
        if !self.voices.contains_key(&note) {
            let voice = Voice::new(freq, velocity, self.envelope_params.clone());
            self.voices.insert(note, voice);
            println!(
                "Note ON - Note: {}, Velocity: {}, Frequency: {:.2}",
                note, velocity, freq
            );
        }
    }

    pub fn note_off(&mut self, note: u8) {
        if let Some(voice) = self.voices.get_mut(&note) {
            voice.release();
            println!("Note OFF - Note: {} (releasing)", note);
        } else {
            println!("Note OFF - Note: {} (not found in active voices)", note);
        }
        println!("Active voices after note off: {}", self.voices.len());
    }

    pub fn generate_sample(&mut self) -> f32 {
        let mut sum = 0.0;
        let active_voices = self.voices.len() as f32;

        for (_note, voice) in self.voices.iter_mut() {
            if voice.is_active() {
                let sample = self.oscillator.generate(voice.phase) * voice.current_amplitude();
                sum += sample;
                voice.update(1.0 / self.sample_rate);
            }
        }

        // Remove inactive voices
        self.voices.retain(|_, voice| voice.is_active());

        // Increase the volume and apply soft clipping
        let output = if active_voices > 0.0 {
            (sum / active_voices.sqrt()) * 2.0 // Increase volume
        } else {
            0.0
        };
        let final_output = output.tanh(); // Soft clipping

        // Debug output (print every 44100 samples, which is about once per second at 44.1kHz)
        if self.sample_count % 44100 == 0 {
            println!(
                "Active voices: {}, Output: {:.4}",
                self.voices.len(),
                final_output
            );
        }
        self.sample_count += 1;

        final_output
    }
}
