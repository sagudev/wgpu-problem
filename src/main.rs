use std::borrow::Cow::{self, Borrowed};
use std::collections::HashMap;

use wgpu_core::api;
use wgpu_core::binding_model::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindingResource, BufferBinding,
    PipelineLayoutDescriptor,
};
use wgpu_core::command::{PassChannel, RenderPassColorAttachment, RenderPassDescriptor, StoreOp};
use wgpu_core::device::ImplicitPipelineIds;
use wgpu_core::global::Global;
use wgpu_core::hal_api::HalApi;
use wgpu_core::identity::IdentityManager;
use wgpu_core::instance::{AdapterInputs, Instance};
use wgpu_core::naga::Module;
use wgpu_core::pipeline::{
    FragmentState, ProgrammableStageDescriptor, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderModuleSource, VertexState,
};
use wgpu_core::resource::TextureDescriptor;
use wgpu_types::Backend::Vulkan;
use wgpu_types::{
    Backends, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferDescriptor, BufferUsages,
    Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, DeviceDescriptor, Extent3d,
    FrontFace, ImageSubresourceRange, InstanceDescriptor, MultisampleState, PolygonMode,
    PowerPreference, PrimitiveState, PrimitiveTopology, RequestAdapterOptions, ShaderBoundChecks,
    ShaderStages, TextureAspect, TextureDimension, TextureFormat, TextureUsages,
};

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}

use wgpu_core::id::markers::{
    Adapter, BindGroup, BindGroupLayout, Buffer, CommandEncoder, ComputePipeline, Device,
    PipelineLayout, RenderBundle, RenderPipeline, Sampler, ShaderModule, Texture, TextureView,
};
use wgpu_core::id::{
    AdapterId, BindGroupId, BindGroupLayoutId, BufferId, CommandEncoderId, ComputePipelineId,
    DeviceId, Id, Marker, PipelineLayoutId, RenderBundleId, RenderPipelineId, SamplerId,
    ShaderModuleId, TextureId, TextureViewId,
};

use wgpu_core::command::DynRenderPass;

pub struct IdentityHub {
    adapters: IdentityManager<Adapter>,
    devices: IdentityManager<Device>,
    buffers: IdentityManager<Buffer>,
    bind_groups: IdentityManager<BindGroup>,
    bind_group_layouts: IdentityManager<BindGroupLayout>,
    compute_pipelines: IdentityManager<ComputePipeline>,
    pipeline_layouts: IdentityManager<PipelineLayout>,
    shader_modules: IdentityManager<ShaderModule>,
    command_encoders: IdentityManager<CommandEncoder>,
    textures: IdentityManager<Texture>,
    texture_views: IdentityManager<TextureView>,
    samplers: IdentityManager<Sampler>,
    render_pipelines: IdentityManager<RenderPipeline>,
    render_bundles: IdentityManager<RenderBundle>,
}

impl IdentityHub {
    fn new() -> Self {
        IdentityHub {
            adapters: IdentityManager::new(),
            devices: IdentityManager::new(),
            buffers: IdentityManager::new(),
            bind_groups: IdentityManager::new(),
            bind_group_layouts: IdentityManager::new(),
            compute_pipelines: IdentityManager::new(),
            pipeline_layouts: IdentityManager::new(),
            shader_modules: IdentityManager::new(),
            command_encoders: IdentityManager::new(),
            textures: IdentityManager::new(),
            texture_views: IdentityManager::new(),
            samplers: IdentityManager::new(),
            render_pipelines: IdentityManager::new(),
            render_bundles: IdentityManager::new(),
        }
    }
}

fn er<E: std::fmt::Debug>(e: Option<E>) {
    if let Some(e) = e {
        panic!("{e:?}");
    }
}

fn id<T: Marker>(id: Id<T>) -> String {
    format!("{id:?}")
}

