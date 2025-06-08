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
    Hit(f32),
    Miss(f32),
    Slash,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AttackAssets {
    #[dependency]
    pub wind_slash: Vec<Handle<AudioSource>>,
    pub weapon_hit: Vec<Handle<AudioSource>>,
    pub weapon_miss: Vec<Handle<AudioSource>>,
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
            weapon_hit: vec![
                assets.load("audio/sound_effects/combat/weapon_hit_0008.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_0016.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_0031.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_0062.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_0125.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_0250.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_0500.ogg"),
                assets.load("audio/sound_effects/combat/weapon_hit_1000.ogg"),
            ],
            weapon_miss: vec![
                assets.load("audio/sound_effects/combat/weapon_miss_0008.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_0016.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_0031.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_0062.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_0125.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_0250.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_0500.ogg"),
                assets.load("audio/sound_effects/combat/weapon_miss_1000.ogg"),
            ],
        }
    }
}

fn wut_sound_to_play(cooldown_second: f32) -> usize {
    match cooldown_second {
        a if a >= 1.0 => 7,
        a if a >= 0.5 => 6,
        a if a >= 0.25 => 5,
        a if a >= 0.125 => 4,
        a if a >= 0.062 => 3,
        a if a >= 0.031 => 2,
        a if a >= 0.016 => 1,
        a if a >= 0.008 => 0,
        _ => 0,
    }
}

pub fn play_attack_sound(
    mut sound_event: EventReader<AttackSound>,
    mut commands: Commands,
    attack_assets: Res<AttackAssets>,
) {
    for event in sound_event.read() {
        let sound = match event {
            AttackSound::Hit(cooldown_second) => {
                let sound_index = wut_sound_to_play(*cooldown_second);
                attack_assets.weapon_hit[sound_index].clone()
            }
            AttackSound::Miss(cooldown_second) => {
                let sound_index = wut_sound_to_play(*cooldown_second);
                attack_assets.weapon_miss[sound_index].clone()
            }
            AttackSound::Slash => {
                let rng = &mut rand::thread_rng();
                attack_assets.wind_slash.choose(rng).unwrap().clone()
            }
        };

        commands.spawn(sound_effect(sound));
    }
}
