use wgpu::{Device, Queue};

use crate::{simulation_context::SimulationContext, simulator::SimulatorContext};

mod buffers;
mod resource_manager;
mod simulation_context;
mod simulator;

pub struct Simulator {
    context: SimulatorContext,
    sim_context: SimulationContext,
}

impl Simulator {
    pub fn new(device: Device, queue: Queue, config_file_path: &String) -> Simulator {
        let context_pair = SimulatorContext::new(device, queue, config_file_path);
        Simulator {
            context: context_pair.0,
            sim_context: context_pair.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
