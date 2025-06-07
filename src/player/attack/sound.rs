use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{asset_tracking::LoadResource, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<AttackSound>();

    app.register_type::<AttackAssets>();
    app.load_resource::<AttackAssets>();
}

#[derive(Event)]
pub enum AttackSound {
    Hit,
    Miss,
    Slash,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AttackAssets {
    #[dependency]
    pub wind_slash: Vec<Handle<AudioSource>>,
}

impl FromWorld for AttackAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            wind_slash: vec![
                assets.load("audio/sound_effects/combat/wind_slash_1.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_2.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_3.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_4.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_5.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_6.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_7.ogg"),
                assets.load("audio/sound_effects/combat/wind_slash_8.ogg"),
            ],
        }
    }
}

pub fn play_attack_sound(
    mut sound_event: EventReader<AttackSound>,
    mut commands: Commands,
    attack_assets: Res<AttackAssets>,
) {
    for sound in sound_event.read() {
        match sound {
            AttackSound::Hit => {
                // Play hit sound logic here
            }
            AttackSound::Miss => {
                // Play miss sound logic here
            }
            AttackSound::Slash => {
                let rng = &mut rand::thread_rng();
                let random_wind_slash = attack_assets.wind_slash.choose(rng).unwrap().clone();
                commands.spawn(sound_effect(random_wind_slash));
            }
        }
    }
}
