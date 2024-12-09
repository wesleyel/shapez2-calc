use types::Shape;

mod types;

fn main() {
    let shape = Shape::try_from_string("WycwP-Rc:P-SyRbWy:RkSuWySy");
    if let Some(shape) = shape {
        println!("{}", shape.to_string());
        println!("{}", shape.to_shapez2_shape_viewer());
    } else {
        println!("Invalid shape string");
    }
}
