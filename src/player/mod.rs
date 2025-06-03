pub mod character;
mod configs;
mod movement;
pub mod state;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, character::plugin, movement::plugin));
}
