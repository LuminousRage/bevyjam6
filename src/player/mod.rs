pub mod attack;
pub mod character;
mod configs;
pub mod input;
pub mod movement;
pub mod weapon;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        character::plugin,
        weapon::plugin,
        movement::plugin,
        attack::plugin,
    ));
}
