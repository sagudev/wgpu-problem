use std::borrow::Cow;
use std::collections::HashMap;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BufferDescriptor, BufferUsages,
    Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, ComputePipelineDescriptor,
    DeviceDescriptor, Extent3d, Features, FragmentState, ImageCopyBuffer, Instance,
    InstanceDescriptor, Limits, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureViewDescriptor, VertexState,
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
    let (device, _queue) = adapter
        .request_device(&DeviceDescriptor::default(), None)
        .await
        .unwrap();
    device.on_uncaptured_error(Box::new(|err| println!("ERROR: {err:?}")));
    let shader01 = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Owned(
            "
@vertex
fn main()-> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}"
            .to_string(),
        )),
    });
    let shader11 = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Owned(
            "
@group(0) @binding(0) var<uniform> binding: f32;

@fragment
fn main() -> @location(0) vec4<f32> {
    _ = binding;
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}"
            .to_string(),
        )),
    });

    let shader21 = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Owned(
            "
@group(0) @binding(0) var<uniform> binding: f32;

@compute @workgroup_size(1) fn main(
  @builtin(global_invocation_id) id: vec3<u32>
) {
    _ = binding;
}
"
            .to_string(),
        )),
    });

    // explicit
    #[cfg(feature = "explicit")]
    let explicit_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[],
    });
    #[cfg(feature = "explicit")]
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&explicit_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        #[cfg(feature = "explicit")]
        layout: Some(&pipeline_layout),
        #[cfg(not(feature = "explicit"))]
        layout: None, // auto
        module: &shader21,
        entry_point: Some("main"),
        compilation_options: PipelineCompilationOptions::default(),
        cache: None,
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        #[cfg(feature = "explicit")]
        layout: Some(&pipeline_layout),
        #[cfg(not(feature = "explicit"))]
        layout: None, // auto
        vertex: VertexState {
            module: &shader01,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions {
                constants: &HashMap::new(),
                zero_initialize_workgroup_memory: true,
            },
            buffers: &[],
        },
        primitive: PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: 4294967295,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(FragmentState {
            module: &shader11,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions {
                constants: &HashMap::new(),
                zero_initialize_workgroup_memory: true,
            },
            targets: &[Some(ColorTargetState {
                format: TextureFormat::Rgba8Unorm,
                blend: None,
                write_mask: ColorWrites::RED
                    | ColorWrites::GREEN
                    | ColorWrites::BLUE
                    | ColorWrites::ALPHA,
            })],
        }),
        multiview: None,
        cache: None,
    });

    compute_pipeline.get_bind_group_layout(0); // panics if not explicitly given layout
    render_pipeline.get_bind_group_layout(0); // panics if not explicitly given layout
}
