use crate::envelope::{Envelope, EnvelopeParams, EnvelopeState};

#[derive(Clone)]
pub struct Voice {
    pub frequency: f32,
    pub phase: f32,
    amplitude: f32,
    envelope: Envelope,
    update_count: usize,
}

impl Voice {
    pub fn new(frequency: f32, velocity: u8, envelope_params: EnvelopeParams) -> Self {
        Voice {
            frequency,
            phase: 0.0,
            amplitude: velocity as f32 / 127.0,
            envelope: Envelope::new(envelope_params),
            update_count: 0,
        }
    }

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
