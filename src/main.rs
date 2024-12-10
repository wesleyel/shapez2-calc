use types::Shape;

mod types;

fn main() {
    for _ in 0..10 {
        let shape = Shape::new_random();
        println!("{}", shape.to_string());
        println!("{}", shape.to_shapez2_shape_viewer());
    }
}
