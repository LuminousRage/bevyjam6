use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerState>()
        .add_event::<ModifyInsanityEvent>()
        .add_event::<ModifyTurnEvent>()
        .add_systems(Update, (modify_insanity, modify_remaining_turns).chain());
}

// I don't think this should be default, but for now let's leave it as is.
#[derive(Resource, Default)]
struct PlayerState {
    insanity: u8,
    remaining_turns: u8,
}

// Add an event writer and write to this event when you want to modify the player's insanity.
#[derive(Event)]
struct ModifyInsanityEvent(i8);

#[derive(Event)]
struct ModifyTurnEvent(i8);

// where should these consts go?
const INSANITY_MAX: u8 = 100;
const TURN_MAX: u8 = 100;

fn modify_insanity(
    mut ev_insanity: EventReader<ModifyInsanityEvent>,
    mut player_state: ResMut<PlayerState>,
) {
    for ev in ev_insanity.read() {
        player_state.insanity = player_state
            .insanity
            .saturating_add_signed(ev.0)
            .min(INSANITY_MAX);
    }
}

fn modify_remaining_turns(
    mut ev_turns: EventReader<ModifyInsanityEvent>,
    mut player_state: ResMut<PlayerState>,
) {
    for ev in ev_turns.read() {
        player_state.remaining_turns = player_state
            .remaining_turns
            .saturating_add_signed(ev.0)
            .min(TURN_MAX);
    }
}
