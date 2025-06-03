pub mod character;
pub mod state;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, character::plugin));
}
