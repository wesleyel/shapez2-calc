use crate::shape::{Shape, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateDirection {
    Clockwise,
    CounterClockwise,
}

pub trait Rotatable: Sized + Copy {
    fn rotate_once(&self) -> Self;
    fn rotate_once_reverse(&self) -> Self;
    fn rotate_180(&self) -> Self {
        self.rotate(RotateDirection::Clockwise, SHAPEZ2_DEMENTION / 2)
    }
    fn rotated(&mut self, direction: RotateDirection, times: usize) {
        match direction {
            RotateDirection::Clockwise => {
                for _ in 0..times {
                    *self = self.rotate_once();
                }
            }
            RotateDirection::CounterClockwise => {
                for _ in 0..times {
                    *self = self.rotate_once_reverse();
                }
            }
        }
    }
    fn rotate(&self, direction: RotateDirection, times: usize) -> Self {
        let mut shape = *self;
        shape.rotated(direction, times);
        shape
    }
}

impl Rotatable for Shape {
    fn rotate_once(&self) -> Shape {
        let mut shape = *self;
        let mut new_items = Shape::new().items;
        for i in 0..SHAPEZ2_LAYER {
            for (j, item) in new_items.iter_mut().enumerate().take(SHAPEZ2_DEMENTION) {
                item[SHAPEZ2_DEMENTION - 1 - i] = self.items[i][j];
            }
        }
        shape.items = new_items;
        shape
    }

    fn rotate_once_reverse(&self) -> Shape {
        let mut shape = *self;
        let mut new_items = Shape::new().items;
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                new_items[SHAPEZ2_DEMENTION - 1 - j][i] = self.items[i][j];
            }
        }
        shape.items = new_items;
        shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rotate_once() {
        let shape = Shape::random();
        let new_shape = shape.rotate_once();
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                assert_eq!(
                    new_shape.items[i][SHAPEZ2_DEMENTION - 1 - j],
                    shape.items[j][i]
                );
            }
        }
    }
}
