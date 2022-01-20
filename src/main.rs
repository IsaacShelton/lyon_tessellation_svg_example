#[macro_use]
extern crate enum_display_derive;

use lyon::path::{
    builder::{BorderRadii, PathBuilder},
    math::{point, Rect, Size},
    Polygon, Winding,
};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
    StrokeVertex, VertexBuffers,
};

use std::fmt::Display;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// Let's use our own custom vertex type instead of the default one.
#[derive(Copy, Clone, Debug)]
struct MyVertex {
    position: [f32; 2],
}

#[derive(Display)]
enum Choice {
    Bezier,
    Circle,
    RoundRect,
    Polygon,
}

fn main() {
    let choices = [
        Choice::Bezier,
        Choice::Circle,
        Choice::RoundRect,
        Choice::Polygon,
    ];

    for choice in choices {
        make_example(choice);
    }
}

fn make_example(choice: Choice) {
    let mut builder = lyon::path::Path::builder();

    // Build abstract geometry
    match choice {
        Choice::Bezier => {
            builder.begin(point(0.0, 0.0));
            builder.quadratic_bezier_to(point(60.0, 0.0), point(90.0, 90.0));
            builder.end(true);
        }
        Choice::Circle => {
            builder.add_circle(point(0.0, 0.0), 90.0, Winding::Positive);
        }
        Choice::RoundRect => {
            builder.add_rounded_rectangle(
                &Rect::new(point(5.0, 5.0), Size::new(80.0, 80.0)),
                &BorderRadii::new(5.0),
                Winding::Positive,
            );
        }
        Choice::Polygon => {
            let polygon = Polygon {
                points: &[point(45.0, 10.0), point(80.0, 80.0), point(10.0, 80.0)],
                closed: true,
            };

            builder.add_polygon(polygon);
        }
    }

    // Create svg representation
    let path = builder.build();
    let document = stroke(&path);
    let document = document.add(fill(&path));

    // Create result directory
    let result_directory = "svg_output";
    std::fs::create_dir_all(result_directory).expect("Failed to create output directory");

    // Save svg file
    let filename = format!("{}/{}.svg", result_directory, choice);
    svg::save(&filename, &document).unwrap();
}

fn stroke(lyon_path: &lyon::path::Path) -> svg::Document {
    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    // Create tessellated geometry for stroke
    tessellator
        .tessellate_path(
            lyon_path,
            &StrokeOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| MyVertex {
                position: vertex.position().to_array(),
            }),
        )
        .unwrap();

    // Create svg representation for stroke geometry
    make_document(geometry, "blue")
}

fn fill(lyon_path: &lyon::path::Path) -> svg::Document {
    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    // Create tessellated geometry for fill
    tessellator
        .tessellate_path(
            lyon_path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| MyVertex {
                position: vertex.position().to_array(),
            }),
        )
        .unwrap();

    // Create svg representation for fill geometry
    make_document(geometry, "red")
}

fn make_document(geometry: VertexBuffers<MyVertex, u16>, fill_color: &str) -> svg::Document {
    // Create empty svg document with view window 90x90
    let mut document = Document::new().set("viewBox", (0, 0, 90, 90));

    // Add each triangle
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
