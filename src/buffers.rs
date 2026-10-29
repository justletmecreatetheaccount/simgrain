use wgpu::{Buffer, BufferDescriptor, Device};

pub struct Buffers {
    hot_buffer: Buffer,
    cold_buffer: Buffer,
}

impl Buffers {
    pub(crate) fn new(device: &Device, number_grains: u64, number_attributes: u32) -> Buffers {
        let hot_buffer_desc = BufferDescriptor {
            label: Some("Hot Buffer"),
            size: number_grains * (3 * 8 + 3 * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        let cold_buffer_desc = BufferDescriptor {
            label: Some("Cold Buffer"),
            size: (number_attributes * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        let hot_buffer = device.create_buffer(&hot_buffer_desc);
        let cold_buffer = device.create_buffer(&cold_buffer_desc);

        Buffers {
            hot_buffer,
            cold_buffer,
        }
    }

    pub(crate) fn hot_buffer(&self) -> &Buffer {
        &self.hot_buffer
    }

    pub(crate) fn cold_buffer(&self) -> &Buffer {
        &self.cold_buffer
    }
}
