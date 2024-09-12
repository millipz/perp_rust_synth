use std::time::Instant;

#[derive(Clone)]
pub struct EnvelopeParams {
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain_level: f32,
    pub release_time: f32,
}

impl Default for EnvelopeParams {
    fn default() -> Self {
        EnvelopeParams {
            attack_time: 0.005,
            decay_time: 0.02,
            sustain_level: 0.7,
            release_time: 0.01,
        }
    }
}

#[derive(Clone)]
pub struct Envelope {
    params: EnvelopeParams,
    pub state: EnvelopeState,
    value: f32,
    start_time: Instant,
}

#[derive(Clone, PartialEq)]
pub enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    pub fn new(params: EnvelopeParams) -> Self {
        Envelope {
            params,
            state: EnvelopeState::Attack,
            value: 0.0,
            start_time: Instant::now(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let target = match self.state {
            EnvelopeState::Attack => {
                let t = elapsed / self.params.attack_time;
                if t >= 1.0 {
                    self.state = EnvelopeState::Decay;
                    self.start_time = Instant::now();
                    println!("Envelope state changed to Decay");
                    1.0
                } else {
                    t
                }
            }
            EnvelopeState::Decay => {
                let t = elapsed / self.params.decay_time;
                let env = 1.0 + (self.params.sustain_level - 1.0) * t;
                if env <= self.params.sustain_level {
                    self.state = EnvelopeState::Sustain;
                    println!("Envelope state changed to Sustain");
                    self.params.sustain_level
                } else {
                    env
                }
            }
            EnvelopeState::Sustain => self.params.sustain_level,
            EnvelopeState::Release => {
                let t = elapsed / self.params.release_time;
                if t >= 1.0 {
                    println!("Envelope completed release");
                    0.0
                } else {
                    (1.0 - t) * self.value
                }
            }
        };

        // Use dt to smooth the envelope transition
        let smoothing_factor = 1.0 - (-50.0 * dt).exp();
        self.value += (target - self.value) * smoothing_factor;
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn is_active(&self) -> bool {
        self.value > 0.001 || self.state != EnvelopeState::Release
    }

    pub fn release(&mut self) {
        self.state = EnvelopeState::Release;
        self.start_time = Instant::now();
    }
}
