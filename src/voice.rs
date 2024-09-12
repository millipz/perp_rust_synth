use crate::envelope::{Envelope, EnvelopeParams};

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

        // Debug print (every 100 updates)
        if self.update_count % 100 == 0 {
            println!(
                "Voice update - Frequency: {:.2}, Amplitude: {:.4}, Is active: {}",
                self.frequency,
                self.current_amplitude(),
                self.is_active()
            );
        }
        self.update_count += 1;
    }

    pub fn current_amplitude(&self) -> f32 {
        self.amplitude * self.envelope.value()
    }

    pub fn is_active(&self) -> bool {
        let is_active = self.envelope.is_active();
        if !is_active {
            println!("Voice became inactive - Frequency: {:.2}", self.frequency);
        }
        is_active
    }

    pub fn release(&mut self) {
        self.envelope.release();
    }
}
