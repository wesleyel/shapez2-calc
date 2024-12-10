use crate::shape::{Shape, SingleItem, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER};

pub trait HalfDestroyable {
    fn half_destroyed(&mut self);
    fn half_destroy(&self) -> Self;
}

impl HalfDestroyable for Shape {
    fn half_destroyed(&mut self) {
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                if j * 2 / SHAPEZ2_DEMENTION > 0 {
                    self.items[i][j] = SingleItem::new();
                }
            }
        }
    }

    fn half_destroy(&self) -> Self {
        let mut shape = self.clone();
        shape.half_destroyed();
        shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_destroy() {
        let shape = Shape::new_random();
        let new_shape = shape.half_destroy();
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                if j * 2 / SHAPEZ2_DEMENTION > 0 {
                    assert_eq!(new_shape.items[i][j], SingleItem::new());
                } else {
                    assert_eq!(new_shape.items[i][j], shape.items[i][j]);
                }
            }
        }
    }
}
