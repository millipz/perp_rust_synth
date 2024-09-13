use crate::envelope::{Envelope, EnvelopeParams, EnvelopeState};

/// Represents a single voice in the synthesizer.
///
/// A voice is a single sound-producing entity that can be controlled
/// independently. It has its own frequency, phase, amplitude, and envelope.
#[derive(Clone)]
pub struct Voice {
    /// The frequency of the voice, in Hz.
    pub frequency: f32,
    /// The current phase of the voice, in the range [0, 1).
    pub phase: f32,
    /// The amplitude of the voice, in the range [0, 1).
    amplitude: f32,
    /// The envelope of the voice, which controls its amplitude over time.
    envelope: Envelope,
    /// The number of updates that have been performed on the voice.
    update_count: usize,
}

impl Voice {
    /// Creates a new voice with the given frequency, velocity, and envelope parameters.
    ///
    /// The velocity is used to set the initial amplitude of the voice.
    ///
    /// # Parameters
    ///
    /// * `frequency`: The frequency of the voice, in Hz.
    /// * `velocity`: The velocity of the voice, in the range [0, 127].
    /// * `envelope_params`: The parameters of the envelope.
    ///
    /// # Returns
    ///
    /// A new voice with the given parameters.
    pub fn new(frequency: f32, velocity: u8, envelope_params: EnvelopeParams) -> Self {
        Voice {
            frequency,
            phase: 0.0,
            amplitude: velocity as f32 / 127.0,
            envelope: Envelope::new(envelope_params),
            update_count: 0,
        }
    }

    /// Updates the voice by the given time step.
    ///
    /// This method updates the phase and amplitude of the voice, and also
    /// updates the envelope.
    ///
    /// # Parameters
    ///
    /// * `dt`: The time step to update by.
    pub fn update(&mut self, dt: f32) {
        self.phase += self.frequency * dt;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        self.envelope.update(dt);

        // Debug print (every 44100 updates, which is about once per second at 44.1kHz)
        if self.update_count % 44100 == 0 {
            println!(
                "Voice update - Frequency: {:.2}, Amplitude: {:.4}, Is active: {}",
                self.frequency,
                self.current_amplitude(),
                self.is_active()
            );
        }
        self.update_count += 1;
    }

    pub fn is_active(&self) -> bool {
        self.envelope.is_active()
    }

    pub fn release(&mut self) {
        if !matches!(self.envelope.state, EnvelopeState::Release) {
            self.envelope.release();
            println!("Voice released - Frequency: {:.2}", self.frequency);
        }
    }

    pub fn current_amplitude(&self) -> f32 {
        self.amplitude * self.envelope.value()
    }
}
