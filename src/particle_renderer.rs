use gfx;

const QUAD_VERTICES: [Vertex; 4] = [
    Vertex { position: [-0.5,  0.5] },
    Vertex { position: [-0.5, -0.5] },
    Vertex { position: [ 0.5, -0.5] },
    Vertex { position: [ 0.5,  0.5] },
];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

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
        scale: f32 = "u_Scale",
    }

    pipeline pipe {
        vertex: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        scale: gfx::Global<f32> = "u_Scale",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<gfx::format::Srgba8> = "Target0",
    }
}

fn fill_instances(attributes: &mut [Instance], instances_per_length: u32, size: f32) {
    let gap = 0.4 / (instances_per_length + 1) as f32;
    println!("gap: {}", gap);

    let begin = -1. + gap + (size /2.);
    let mut translate = [begin, begin];

    let length = instances_per_length as usize;
    for x in 0..length {
        for y in 0..length {
            let i = x*length + y;
            attributes[i] = Instance {
                translate: translate,
                color: (((x*5) << 8) | ((y*5) << 16) | 0xFF) as u32
            };
            translate[1] += size + gap;
        }
        translate[1] = begin;
        translate[0] += size + gap;
    }
 }

const MAX_INSTANCE_COUNT: usize = 2048;

pub struct ParticleRenderer<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
}

impl<R: gfx::Resources> ParticleRenderer<R> {
    pub fn new<F: gfx::Factory<R>>(mut factory: &mut F, color: gfx::handle::RenderTargetView<R, gfx::format::Srgba8>) -> Self {
        use gfx::traits::FactoryExt;

        let instances_per_length: u32 = 100;
        println!("{} instances per length", instances_per_length);
        let instance_count = instances_per_length * instances_per_length;
        println!("{} instances", instance_count);
        // assert!(instance_count as usize <= MAX_INSTANCE_COUNT);
         let size = 1.6 / instances_per_length as f32;
        println!("size: {}", size);

        let use_mapping = false;
        let quad_instances = if use_mapping {
            let buf = factory.create_buffer_dynamic(MAX_INSTANCE_COUNT, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap();
            let mut attributes = factory.map_buffer_writable(&buf);
            fill_instances(attributes.to_mut_slice(), instances_per_length, size);
            buf
        }else {
            let mut attributes = (0..instance_count).map(|_| Instance {
                translate: [0.0, 0.0],
                color: 0,
            }).collect::<Vec<_>>();
            fill_instances(&mut attributes, instances_per_length, size);
            factory.create_buffer_const(&attributes, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap()
        };

        let (quad_vertices, mut slice) = factory.create_vertex_buffer_with_slice(&QUAD_VERTICES, &QUAD_INDICES[..]);
        slice.instances = Some((instance_count, 0));
        let locals = Locals { scale: size };

        ParticleRenderer {
            pso: factory.create_pipeline_simple(
                include_bytes!("../shader/instancing_150.glslv"),
                include_bytes!("../shader/instancing_150.glslf"),
                pipe::new()
                ).unwrap(),
            data: pipe::Data {
                vertex: quad_vertices,
                instance: quad_instances,
                scale: size,
                locals: factory.create_buffer_const(&[locals],
                    gfx::BufferRole::Uniform, gfx::Bind::empty()
                    ).unwrap(),
                out: color,
            },
            slice: slice,
        }
    }

    pub fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        encoder.clear(&self.data.out, [0.1, 0.2, 0.3, 1.0]);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
