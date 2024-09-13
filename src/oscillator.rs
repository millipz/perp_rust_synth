/// A simple oscillator that generates a waveform.
pub struct Oscillator {
    /// The sample rate of the oscillator.
    sample_rate: f32,
}

impl Oscillator {
    /// Creates a new oscillator with the given sample rate.
    ///
    /// # Arguments
    ///
    /// * `sample_rate`: The sample rate of the oscillator.
    pub fn new(sample_rate: f32) -> Self {
        Oscillator { sample_rate }
    }

    /// Generates a sample of the waveform at the given phase.
    ///
    /// # Arguments
    ///
    /// * `phase`: The phase of the waveform, in the range [0, 1).
    ///
    /// # Returns
    ///
    /// The generated sample value.
    pub fn generate(&self, phase: f32) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let mut sample = 2.0 * phase - 1.0;
        sample -= Self::poly_blep(phase, dt);
        sample
    }

    /// Computes the polyblep correction for the given phase and sample period.
    ///
    /// # Arguments
    ///
    /// * `t`: The phase of the waveform, in the range [0, 1).
    /// * `dt`: The sample period.
    ///
    /// # Returns
    ///
    /// The polyblep correction value.
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
