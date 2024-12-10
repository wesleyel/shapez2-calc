use crate::shape::{Shape, SingleItem, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER};

pub enum RotateAngle {
    Clockwise90,
    Clockwise180,
    Clockwise270,
    CounterClockwise90,
    CounterClockwise180,
    CounterClockwise270,
}

trait Rotatable {
    fn rotated(&mut self, angle: RotateAngle);
    fn rotate(&self, angle: RotateAngle) -> Self;
    fn rotate_clockwise_90(&self) -> Self;
    fn rotate_counter_clockwise_90(&self) -> Self;
}

impl Rotatable for Shape {
    fn rotate_clockwise_90(&self) -> Shape {
        let mut shape = self.clone();
        let mut new_items = [[SingleItem::new(); SHAPEZ2_DEMENTION]; SHAPEZ2_LAYER];
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                new_items[j][SHAPEZ2_DEMENTION - 1 - i] = self.items[i][j];
            }
        }
        shape.items = new_items;
        shape
    }

    fn rotate_counter_clockwise_90(&self) -> Shape {
        let mut shape = self.clone();
        let mut new_items = [[SingleItem::new(); SHAPEZ2_DEMENTION]; SHAPEZ2_LAYER];
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                new_items[SHAPEZ2_DEMENTION - 1 - j][i] = self.items[i][j];
            }
        }
        shape.items = new_items;
        shape
    }

    fn rotate(&self, angle: RotateAngle) -> Self {
        match angle {
            RotateAngle::Clockwise90 => self.rotate_clockwise_90(),
            RotateAngle::Clockwise180 => self.rotate_clockwise_90().rotate_clockwise_90(),
            RotateAngle::Clockwise270 => self.rotate_counter_clockwise_90(),
            RotateAngle::CounterClockwise90 => self.rotate_counter_clockwise_90(),
            RotateAngle::CounterClockwise180 => self
                .rotate_counter_clockwise_90()
                .rotate_counter_clockwise_90(),
            RotateAngle::CounterClockwise270 => self.rotate_clockwise_90(),
        }
    }

    fn rotated(&mut self, angle: RotateAngle) {
        *self = self.rotate(angle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rotate_clockwise_90() {
        let mut shape = Shape::new_random();
        let shape_rotated = shape.rotate_clockwise_90();
        shape.rotated(RotateAngle::Clockwise90);
        assert_eq!(shape, shape_rotated);
    }

    #[test]
    fn test_rotate_counter_clockwise_90() {
        let mut shape = Shape::new_random();
        let shape_rotated = shape.rotate_counter_clockwise_90();
        shape.rotated(RotateAngle::CounterClockwise90);
        assert_eq!(shape, shape_rotated);
    }

    #[test]
    fn test_rotate_180() {
        let mut shape = Shape::new_random();
        let shape_rotated = shape.rotate(RotateAngle::Clockwise180);
        shape.rotated(RotateAngle::Clockwise180);
        assert_eq!(shape, shape_rotated);
    }

    #[test]
    fn test_rotate_270() {
        let mut shape = Shape::new_random();
        let shape_rotated = shape.rotate(RotateAngle::Clockwise270);
        shape.rotated(RotateAngle::Clockwise270);
        assert_eq!(shape, shape_rotated);
    }
}
