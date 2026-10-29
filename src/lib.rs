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

    pub fn context(&self) -> &SimulatorContext {
        &self.context
    }

    pub fn sim_context(&self) -> &SimulationContext {
        &self.sim_context
    }
}

#[cfg(test)]
mod tests {
    use pollster::FutureExt;
    use wgpu::Trace;

    use super::*;

    #[test]
    fn it_works() {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: Default::default(),
                compatible_surface: None,
                apply_limit_buckets: Default::default(),
            })
            .block_on()
            .expect("No compatible Adapter, cry");

        // let adapter = instance.request_adapter(RequestAdapterOptions {});

        println!("Chosan adapah : {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("LowPower Rendering Device"),
                required_features: Default::default(),
                required_limits: Default::default(),
                experimental_features: Default::default(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: Trace::Off,
            })
            .block_on()
            .expect("Can't create a Device and Queue, don't know how that could happen");

        let simulator = Simulator::new(
            device,
            queue,
            &String::from("test_simulation_state/latticeSmall"),
        );

        assert_eq!(4, 4);
    }
}
