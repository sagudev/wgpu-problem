use std::borrow::Cow;
use std::collections::HashMap;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BufferBinding, BufferDescriptor, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, DeviceDescriptor,
    Extent3d, Features, FragmentState, ImageCopyBuffer, Instance, InstanceDescriptor, Limits,
    MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    VertexState,
};

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}

async fn run() {
    let i = Instance::new(InstanceDescriptor {
        ..Default::default()
    });
    let adapter = i
        .request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
    device.on_uncaptured_error(Box::new(|err| println!("ERROR: {err:?}")));

    let buffer61 = device.create_buffer(&BufferDescriptor {
        label: None,
        size: 1024,
        usage: BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });
    let buffer71 = device.create_buffer(&BufferDescriptor {
        label: None,
        size: 1024,
        usage: BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });
    let buffer81 = device.create_buffer(&BufferDescriptor {
        label: None,
        size: 1024,
        usage: BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });
    let bind_group_layout_41 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    let bind_group21 = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout_41,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &buffer61,
                    offset: 0,
                    size: None,
                }),
            },
            BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &buffer71,
                    offset: 0,
                    size: None,
                }),
            },
            BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &buffer81,
                    offset: 0,
                    size: None,
                }),
            },
        ],
    });
    let shader21 = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(
            "
        @compute @workgroup_size(1)
        fn main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) {
        }",
        )),
    });
    let bind_group_layout_51 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    let pipeline_layout21 = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout_51],
        push_constant_ranges: &[],
    });
    let pipeline21 = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout21),
        module: &shader21,
        entry_point: "main",
        compilation_options: PipelineCompilationOptions::default(),
        cache: None,
    });
    // ...
    let mut cmd_enc51 = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    {
        let mut pass51 = cmd_enc51.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass51.set_pipeline(&pipeline21);
        pass51.set_bind_group(0, &bind_group21, &[]);
        pass51.dispatch_workgroups(0, 1, 1);
    } // pass end
    println!("End")
}
