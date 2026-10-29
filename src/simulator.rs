use std::{
    io::Read,
    path::{Path, PathBuf},
};

use wgpu::{Device, Queue};

use crate::{buffers::Buffers, simulation_context::SimulationContext};

// Device and Queue are a cheap Arc bump beneath the hood (Normally)
pub struct SimulatorContext {
    device: Device,
    queue: Queue,
    buffers: Buffers,
}

impl SimulatorContext {
    pub(crate) fn new(
        device: Device,
        queue: Queue,
        config_dir_path: &String,
    ) -> (SimulatorContext, SimulationContext) {
        let dir_path = PathBuf::from(config_dir_path);
        let simulation_context: SimulationContext = toml::from_str(
            std::fs::read_to_string(&dir_path.join("config.toml"))
                .expect(
                    &format!(
                        "Couldn't read simulation config file: {}",
                        &dir_path
                            .join("config.toml")
                            .to_str()
                            .expect("Couldn't transform path to string to give debug data"),
                    )
                    .into_boxed_str(),
                )
                .as_str(),
        )
        .expect("Could turn string to toml object");

        let number_attributes: u32 = 1; // Number of attributes per grain

        let buffers = Buffers::new(&device, simulation_context.n_grains(), number_attributes);

        // Load data in buffers

        // POSITIONS vec3<f64>
        let mut buf = vec![];
        std::fs::File::open(&dir_path.join("positions.bin"))
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();

        queue.write_buffer(buffers.hot_buffer(), 0, &buf);

        // RADII
        let mut buf = vec![];
        std::fs::File::open(&dir_path.join("radii.bin"))
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();

        queue.write_buffer(buffers.cold_buffer(), 0, &buf);

        (
            SimulatorContext {
                device,
                queue,
                buffers,
            },
            simulation_context,
        )
    }

    pub fn buffers(&self) -> &Buffers {
        &self.buffers
    }
}
