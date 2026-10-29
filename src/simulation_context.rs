use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct SimulationContext {
    n_grains: u64,
    dtype: u32,
    dt: f32,
}

impl SimulationContext {
    pub fn n_grains(&self) -> u64 {
        self.n_grains
    }

    pub fn dtype(&self) -> u32 {
        self.dtype
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }
}
