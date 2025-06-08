use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Animation>();
    app.add_systems(Update, tick_animation);
}

pub fn reversible_animation(reverse: &mut bool, frame: &mut usize, num_frames: usize) {
    if *reverse {
        if *frame == 0 {
            *reverse = false;
            return;
        }
        *frame -= 1;
        if *frame == 0 {
            *reverse = false;
        }
    } else {
        if *frame == num_frames - 1 {
            *reverse = true;
        }
        *frame += 1;
        if *frame == num_frames - 1 {
            *reverse = true;
        }
    }
}

#[derive(Resource)]
pub struct Animation(pub Timer);

impl Default for Animation {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}

fn tick_animation(time: Res<Time>, mut animation: ResMut<Animation>) {
    animation.0.tick(time.delta());
}
