use crate::shape::{Shape, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER};

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
            for j in 0..SHAPEZ2_DEMENTION {
                self.items[ori_layer_height + i][j] = other_on_top.items[i][j];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_stack() {
        let bot = Shape::random_with_height(2);
        let top = Shape::random_with_height(2);
        let new_shape = Shape::stack(&bot, &top);
        eprintln!("{}", bot.to_shapez2_shape_viewer());
        eprintln!("{}", top.to_shapez2_shape_viewer());
        eprintln!("{}", new_shape.to_shapez2_shape_viewer());

        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                if i < SHAPEZ2_LAYER - 1 {
                    assert_eq!(new_shape.items[i][j], bot.items[i][j]);
                } else {
                    assert_eq!(new_shape.items[i][j], top.items[i - SHAPEZ2_LAYER + 1][j]);
                }
            }
        }
    }
}
