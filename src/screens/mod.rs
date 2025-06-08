//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[states(scoped_entities)]
pub enum Screen {
    Splash,
    Title,
    Loading,
    Gameplay,
}

impl Default for Screen {
    fn default() -> Self {
        #[cfg(feature = "dev")]
        return Screen::Loading;

        #[cfg(not(feature = "dev"))]
        return Screen::Splash;
    }
}
