use lyon::path::Path;
use lyon::math::point;
use lyon::tessellation::{VertexBuffers, FillTessellator, FillVertex, FillOptions};
use lyon::tessellation::geometry_builder::simple_builder;
use euc::{Pipeline, buffer::Buffer2d, rasterizer::{Triangles, BackfaceCullingDisabled}};
use minifb::{Window, WindowOptions, Key, KeyRepeat};
use vek::{Vec3, Vec4, Mat4};

struct Shader<'a> {
    /// The vertices to render
    vertices: &'a [FillVertex],
    /// The Model-View-Projection matrix
    mvp: Mat4<f32>,
}

impl<'a> Pipeline for Shader<'a> {
    type Vertex = u16;
    /// No need to return the normal because backface culling isn't used
    type VsOut = ();
    type Pixel = u32;

    #[inline(always)]
    fn vert(&self, &index: &Self::Vertex) -> ([f32; 3], Self::VsOut) {
        let vert = self.vertices[index as usize];
        let pos = Vec4::from_point(vert.position.to_3d().to_array());
        let pos_cam = Vec3::from(self.mvp * pos).into_array();
        (pos_cam, ())
    }

    #[inline(always)]
    fn frag(&self, _: &Self::VsOut) -> Self::Pixel {
        let bytes = [255, 0, 0, 255]; // Red

        (bytes[2] as u32) << 0 |
        (bytes[1] as u32) << 8 |
        (bytes[0] as u32) << 16 |
        (bytes[3] as u32) << 24
    }
}

fn main() {
    // Create a simple path.
    let mut path_builder = Path::builder();
    path_builder.move_to(point(0.0, 0.0));
    path_builder.line_to(point(1.0, 2.0));
    path_builder.line_to(point(2.0, 0.0));
    path_builder.line_to(point(1.0, 1.0));
    path_builder.close();
    let path = path_builder.build();

    // Create the destination vertex and index buffers.
    let mut buffers: VertexBuffers<FillVertex, u16> = VertexBuffers::new();

    {
        let mut vertex_builder = simple_builder(&mut buffers);

        // Create the tessellator.
        let mut tessellator = FillTessellator::new();

        // Compute the tessellation.
        let result = tessellator.tessellate_path(
            path.iter(),
            &FillOptions::default().with_normals(false),
            &mut vertex_builder
        );
        assert!(result.is_ok());
    }

    println!("The generated vertices are: {:?}.", &buffers.vertices[..]);
    println!("The generated indices are: {:?}.", &buffers.indices[..]);

    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    let mut color = Buffer2d::new([WIDTH, HEIGHT], 0);
    let mut depth = Buffer2d::new([WIDTH, HEIGHT], 1.0);

    // Scaling needs to be set such that all coordinates are between -1.0 and 1.0
    let mvp = Mat4::scaling_3d(0.5);
    let shader = Shader {
        vertices: &buffers.vertices[..],
        mvp,
    };
    // Need to disable backface culling because there are no backfaces in 2D graphics
    // and because the tessellator always seem to return the wrong vertex order
    shader.draw::<Triangles<_, BackfaceCullingDisabled>, _>(
        &buffers.indices[..],
        &mut color,
        &mut depth,
    );

    let mut win = Window::new("Kale", WIDTH, HEIGHT,
        WindowOptions::default()).unwrap();
    while win.is_open() && !win.is_key_pressed(Key::Escape, KeyRepeat::No) {
        win.update_with_buffer(color.as_ref()).unwrap();
    }
}