async fn run() {
    let global = Global::new(
        "lol",
        InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        },
    );
    let hub = IdentityHub::new();
    let adapter_id = global
        .request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: None,
            },
            AdapterInputs::IdSet(&[hub.adapters.process(Vulkan)]),
        )
        .unwrap();
    assert_eq!(id(adapter_id), "Id(0,1,vk)");
    let did = hub.devices.process(Vulkan);
    let (device_id, queue_id, error) = global.adapter_request_device::<api::Vulkan>(
        adapter_id,
        &DeviceDescriptor {
            label: None,
            ..Default::default()
        },
        None,
        Some(did),
        Some(did.into_queue_id()),
    );
    er(error);
    assert_eq!(id(device_id), "Id(0,1,vk)");

    let (bind_group_layout_01, error) = global.device_create_bind_group_layout::<api::Vulkan>(
        device_id,
        &BindGroupLayoutDescriptor {
            label: None,
            entries: Cow::Borrowed(&[]),
        },
        Some(hub.bind_group_layouts.process(Vulkan)),
    );
    er(error);
    assert_eq!(id(bind_group_layout_01), "Id(0,1,vk)");

    let (bind_group_layout_11, error) = global.device_create_bind_group_layout::<api::Vulkan>(
        device_id,
        &BindGroupLayoutDescriptor {
            label: None,
            entries: Borrowed(&[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }]),
        },
        Some(hub.bind_group_layouts.process(Vulkan)),
    );
    er(error);
    assert_eq!(id(bind_group_layout_11), "Id(1,1,vk)");

    let (pipeline_layout_01, error) = global.device_create_pipeline_layout::<api::Vulkan>(
        device_id,
        &PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: Borrowed(&[
                bind_group_layout_01,
                bind_group_layout_01,
                bind_group_layout_11,
                bind_group_layout_11,
            ]),
            push_constant_ranges: Borrowed(&[]),
        },
        Some(hub.pipeline_layouts.process(Vulkan)),
    );
    er(error);
    assert_eq!(id(pipeline_layout_01), "Id(0,1,vk)");

    // Id(2,1,vk), Id(3,1,vk), Id(4,1,vk), Id(5,1,vk)]
    let bind_group_layout_21 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_21), "Id(2,1,vk)");
    let bind_group_layout_31 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_31), "Id(3,1,vk)");
    let bind_group_layout_41 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_41), "Id(4,1,vk)");
    let bind_group_layout_51 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_51), "Id(5,1,vk)");

    let pipeline_layout_11 = hub.pipeline_layouts.process(Vulkan);
    assert_eq!(id(pipeline_layout_11), "Id(1,1,vk)");

    let (shader_01, shader_11, render_pipeline_01) = fun_name(
        device_id,
        &global,
        &hub,
        None,
        Some(ImplicitPipelineIds {
            root_id: pipeline_layout_11,
            group_ids: &[
                bind_group_layout_21,
                bind_group_layout_31,
                bind_group_layout_41,
                bind_group_layout_51,
            ],
        }),
    );
    assert_eq!(id(shader_01), "Id(0,1,vk)");
    assert_eq!(id(shader_11), "Id(1,1,vk)");
    assert_eq!(id(render_pipeline_01), "Id(0,1,vk)");

    let bind_group_layout_61 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_61), "Id(6,1,vk)");
    let bind_group_layout_71 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_71), "Id(7,1,vk)");
    let bind_group_layout_81 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_81), "Id(8,1,vk)");
    let bind_group_layout_91 = hub.bind_group_layouts.process(Vulkan);
    assert_eq!(id(bind_group_layout_91), "Id(9,1,vk)");

    let pipeline_layout_21 = hub.pipeline_layouts.process(Vulkan);
    assert_eq!(id(pipeline_layout_21), "Id(2,1,vk)");

    let (shader_21, shader_31, render_pipeline_11) = fun_name(
        device_id,
        &global,
        &hub,
        None,
        Some(ImplicitPipelineIds {
            root_id: pipeline_layout_21,
            group_ids: &[
                bind_group_layout_61,
                bind_group_layout_71,
                bind_group_layout_81,
                bind_group_layout_91,
            ],
        }),
    );
    assert_eq!(id(shader_21), "Id(2,1,vk)");
    assert_eq!(id(shader_31), "Id(3,1,vk)");
    assert_eq!(id(render_pipeline_11), "Id(1,1,vk)");
    let (shader_41, shader_51, render_pipeline_21) =
        fun_name(device_id, &global, &hub, Some(pipeline_layout_01), None);
    assert_eq!(id(shader_41), "Id(4,1,vk)");
    assert_eq!(id(shader_51), "Id(5,1,vk)");
    assert_eq!(id(render_pipeline_21), "Id(2,1,vk)");

    let (buffer_id, error) = global.device_create_buffer::<api::Vulkan>(
        device_id,
        &BufferDescriptor {
            label: None,
            size: 16,
            usage: BufferUsages::UNIFORM,
            mapped_at_creation: false,
        },
        Some(hub.buffers.process(Vulkan)),
    );
    assert_eq!(id(buffer_id), "Id(0,1,vk)");
    er(error);

    let (bind_group_01, error) = global.device_create_bind_group::<api::Vulkan>(
        device_id,
        &BindGroupDescriptor {
            label: None,
            layout: bind_group_layout_21,
            entries: Borrowed(&[]),
        },
        Some(hub.bind_groups.process(Vulkan)),
    );
    assert_eq!(id(bind_group_01), "Id(0,1,vk)");
    er(error);

    let (bind_group_11, error) = global.device_create_bind_group::<api::Vulkan>(
        device_id,
        &BindGroupDescriptor {
            label: None,
            layout: bind_group_layout_31,
            entries: Borrowed(&[]),
        },
        Some(hub.bind_groups.process(Vulkan)),
    );
    assert_eq!(id(bind_group_11), "Id(1,1,vk)");
    er(error);

    let (bind_group_21, error) = global.device_create_bind_group::<api::Vulkan>(
        device_id,
        &BindGroupDescriptor {
            label: None,
            layout: bind_group_layout_51,
            entries: Borrowed(&[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer_id,
                    offset: 0,
                    size: None,
                }),
            }]),
        },
        Some(hub.bind_groups.process(Vulkan)),
    );
    assert_eq!(id(bind_group_21), "Id(2,1,vk)");
    er(error);

    let (bind_group_31, error) = global.device_create_bind_group::<api::Vulkan>(
        device_id,
        &BindGroupDescriptor {
            label: None,
            layout: bind_group_layout_41,
            entries: Borrowed(&[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer_id,
                    offset: 0,
                    size: None,
                }),
            }]),
        },
        Some(hub.bind_groups.process(Vulkan)),
    );
    assert_eq!(id(bind_group_31), "Id(3,1,vk)");
    er(error);

    let (cmd_enc01, error) = global.device_create_command_encoder::<api::Vulkan>(
        device_id,
        &CommandEncoderDescriptor { label: None },
        Some(hub.command_encoders.process(Vulkan)),
    );
    assert_eq!(id(cmd_enc01), "Id(0,1,vk)");
    er(error);

    let (tex, error) = global.device_create_texture::<api::Vulkan>(
        device_id,
        &TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: Vec::new(),
        },
        Some(hub.textures.process(Vulkan)),
    );
    assert_eq!(id(tex), "Id(0,1,vk)");
    er(error);

    let (tex_view, error) = global.texture_create_view::<api::Vulkan>(
        tex,
        &wgpu_core::resource::TextureViewDescriptor {
            label: None,
            format: None,
            dimension: None,
            range: ImageSubresourceRange {
                aspect: TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: 0,
                array_layer_count: None,
            },
        },
        Some(hub.texture_views.process(Vulkan)),
    );
    assert_eq!(id(tex_view), "Id(0,1,vk)");
    er(error);

    {
        let (mut pass, error) = global.command_encoder_create_render_pass::<api::Vulkan>(
            cmd_enc01,
            &RenderPassDescriptor {
                label: None,
                timestamp_writes: None,
                color_attachments: Borrowed(&[
                    // 1.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 2.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 3.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 4.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 5.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 6.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 7.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 8.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                    // 9.
                    Some(RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        channel: PassChannel {
                            load_op: wgpu_core::command::LoadOp::Clear,
                            store_op: wgpu_core::command::StoreOp::Store,
                            clear_value: Color::TRANSPARENT,
                            read_only: false,
                        },
                    }),
                ]),
                depth_stencil_attachment: None,
                occlusion_query_set: None,
            },
        );
        er(error);
        pass.set_pipeline(&global, render_pipeline_01).unwrap();
        pass.set_bind_group(&global, 0, bind_group_01, &[]).unwrap();
        pass.set_bind_group(&global, 1, bind_group_11, &[]).unwrap();
        pass.set_bind_group(&global, 2, bind_group_21, &[]).unwrap();
        pass.set_bind_group(&global, 3, bind_group_31, &[]).unwrap();
        pass.draw(&global, 0, 1, 0, 0).unwrap();
        pass.end(&global).unwrap();
    } // pass end
    println!("End")
}

