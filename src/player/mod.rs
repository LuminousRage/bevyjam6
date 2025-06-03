pub mod character;
mod configs;
pub mod movement;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((character::plugin, movement::plugin));
}
