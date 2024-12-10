use shapez2_calc::shape::Shape;

fn main() {
    for _ in 0..10 {
        let shape = Shape::random();
        println!("{}", shape.to_minify_string());
        println!("{}", shape.to_shapez2_shape_viewer());
    }
}
