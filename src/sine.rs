use std::f64::consts::PI;

pub const SAMPLE_RATE: u32 = 44_100;
pub const BIT_DEPTH: u32 = 16;
pub const MAX_AMPLITUDE: u32 = 2_u32.pow(BIT_DEPTH - 1) - 1;

pub struct SineOscillator {
    frequency: f64,
    amplitude: f64,
    angle: f64,
    offset: f64,
}

impl SineOscillator {
    pub fn new(frequency: f64, amplitude: f64) -> Self {
        SineOscillator {
            frequency,
            amplitude,
            angle: 0.0,
            offset: 2_f64 * PI * frequency / SAMPLE_RATE as f64,
        }
    }

    pub fn process(&mut self) -> f64 {
        let sample = self.amplitude * self.angle.sin();
        self.angle += self.offset;
        sample
    }
}
