//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod story;
pub mod title;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        story::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[states(scoped_entities)]
pub enum Screen {
    Splash,
    Title,
    Loading,
    Story,
    Gameplay,
}

impl Default for Screen {
    fn default() -> Self {
        #[cfg(feature = "dev")]
        // return Screen::Splash;
        return Screen::Loading;

        #[cfg(not(feature = "dev"))]
        return Screen::Splash;
    }
}