fn fun_name(
    device_id: DeviceId,
    global: &Global,
    hub: &IdentityHub,
    layout: Option<PipelineLayoutId>,
    implicit_pipeline_ids: Option<wgpu_core::device::ImplicitPipelineIds>,
) -> (ShaderModuleId, ShaderModuleId, RenderPipelineId) {
    let (shader1, error) = global.device_create_shader_module::<api::Vulkan>(
        device_id,
        &ShaderModuleDescriptor {
            label: None,
            shader_bound_checks: ShaderBoundChecks::default(),
        },
        ShaderModuleSource::Wgsl(Borrowed(
            "@group(2) @binding(0) var<uniform> u1: vec4f;
        @group(3) @binding(0) var<uniform> u2: vec4f;
        @vertex fn main() -> @builtin(position) vec4f { return u1 + u2; }
        ",
        )),
        Some(hub.shader_modules.process(Vulkan)),
    );
    er(error);

    let (shader2, error) = global.device_create_shader_module::<api::Vulkan>(
        device_id,
        &ShaderModuleDescriptor {
            label: None,
            shader_bound_checks: ShaderBoundChecks::default(),
        },
        ShaderModuleSource::Wgsl(Borrowed("@fragment fn main() {}")),
        Some(hub.shader_modules.process(Vulkan)),
    );
    er(error);

    let (render_pipeline, error) = global.device_create_render_pipeline::<api::Vulkan>(
        device_id,
        &RenderPipelineDescriptor {
            label: None,
            layout,
            vertex: VertexState {
                stage: ProgrammableStageDescriptor {
                    module: shader1,
                    entry_point: Some(Cow::Borrowed("main")),
                    constants: Cow::Owned(HashMap::new()),
                    zero_initialize_workgroup_memory: false,
                },
                buffers: Borrowed(&[]),
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
                targets: Borrowed(&[Some(ColorTargetState {
                    format: TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: ColorWrites::empty(),
                })]),
                stage: ProgrammableStageDescriptor {
                    module: shader2,
                    entry_point: Some(Cow::Borrowed("main")),
                    constants: Cow::Owned(HashMap::new()),
                    zero_initialize_workgroup_memory: false,
                },
            }),
            multiview: None,
            cache: None,
        },
        Some(hub.render_pipelines.process(Vulkan)),
        implicit_pipeline_ids,
    );
    er(error);
    (shader1, shader2, render_pipeline)
}
