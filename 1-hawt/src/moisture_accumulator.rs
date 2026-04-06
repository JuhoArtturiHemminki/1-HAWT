use crate::fractal_feedback::{PHI, H_C};

pub struct MoistureAccumulator {
    pub target_dew_point: f64,
    pub yield_ml_sim: f64,
    pub uv_pulse_active: bool,
}

impl MoistureAccumulator {
    pub const fn new() -> Self {
        Self {
            target_dew_point: 12.0,
            yield_ml_sim: 0.0,
            uv_pulse_active: false,
        }
    }

    pub fn track_condensation(&mut self, current_temp: f64, drift: &mut 
f64) -> f64 {
        let delta = current_temp - self.target_dew_point;
        if delta > 0.0 {
            *drift += (delta * PHI) / H_C;
            self.yield_ml_sim += 0.001 * PHI;
            self.uv_pulse_active = true;
        } else {
            *drift -= (delta.abs() * PHI) / H_C;
            self.uv_pulse_active = false;
        }
        self.yield_ml_sim
    }
}

