pub mod behaviour;
pub mod sound;
pub mod systems;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((systems::plugin, sound::plugin, behaviour::plugin));
}
