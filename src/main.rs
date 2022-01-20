use lyon::math::point;
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
    StrokeVertex, VertexBuffers,
};

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// Let's use our own custom vertex type instead of the default one.
#[derive(Copy, Clone, Debug)]
struct MyVertex {
    position: [f32; 2],
}

fn main() {
    let mut builder = lyon::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.quadratic_bezier_to(point(60.0, 0.0), point(90.0, 90.0));
    builder.end(true);

    let path = builder.build();

    let document = stroke(&path);
    let document = document.add(fill(&path));

    svg::save("image.svg", &document).unwrap();
}

fn stroke(lyon_path: &lyon::path::Path) -> svg::Document {
    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    tessellator
        .tessellate_path(
            lyon_path,
            &StrokeOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| MyVertex {
                position: vertex.position().to_array(),
            }),
        )
        .unwrap();

    make_document(geometry, "blue")
}

fn fill(lyon_path: &lyon::path::Path) -> svg::Document {
    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    // Compute the tessellation.
    tessellator
        .tessellate_path(
            lyon_path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| MyVertex {
                position: vertex.position().to_array(),
            }),
        )
        .unwrap();

    make_document(geometry, "red")
}

fn make_document(geometry: VertexBuffers<MyVertex, u16>, fill_color: &str) -> svg::Document {
    let mut document = Document::new().set("viewBox", (0, 0, 90, 90));

    for triangle in 0..(geometry.indices.len() / 3) {
        let mut data = Data::new();

        for i in 0..3 {
            let vertex = geometry.vertices[geometry.indices[triangle * 3 + i] as usize];

            if i == 0 {
                data = data.move_to((vertex.position[0], vertex.position[1]));
            } else {
                data = data.line_to((vertex.position[0], vertex.position[1]));
            }
        }

        data = data.close();

        let path = Path::new()
            .set("fill", fill_color)
            .set("stroke", "black")
            .set("stroke-width", 0.1)
            .set("d", data);

        document = document.add(path);
    }

    document
}
