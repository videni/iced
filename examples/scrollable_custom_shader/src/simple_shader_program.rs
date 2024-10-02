use iced::{mouse, Point, Transformation};
use iced::{Rectangle, Size};
use crate::wgpu;
use iced::widget::shader::{self, Viewport};
use iced::widget::shader::wgpu::util::DeviceExt;

pub struct SimpleShaderProgram
{
    pub image: String,
}

#[derive(Default)]
pub struct State {
}


impl<Message> shader::Program<Message> for SimpleShaderProgram
{
    type State = State;
    type Primitive = SimpleShaderProgramPrimitive;

    fn draw(&self, _state: &Self::State, _cursor: mouse::Cursor, bounds: Rectangle) -> Self::Primitive {
        SimpleShaderProgramPrimitive {
            image: self.image.clone(),
        }
    }
}

#[derive(Debug)]
pub struct SimpleShaderProgramPrimitive
{
    image: String,
}

static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn counter_increment() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}


impl shader::Primitive for SimpleShaderProgramPrimitive
{
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut shader::Storage,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, queue, format));
        }

        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        let position = (*bounds) * (viewport.scale_factor() as f32) * viewport.projection();

        // Upload data to GPU
        pipeline.update(device, queue, self.image.as_str(), position);
    }

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        // At this point our pipeline should always be initialized
        let pipeline = storage.get::<Pipeline>().unwrap();

        // Render primitive
        pipeline.render(encoder, target, clip_bounds);
    }
}


struct Pipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: Option<wgpu::BindGroup>,
    render_pipeline: wgpu::RenderPipeline,
    index_buffer: Option<wgpu::Buffer>,
    vertex_buffer: Option<wgpu::Buffer>
}

impl Pipeline {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // Corresponds to `@location(0)` in the vertex shader
                    format: wgpu::VertexFormat::Float32x2, // 2D position
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1, // Corresponds to `@location(1)` in the vertex shader
                    format: wgpu::VertexFormat::Float32x2, // 2D texture coordinates
                },
            ],
        };

        // Create the bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create the render pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("A Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout], // Define your vertex buffer here
                compilation_options:  wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options:  wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //     label: Some("simple shader program encoder"),
        // });

        Self {
            render_pipeline,
            bind_group_layout,
            index_buffer: None,
            vertex_buffer: None,
            bind_group: None,
        }
    }

    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, image: &str, position: Rectangle) {
        use image::GenericImageView;

        // Load the image data in to texture
        let image_data = image::open(format!(
            "{}/resources/{}.png",
            env!("CARGO_MANIFEST_DIR"),
            image
        )).unwrap();

        let (width, height) = image_data.dimensions();
        let rgba_image = image_data.to_rgba8();
        let raw_data = rgba_image.into_raw();

        // Create the wgpu texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Copy the pixel data into the texture
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &raw_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            }
        );
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create the bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Bind Group"),
        });

        let top_left = [position.x, position.y];
        let top_right = [position.x + position.width, position.y];
        let bottom_left = [position.x, position.y + position.height];
        let bottom_right = [position.x + position.width, position.y + position.height];

         // Vertex data for a quad (4 vertices)
         let vertices: [Vertex; 4] = [
            Vertex::new(bottom_left , [0.0, 1.0]), // bottom-left
            Vertex::new(bottom_right, [1.0, 1.0]),  // bottom-right
            Vertex::new(top_right, [1.0, 0.0]),   // top-right
            Vertex::new(top_left, [0.0, 0.0]),  // top-left
        ];

        // Create a vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index data (6 indices for 2 triangles forming a quad)
        let indices: [u16; 6] = [
            0, 1, 2,  // First triangle (bottom-left, bottom-right, top-right)
            0, 2, 3,  // Second triangle (bottom-left, top-right, top-left)
        ];

        // Create an index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.bind_group = Some(bind_group);
        self.index_buffer = Some(index_buffer);
        self.vertex_buffer = Some(vertex_buffer);
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, iced_target: &wgpu::TextureView, clip_bounds: &Rectangle<u32>) {
        // Begin a new render pass
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: iced_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("Render Pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_scissor_rect(clip_bounds.x, clip_bounds.y, clip_bounds.width, clip_bounds.height);

        // Set the vertex buffer and index buffer
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);

        // Set the pipeline and bind group
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);

        // Draw indexed: 6 indices (two triangles), 1 instance
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        Self { position, tex_coords }
    }
}


#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    transform: [f32; 16],
    /// Uniform values must be 256-aligned;
    /// see: [`wgpu::Limits`] `min_uniform_buffer_offset_alignment`.
    _padding: [f32; 48],
}

impl Uniforms {
    pub fn new(transform: Transformation) -> Self {
        Self {
            transform: transform.into(),
            _padding: [0.0; 48],
        }
    }

    pub fn entry() -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: wgpu::BufferSize::new(
                    std::mem::size_of::<Self>() as u64,
                ),
            },
            count: None,
        }
    }

    pub fn min_size() -> Option<wgpu::BufferSize> {
        wgpu::BufferSize::new(std::mem::size_of::<Self>() as u64)
    }
}