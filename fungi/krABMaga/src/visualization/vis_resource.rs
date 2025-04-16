use crate::model::state::FungiState;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::{bevy::prelude::Image, visualization::fields::number_grid_2d::BatchRender};

impl BatchRender<FungiState> for DenseNumberGrid2D<f32> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.get_value(loc) {
            Some(_val) => {
                [34u8, 139u8, 34u8, 255u8]
            }
            None => [0u8, 255u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &FungiState) -> Image {
        state.resource_grid.texture()
    }
}
