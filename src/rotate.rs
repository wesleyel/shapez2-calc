use crate::shape::{Shape, SingleLayer, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER};

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
        for i in 0..SHAPEZ2_LAYER {
            shape[i] = shape[i].rotate_once();
        }
        shape
    }

    fn rotate_once_reverse(&self) -> Shape {
        let mut shape = *self;
        for i in 0..SHAPEZ2_LAYER {
            shape[i] = shape[i].rotate_once_reverse();
        }
        shape
    }
}

impl Rotatable for SingleLayer {
    fn rotate_once(&self) -> SingleLayer {
        let mut layer = *self;
        let ori_layer = *self;
        for i in 0..SHAPEZ2_DEMENTION {
            layer.items[SHAPEZ2_DEMENTION - 1 - i] = ori_layer.items[i];
        }
        layer
    }

    fn rotate_once_reverse(&self) -> SingleLayer {
        let mut layer = *self;
        let ori_layer = *self;
        for i in 0..SHAPEZ2_DEMENTION {
            layer.items[i] = ori_layer.items[SHAPEZ2_DEMENTION - 1 - i];
        }
        layer
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
