use crate::envelope::EnvelopeParams;
use crate::oscillator::Oscillator;
use crate::voice::Voice;
use std::collections::{HashMap, VecDeque};

struct Reverb {
    buffer: VecDeque<f32>,
    decay: f32,
}

impl Reverb {
    fn new(buffer_size: usize, decay: f32) -> Self {
        Reverb {
            buffer: VecDeque::with_capacity(buffer_size),
            decay,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = if let Some(&last) = self.buffer.back() {
            input + last * self.decay
        } else {
            input
        };

        self.buffer.push_back(output);
        if self.buffer.len() > self.buffer.capacity() {
            self.buffer.pop_front();
        }

        output
    }
}
/// Represents a synthesizer that can play multiple voices.
pub struct Synth {
    /// A map of active voices, keyed by note number.
    voices: HashMap<u8, Voice>,
    /// The sample rate of the synthesizer.
    sample_rate: f32,
    /// The envelope parameters used by the synthesizer.
    envelope_params: EnvelopeParams,
    /// The oscillator used by the synthesizer.
    oscillator: Oscillator,
    /// The current sample count.
    sample_count: usize,
    /// The reverb used by the synthesizer.
    reverb: Reverb,
}

impl Synth {
    /// Creates a new synthesizer with the given sample rate.
    ///
    /// # Arguments
    ///
    /// * `sample_rate`: The sample rate of the synthesizer.
    ///
    /// # Returns
    ///
    /// A new synthesizer instance.
    pub fn new(sample_rate: f32) -> Self {
        Synth {
            voices: HashMap::new(),
            sample_rate,
            envelope_params: EnvelopeParams::default(),
            oscillator: Oscillator::new(sample_rate),
            sample_count: 0,
            reverb: Reverb::new((sample_rate * 0.1) as usize, 0.2), // 100ms delay, 0.2 decay
        }
    }

    /// Turns on a note with the given note number and velocity.
    ///
    /// If a voice is already playing the given note, it will not be restarted.
    ///
    /// # Arguments
    ///
    /// * `note`: The note number to turn on.
    /// * `velocity`: The velocity of the note.
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

    /// Turns off a note with the given note number.
    ///
    /// If a voice is playing the given note, it will be released.
    ///
    /// # Arguments
    ///
    /// * `note`: The note number to turn off.
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
        let clipped_output = output.tanh(); // Soft clipping

        // Apply reverb
        let reverb_output = self.reverb.process(clipped_output);

        // Mix dry and wet signals
        let final_output = 0.7 * clipped_output + 0.3 * reverb_output;

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
