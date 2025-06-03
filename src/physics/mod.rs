pub mod creature;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((creature::plugin));
}
