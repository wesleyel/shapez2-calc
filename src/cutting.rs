use crate::{
    rotate::Rotatable,
    shape::{Shape, SingleItem, SingleLayer, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER},
};

pub trait Cuttable: Sized + Copy + Rotatable {
    fn half_destroyed(&mut self);
    fn half_destroy(&self) -> Self {
        let mut shape = *self;
        shape.half_destroyed();
        shape
    }

    fn cutting(&self) -> [Self; 2] {
        let left_shape = self.rotate_180().half_destroy().rotate_180();
        let right_shape = self.half_destroy();
        [left_shape, right_shape]
    }

    fn swapd(a: &mut Self, b: &mut Self);
    fn swapd_with(&mut self, b: &mut Self) {
        Self::swapd(self, b);
    }
    fn swap(a: &Self, b: &Self) -> [Self; 2] {
        let mut a = *a;
        let mut b = *b;
        Self::swapd(&mut a, &mut b);
        [a, b]
    }
    fn swap_with(&self, other: &Self) -> [Self; 2] {
        Self::swap(self, other)
    }
}

impl Cuttable for Shape {
    fn half_destroyed(&mut self) {
        for i in 0..SHAPEZ2_LAYER {
            self[i].half_destroyed();
        }
    }

    fn swapd(a: &mut Self, b: &mut Self) {
        for i in 0..SHAPEZ2_LAYER {
            SingleLayer::swapd(&mut a[i], &mut b[i]);
        }
    }
}

impl Cuttable for SingleLayer {
    fn half_destroyed(&mut self) {
        for j in 0..SHAPEZ2_DEMENTION {
            if j * 2 / SHAPEZ2_DEMENTION > 0 {
                self.items[j] = SingleItem::default();
            }
        }
    }

    fn swapd(a: &mut Self, b: &mut Self) {
        let ori_a = *a;
        let ori_b = *b;
        for j in 0..SHAPEZ2_DEMENTION {
            if j * 2 / SHAPEZ2_DEMENTION > 0 {
                a.items[j] = ori_b.items[j];
                b.items[j] = ori_a.items[j];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_half_destroy() {
        let shape = Shape::random();
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

    #[test]
    fn test_cutting() {
        let shape = Shape::random();
        let [left_shape, right_shape] = shape.cutting();
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                if j * 2 / SHAPEZ2_DEMENTION > 0 {
                    assert_eq!(left_shape.items[i][j], shape.items[i][j]);
                    assert_eq!(right_shape.items[i][j], SingleItem::new());
                } else {
                    assert_eq!(left_shape.items[i][j], SingleItem::new());
                    assert_eq!(right_shape.items[i][j], shape.items[i][j]);
                }
            }
        }
    }

    #[test]
    fn test_swap() {
        let shape_a = Shape::random();
        let shape_b = Shape::random();
        let [new_shape_a, new_shape_b] = Shape::swap(&shape_a, &shape_b);
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                if j * 2 / SHAPEZ2_DEMENTION > 0 {
                    assert_eq!(new_shape_a.items[i][j], shape_b.items[i][j]);
                    assert_eq!(new_shape_b.items[i][j], shape_a.items[i][j]);
                } else {
                    assert_eq!(new_shape_a.items[i][j], shape_a.items[i][j]);
                    assert_eq!(new_shape_b.items[i][j], shape_b.items[i][j]);
                }
            }
        }
    }

    #[test]
    fn test_swap_with() {
        let mut shape_a = Shape::random();
        let mut shape_b = Shape::random();
        let [new_shape_a, new_shape_b] = shape_a.swap_with(&shape_b);
        shape_a.swapd_with(&mut shape_b);

        assert_eq!(new_shape_a, shape_a);
        assert_eq!(new_shape_b, shape_b);
    }
}
