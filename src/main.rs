use shapez2_calc::shape::Shape;

fn main() {
    for _ in 0..10 {
        let shape = Shape::new_random();
        println!("{}", shape.to_string());
        println!("{}", shape.to_shapez2_shape_viewer());
    }
}
