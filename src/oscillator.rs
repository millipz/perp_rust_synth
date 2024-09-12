pub struct Oscillator {
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Oscillator { sample_rate }
    }

    pub fn generate(&self, phase: f32) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let mut sample = 2.0 * phase - 1.0;
        sample -= Self::poly_blep(phase, dt);
        sample
    }

    fn poly_blep(t: f32, dt: f32) -> f32 {
        if t < dt {
            let t = t / dt;
            2.0 * t - t * t - 1.0
        } else if t > 1.0 - dt {
            let t = (t - 1.0) / dt;
            t * t + 2.0 * t + 1.0
        } else {
            0.0
        }
    }
}
