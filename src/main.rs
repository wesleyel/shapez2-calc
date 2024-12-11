use shapez2_calc::{
    cutting::Cuttable,
    rotate::Rotatable,
    shape::{EColor, EShape, Shape, SingleItem, SingleLayer, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER},
    stack::Stackable,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn needed_layers(items: Vec<SingleItem>) -> Vec<SingleLayer> {
    let mut layers = Vec::new();
    items.iter().for_each(|item| {
        let layer = SingleLayer::new_with_shape_color(item.shape, item.color);
        layers.push(layer);
    });
    layers
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    layer: SingleLayer,
    path: Vec<SingleLayer>,
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

fn heuristic(layer: &SingleLayer, goal: &SingleLayer) -> usize {
    let mut count = 0;
    for j in 0..SHAPEZ2_DEMENTION {
        let item_s = layer.items[j];
        let item_g = goal.items[j];
        if item_s.shape == item_g.shape {
            count += 1;
        }
        if item_s.color == item_g.color {
            count += 1;
        }
    }

    count
}

fn a_star(start_shapes: &[SingleLayer], goal: &SingleLayer) -> Option<Vec<SingleLayer>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    for layer in start_shapes {
        let initial_state = State {
            layer: layer.clone(),
            path: vec![layer.clone()],
            cost: 0,
            heuristic: heuristic(layer, goal),
        };
        open_set.push(initial_state);
    }

    let mut cnt = 0;

    while let Some(current) = open_set.pop() {
        if current.layer == *goal {
            return Some(current.path);
        }

        if closed_set.contains(&current.layer) {
            continue;
        }
        closed_set.insert(current.layer.clone());

        let new_states = vec![
            current.layer.rotate_once(),
            current.layer.rotate_once_reverse(),
            current.layer.rotate_180(),
            current.layer.cutting()[0],
            current.layer.cutting()[1],
        ];

        for new_state in new_states {
            if !closed_set.contains(&new_state) {
                let mut new_path = current.path.clone();
                new_path.push(new_state.clone());
                let new_cost = current.cost + 1;
                let new_heuristic = heuristic(&new_state, goal);
                let next_state = State {
                    layer: new_state,
                    path: new_path,
                    cost: new_cost,
                    heuristic: new_heuristic,
                };
                open_set.push(next_state);
            }
        }

        for shape in start_shapes {
            let [swap_a, swap_b] = current.layer.swap_with(shape);
            if !closed_set.contains(&swap_a) {
                let mut new_path = current.path.clone();
                new_path.push(swap_a.clone());
                let new_cost = current.cost + 1;
                let new_heuristic = heuristic(&swap_a, goal);
                let next_state = State {
                    layer: swap_a,
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
                    layer: swap_b,
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
    let goal_shape = Shape::try_from_string("Sb----Wm:--CcP-P-:--P-----:--Sc--Sg").unwrap();
    let needed_shapes = needed_layers(goal_shape.unique_flat_items());
    println!(
        "Goal shape: {}\n{}",
        goal_shape,
        goal_shape.to_shapez2_shape_viewer()
    );
    needed_shapes.iter().for_each(|shape| {
        println!("Needed shape: {}", shape,);
    });

    let mut final_path = Vec::new();
    goal_shape.into_iter().for_each(|layer| {
        if let Some(path) = a_star(&needed_shapes, &layer) {
            final_path.extend(path);
        } else {
            println!("No path found for layer: {:?}", layer);
            return;
        }
    });

    println!("Final path: {:?}", final_path);
}
