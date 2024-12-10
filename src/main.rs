use shapez2_calc::{
    cutting::{Cuttable, Swapable},
    rotate::Rotatable,
    shape::{EColor, EShape, Shape, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER},
    stack::Stackable,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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

#[derive(Clone, Eq, PartialEq)]
struct State {
    shape: Shape,
    path: Vec<Shape>,
    cost: usize,
    heuristic: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(shape: &Shape, goal: &Shape) -> usize {
    let mut count = 0;
    for i in 0..SHAPEZ2_LAYER {
        for j in 0..SHAPEZ2_DEMENTION {
            let item_s = shape.items[i][j];
            let item_g = goal.items[i][j];
            if item_s.shape == item_g.shape {
                count += 1;
            }
            if item_s.color == item_g.color {
                count += 1;
            }
        }
    }
    count
}

fn a_star(start_shapes: &[Shape], goal: &Shape) -> Option<Vec<Shape>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    for shape in start_shapes {
        let initial_state = State {
            shape: shape.clone(),
            path: vec![shape.clone()],
            cost: 0,
            heuristic: heuristic(shape, goal),
        };
        open_set.push(initial_state);
    }

    let mut cnt = 0;

    while let Some(current) = open_set.pop() {
        if current.shape == *goal {
            return Some(current.path);
        }

        if closed_set.contains(&current.shape) {
            continue;
        }
        closed_set.insert(current.shape.clone());

        let new_states = vec![
            current.shape.rotate_once(),
            current.shape.rotate_once_reverse(),
            current.shape.rotate_180(),
            current.shape.cutting()[0],
            current.shape.cutting()[1],
        ];

        for new_state in new_states {
            if !closed_set.contains(&new_state) {
                let mut new_path = current.path.clone();
                new_path.push(new_state.clone());
                let new_cost = current.cost + 1; // 假设每个操作的代价为1
                let new_heuristic = heuristic(&new_state, goal);
                let next_state = State {
                    shape: new_state,
                    path: new_path,
                    cost: new_cost,
                    heuristic: new_heuristic,
                };
                open_set.push(next_state);
            }
        }

        for shape in start_shapes {
            let [swap_a, swap_b] = current.shape.swap_with(shape);
            if !closed_set.contains(&swap_a) {
                let mut new_path = current.path.clone();
                new_path.push(swap_a.clone());
                let new_cost = current.cost + 1;
                let new_heuristic = heuristic(&swap_a, goal);
                let next_state = State {
                    shape: swap_a,
                    path: new_path,
                    cost: new_cost,
                    heuristic: new_heuristic,
                };
                open_set.push(next_state);
            }
            if !closed_set.contains(&swap_b) {
                let mut new_path = current.path.clone();
                new_path.push(swap_b.clone());
                let new_cost = current.cost + 1;
                let new_heuristic = heuristic(&swap_b, goal);
                let next_state = State {
                    shape: swap_b,
                    path: new_path,
                    cost: new_cost,
                    heuristic: new_heuristic,
                };
                open_set.push(next_state);
            }

            let stacked = current.shape.stack_with(shape);
            if !closed_set.contains(&stacked) {
                let mut new_path = current.path.clone();
                new_path.push(stacked.clone());
                let new_cost = current.cost + 1;
                let new_heuristic = heuristic(&stacked, goal);
                let next_state = State {
                    shape: stacked,
                    path: new_path,
                    cost: new_cost,
                    heuristic: new_heuristic,
                };
                open_set.push(next_state);
            }
        }

        cnt += 1;
        if cnt % 1000 == 0 {
            eprintln!("cnt: {}", cnt);
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

    if let Some(path) = a_star(&avaliable_shapes, &goal_shape) {
        println!("find path: {:?}", path);
    } else {
        println!("No path found");
    }
}
