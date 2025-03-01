mod player;
pub use player::*;

use bevy::prelude::*;

pub struct EntsRegistry;
impl Plugin for EntsRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerRegistry);
    }
}
