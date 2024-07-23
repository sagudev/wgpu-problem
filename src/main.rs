use std::borrow::Cow;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BufferBinding, BufferDescriptor, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, DeviceDescriptor,
    Extent3d, FragmentState, FrontFace, Instance, InstanceDescriptor, MultisampleState, Operations,
    PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureDescriptor, TextureFormat,
    TextureUsages, TextureViewDescriptor, VertexState,
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
    device.on_uncaptured_error(Box::new(|err| println!("ERROR: {err:#?}")));

    let bind_group_layout_01 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[],
    });
    let bind_group_layout_11 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let pipeline_layout_01 = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &bind_group_layout_01,
            &bind_group_layout_01,
            &bind_group_layout_11,
            &bind_group_layout_11,
        ],
        push_constant_ranges: &[],
    });
    let (shader_01, shader_11, render_pipeline_01) = fun_name(&device, None);
    let (shader_21, shader_31, render_pipeline_11) = fun_name(&device, None);
    let (shader_41, shader_51, render_pipeline_21) = fun_name(&device, Some(&pipeline_layout_01));

    let buffer01 = device.create_buffer(&BufferDescriptor {
        label: None,
        size: 16,
        usage: BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });

    // Id(2,1,vk), Id(3,1,vk), Id(4,1,vk), Id(5,1,vk)]
    let bind_group_layout_21 = render_pipeline_01.get_bind_group_layout(0);
    let bind_group_layout_31 = render_pipeline_01.get_bind_group_layout(1);
    let bind_group_layout_41 = render_pipeline_01.get_bind_group_layout(2);
    let bind_group_layout_51 = render_pipeline_01.get_bind_group_layout(3);

    let bind_group_01 = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout_21,
        entries: &[],
    });
    let bind_group_11 = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout_31,
        entries: &[],
    });

    let bind_group_21 = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout_51,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(BufferBinding {
                buffer: &buffer01,
                offset: 0,
                size: None,
            }),
        }],
    });
    let bind_group_31 = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout_41,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(BufferBinding {
                buffer: &buffer01,
                offset: 0,
                size: None,
            }),
        }],
    });

    let mut cmd_enc01 = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    let tex = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let tex_view = tex.create_view(&TextureViewDescriptor {
        label: None,
        format: None,
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    });
    {
        let mut pass = cmd_enc01.begin_render_pass(&RenderPassDescriptor {
            label: None,
            timestamp_writes: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &tex_view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&render_pipeline_01);
        pass.set_bind_group(0, &bind_group_01, &[]);
        pass.set_bind_group(0, &bind_group_11, &[]);
        pass.set_bind_group(0, &bind_group_21, &[]);
        pass.set_bind_group(0, &bind_group_31, &[]);
        pass.draw(0..0, 0..1);
    } // pass end
    println!("End")
}

fn fun_name(
    device: &wgpu::Device,
    layout: Option<&wgpu::PipelineLayout>,
) -> (ShaderModule, ShaderModule, RenderPipeline) {
    let shader1 = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(
            "@group(2) @binding(0) var<uniform> u1: vec4f;
            @group(3) @binding(0) var<uniform> u2: vec4f;
            @vertex fn main() -> @builtin(position) vec4f { return u1 + u2; }
            ",
        )),
    });
    let shader2 = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed("@fragment fn main() {}")),
    });
    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout,
        vertex: VertexState {
            module: &shader1,
            entry_point: "main",
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
        },
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: 4294967295,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(FragmentState {
            module: &shader2,
            entry_point: "main",
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[Some(ColorTargetState {
                format: TextureFormat::Rgba8Unorm,
                blend: None,
                write_mask: ColorWrites::empty(),
            })],
        }),
        multiview: None,
        cache: None,
    });
    (shader1, shader2, render_pipeline)
}
