use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct SimulationContext {
    n_grains: u64,
    dtype: u32,
    dt: f32,
}

impl SimulationContext {
    pub(crate) fn n_grains(&self) -> u64 {
        self.n_grains
    }

    pub(crate) fn dtype(&self) -> u32 {
        self.dtype
    }

    pub(crate) fn dt(&self) -> f32 {
        self.dt
    }
}
