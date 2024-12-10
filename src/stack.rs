use crate::shape::{Shape, SHAPEZ2_DEMENTION, SHAPEZ2_LAYER};

pub trait Stackable {
    fn stacked_with(&mut self, other_on_top: &Self);
    fn stack_with(&self, other_on_top: &Self) -> Self
    where
        Self: Sized + Copy,
    {
        let mut shape = *self;
        shape.stacked_with(other_on_top);
        shape
    }
    fn stacked(bot: &mut Self, top: &Self) -> Self
    where
        Self: Sized + Copy,
    {
        let shape = bot;
        shape.stacked_with(top);
        *shape
    }
    fn stack(bot: &Self, top: &Self) -> Self
    where
        Self: Sized + Copy,
    {
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
