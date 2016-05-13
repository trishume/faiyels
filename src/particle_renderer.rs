use gfx;
use cgmath;
use cgmath::{Matrix4, Vector2, SquareMatrix};
use piston_window::Size;

const QUAD_VERTICES: [Vertex; 4] = [
    Vertex { position: [-0.5,  0.5] },
    Vertex { position: [-0.5, -0.5] },
    Vertex { position: [ 0.5, -0.5] },
    Vertex { position: [ 0.5,  0.5] },
];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];
const INITIAL_SCALE: f32 = 8.0;

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "a_Position",
    }

    // color format: 0xRRGGBBAA
    vertex Instance {
        translate: [f32; 2] = "a_Translate",
        color: u32 = "a_Color",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vertex: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        locals_cb: gfx::ConstantBuffer<Locals> = "b_VsLocals",
        out: gfx::RenderTarget<gfx::format::Srgba8> = "Target0",
    }
}

pub struct ParticleRenderer<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    locals: Locals,

    projection: Matrix4<f32>,
    px_per_unit: f32,
    translation: Vector2<f32>
}

impl<R: gfx::Resources> ParticleRenderer<R> {
    pub fn new<F: gfx::Factory<R>>(mut factory: &mut F,
        color: gfx::handle::RenderTargetView<R, gfx::format::Srgba8>, size: Size, data: &[Instance]) -> Self {
        use gfx::traits::FactoryExt;

        let instance_count = data.len() as u32;
        println!("{} instances", instance_count);
        // assert!(instance_count as usize <= MAX_INSTANCE_COUNT);

        let quad_instances = factory.create_buffer_const(data, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap();

        let (quad_vertices, mut slice) = factory.create_vertex_buffer_with_slice(&QUAD_VERTICES, &QUAD_INDICES[..]);
        slice.instances = Some((instance_count, 0));

        ParticleRenderer {
            pso: factory.create_pipeline_simple(
                include_bytes!("../shader/instancing_150.glslv"),
                include_bytes!("../shader/instancing_150.glslf"),
                pipe::new()
                ).unwrap(),
            data: pipe::Data {
                vertex: quad_vertices,
                instance: quad_instances,
                locals_cb: factory.create_constant_buffer(1),
                out: color,
            },
            locals: Locals {
                transform: Matrix4::identity().into(),
            },
            slice: slice,

            px_per_unit: INITIAL_SCALE,
            translation: Vector2::new(-((size.width as f32)/2.0/INITIAL_SCALE), (size.height as f32)/2.0/INITIAL_SCALE),
            projection: cgmath::ortho(-((size.width as f32)/2.0), (size.width as f32)/2.0, -((size.height as f32)/2.0), (size.height as f32)/2.0, -10.0, 10.0)
        }
    }

    fn compute_transform(&self) -> Matrix4<f32> {
        let scale = Matrix4::from_scale(self.px_per_unit);
        let translate = Matrix4::from_translation(self.translation.extend(0.0));
        (self.projection * scale * translate)
    }

    pub fn scroll_canvas(&mut self, x: f32, y: f32) {
        self.translation = self.translation + Vector2::new(x,-y)*(1.0/self.px_per_unit);
    }

    pub fn zoom(&mut self, factor: f32) {
        self.px_per_unit *= factor;
    }

    pub fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        self.locals.transform = self.compute_transform().into();
        encoder.update_constant_buffer(&self.data.locals_cb, &self.locals);
        encoder.clear(&self.data.out, [0.068, 0.076, 0.092, 1.0]);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
