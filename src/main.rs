use std::collections::{HashSet, VecDeque};

use shapez2_calc::{
    cutting::{Cuttable, Swapable},
    rotate::Rotatable,
    shape::{EColor, EShape, Shape},
    stack::Stackable,
};

pub fn basic_shapes() -> [Shape; 36] {
    let mut shapes = [Shape::default(); 36];
    let ecolors = [
        EColor::Red,
        EColor::Green,
        EColor::Blue,
        EColor::Yellow,
        EColor::Magenta,
        EColor::Cyan,
        EColor::White,
        EColor::Black,
        EColor::Uncolored,
    ];
    let eshapes = [
        EShape::Circle,
        EShape::Rectangle,
        EShape::Windmill,
        EShape::Star,
    ];
    let mut index = 0;
    for &color in &ecolors {
        for &shape in &eshapes {
            shapes[index] = Shape::new_simple(shape, color);
            index += 1;
        }
    }
    shapes
}

fn bfs(start_shapes: &[Shape], goal: &Shape) -> Option<Vec<Shape>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    for shape in start_shapes {
        queue.push_back((shape.clone(), vec![shape.clone()]));
        visited.insert(shape.clone());
    }

    while let Some((current, path)) = queue.pop_front() {
        if current == *goal {
            return Some(path);
        }

        let new_states = vec![
            current.rotate_once(),
            current.rotate_once_reverse(),
            current.rotate_180(),
            current.cutting()[0],
            current.cutting()[1],
        ];

        for new_state in new_states {
            if !visited.contains(&new_state) {
                let mut new_path = path.clone();
                new_path.push(new_state.clone());
                queue.push_back((new_state, new_path));
                visited.insert(new_state);
            }
        }

        for shape in start_shapes {
            let [swap_a, swap_b] = current.swap_with(shape);
            if !visited.contains(&swap_a) {
                let mut new_path = path.clone();
                new_path.push(swap_a.clone());
                queue.push_back((swap_a, new_path));
                visited.insert(swap_a);
            }
            if !visited.contains(&swap_b) {
                let mut new_path = path.clone();
                new_path.push(swap_b.clone());
                queue.push_back((swap_b, new_path));
                visited.insert(swap_b);
            }

            let stacked = current.stack_with(shape);
            if !visited.contains(&stacked) {
                let mut new_path = path.clone();
                new_path.push(stacked.clone());
                queue.push_back((stacked, new_path));
                visited.insert(stacked);
            }
        }

        if visited.len() % 1000 == 0 {
            eprintln!("Visited: {}", visited.len());
        }
    }

    None
}

fn main() {
    let avaliable_shapes = basic_shapes();
    let goal_shape = Shape::try_from_string("Sb----Wm:--CcP-P-:--P-----:--Sc--Sg").unwrap();
    eprintln!(
        "Goal shape: {}\n{}",
        goal_shape,
        goal_shape.to_shapez2_shape_viewer()
    );
    avaliable_shapes.iter().for_each(|shape| {
        eprintln!("Avaliable shape: {}", shape,);
    });

    if let Some(path) = bfs(&avaliable_shapes, &goal_shape) {
        println!("find path: {:?}", path);
    } else {
        println!("No path found");
    }
}
