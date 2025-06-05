pub mod boss;
mod configs;
pub mod slime;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((slime::plugin, boss::plugin));
}
