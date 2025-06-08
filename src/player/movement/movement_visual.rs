use bevy::prelude::*;

use crate::{
    PausableSystems,
    animation::{Animation, reversible_animation},
    audio::sound_effect,
    player::{
        character::{Player, PlayerAssets, PlayerLayoutAssets, player_sprite},
        movement::movement::PlayerMovementState,
    },
};
const IDLE_FRAME_NUM: usize = 10;
const RUN_FRAME_NUM: usize = 32;
const JUMP_FRAME_NUM: usize = 34;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpriteImageChange>();
    app.add_systems(
        Update,
        (
            update_player_transform,
            (update_player_image, update_player_sprite_animation).chain(),
        )
            .in_set(PausableSystems),
    );
}

#[derive(Event)]
pub struct SpriteImageChange(pub PlayerMovementState);

fn update_player_transform(mut player: Single<(&Player, &mut Transform)>) {
    let (p, transform) = &mut *player;

    transform.scale.x = p.face_direction.x * transform.scale.x.abs();
}

fn update_player_image(
    mut sprite: Single<&mut Sprite, With<Player>>,
    mut image_change_event: EventReader<SpriteImageChange>,
    player_assets: Res<PlayerAssets>,
    player_layout: Res<PlayerLayoutAssets>,
) {
    for event in image_change_event.read() {
        let (image, texture_atlas) = player_sprite(event.0.clone(), &player_assets, &player_layout);
        sprite.image = image;
        sprite.texture_atlas = texture_atlas;
    }
}

fn update_player_sprite_animation(
    mut player: Single<(&mut Sprite, &mut PlayerMovementState), With<Player>>,
    animation: Res<Animation>,
    player_assets: Res<PlayerAssets>,
    mut commands: Commands,
) {
    let (sprite, player_mode) = &mut *player;

    let Some(texture_atlas) = &mut sprite.texture_atlas else {
        return;
    };

    let faster_timer = animation.1.just_finished();
    let slower_timer = animation.0.just_finished();

    if !faster_timer && !slower_timer {
        return;
    }

    match &mut **player_mode {
        PlayerMovementState::Idle(reverse) => {
            if !slower_timer {
                return;
            }
            reversible_animation(reverse, &mut texture_atlas.index, IDLE_FRAME_NUM);
        }
        PlayerMovementState::Run => {
            texture_atlas.index = (texture_atlas.index + 1) % RUN_FRAME_NUM;
            if texture_atlas.index == 4 || texture_atlas.index == 19 {
                commands.spawn(sound_effect(player_assets.player_step_sound.clone()));
            }
        }
        PlayerMovementState::Jump(_) => {
            if !slower_timer {
                return;
            }
            texture_atlas.index = (texture_atlas.index + 1) % JUMP_FRAME_NUM
        }
        PlayerMovementState::Dash(_) => {}
    }
}
