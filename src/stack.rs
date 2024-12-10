use crate::shape::{Shape, SHAPEZ2_LAYER};

pub trait Stackable: Sized + Copy {
    fn stacked_with(&mut self, other_on_top: &Self);
    fn stack_with(&self, other_on_top: &Self) -> Self {
        let mut shape = *self;
        shape.stacked_with(other_on_top);
        shape
    }
    fn stacked(bot: &mut Self, top: &Self) -> Self {
        let shape = bot;
        shape.stacked_with(top);
        *shape
    }
    fn stack(bot: &Self, top: &Self) -> Self {
        bot.stack_with(top)
    }
}

impl Stackable for Shape {
    fn stacked_with(&mut self, other_on_top: &Self) {
        let ori_layer_height = self.layer_height();
        let layer_needed = SHAPEZ2_LAYER - ori_layer_height;
        for i in 0..layer_needed {
            self[ori_layer_height + i] = other_on_top[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::SHAPEZ2_DEMENTION;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_stack() {
        let layer_height = 2;
        let bot = Shape::random_with_height(layer_height);
        let top = Shape::random_with_height(SHAPEZ2_DEMENTION - layer_height);
        let new_shape = Shape::stack(&bot, &top);
        for i in 0..SHAPEZ2_LAYER {
            if i < layer_height {
                assert_eq!(new_shape.items[i], bot.items[i]);
            } else {
                assert_eq!(new_shape.items[i], top.items[i - layer_height]);
            }
        }
    }
}
