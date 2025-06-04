mod configs;
pub mod imp;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((imp::plugin,));
}
