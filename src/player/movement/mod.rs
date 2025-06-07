mod coyote;
pub mod movement;
pub mod movement_visual;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement_visual::plugin, movement::plugin));
}
