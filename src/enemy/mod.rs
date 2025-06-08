pub mod boss;
pub mod configs;
pub mod eye;
pub mod slime;

// pub configs::;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((slime::plugin, boss::plugin, eye::plugin));
}
