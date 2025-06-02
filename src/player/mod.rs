pub mod state;

mod player;
pub use player::spawn_player;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, player::plugin));
}
