use std::collections::{VecDeque, vec_deque};

use avian2d::math::AdjustPrecision;
use bevy::prelude::*;
use bevy::{
    app::{App, Update},
    asset::Assets,
    ecs::{
        event::EventReader,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    image::TextureAtlasLayout,
    math::Vec2,
    time::Time,
    ui::Val::*,
};

use crate::PausableSystems;
use crate::asset_tracking::LoadResource;
use crate::enemy::boss::BossController;
use crate::enemy::configs::POSITION_1;
use crate::level::arena::LevelAssets;
use crate::menus::Menu;
use crate::screens::Screen;
use crate::screens::title::TitleAssets;
use crate::{
    enemy::{
        boss::boss,
        eye::EyeAssets,
        slime::{SlimeAssets, SlimeController, slime},
    },
    health::DeathEvent,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        process_script_events
            .run_if(resource_exists::<EyeAssets>)
            .run_if(resource_exists::<EyeAssets>)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
    app.add_systems(
        Update,
        (progress_dialogue, update_dialogue_text)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.insert_resource(get_game_script());
}
#[derive(Clone, Copy)]
pub enum Enemy {
    Boss,
    BlackSlime,
    RedSlime,
}

pub enum ScriptEvent {
    Wait(f32),
    WaitForSlimesDead,
    WaitForBossDead,
    Spawn(Enemy, Vec2),
    Dialogue(&'static str, &'static str),
    EndTheGame,
    None,
}

#[derive(Resource, Default)]
pub struct ScriptEventQueue {
    pub queue: VecDeque<ScriptEvent>,
}

fn get_game_script() -> ScriptEventQueue {
    //TODO: find these actual spawns?! and/or remove the ones that the player might be standing on.
    let botleft_spawn = Vec2::new(-500.0, 100.0);
    let botright_spawn = Vec2::new(500.0, 100.0);
    let topleft_spawn = Vec2::new(-500.0, 200.0);
    let topright_spawn = Vec2::new(500.0, 200.0);
    let topmiddle_spawn = Vec2::new(0.0, 200.0);
    let topleft_sky_spawn = Vec2::new(-100.0, 400.0);
    let topright_sky_spawn = Vec2::new(100.0, 400.0);
    let topmiddle_sky_spawn = Vec2::new(0.0, 400.0);
    let boss_spawn = POSITION_1;
    let queue = vec![
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Wait(3.0),
        ScriptEvent::Dialogue("Narrator","[As Ali regains consciousness, a voice echoes in his mind, sharp and demanding.]"),
        ScriptEvent::Dialogue("Mysterious Voice","I've been waiting so long for this. Get up already."),
        ScriptEvent::Dialogue("Narrator","[Ali pushes himself to his feet, his head spinning as the world around him begins to take shape. He blinks, struggling to comprehend his surroundings. He stands on a jagged rock platform, suspended high above an infinite sea of swirling, dark clouds.]"),
        ScriptEvent::Dialogue("Narrator","[Suddenly, a deafening roar shakes the air, a voice booming from the heavens, its power vibrating through Ali's very bones. ]"),
        ScriptEvent::Dialogue("Commanding Voice","You stand in my domain now, mortal. Welcome to your doom!"),
        ScriptEvent::Dialogue("Narrator","[Ali stumbles back, his heart pounding, panic rising in his chest as his eyes dart around in terror.]"),
        ScriptEvent::Dialogue("Mysterious Voice","Quick, there's no time! Pick me up, I'm over here!"),
        ScriptEvent::Dialogue("Narrator","[Ali shakes his head, trying to clear the fog in his mind, his voice trembling with panic.]"),
        ScriptEvent::Dialogue("Ali","What is this place? Who are you? Where are you? What's happening?!"),
        ScriptEvent::Dialogue("Mysterious Voice","The axe, Ali. A weapon of your bloodline, meant for you alone. Without it, you won't survive for long in this place."),
        ScriptEvent::Dialogue("Ali","I don't even know what's happening here! How do you even know my name? Why don't you just explain?"),
        ScriptEvent::Dialogue("Mysterious Voice","There's no time for explanations! I'll explain soon enough just pick up the axe. Now!"),
        ScriptEvent::Dialogue("Narrator","[Ali hesitates, his mind racing with uncertainty. But then his gaze locks onto the large axe resting on the ground. A strange pull tugs at him, almost like an unspoken invitation. Without fully thinking, he crouches and reaches for it. The moment his fingers touch the handle, he feels an immediate, unshakable connection as if the axe was always meant to be in his hands.]"),
        ScriptEvent::Dialogue("Mysterious Voice","Well done. Now, you must prepare yourself. It's about to get dangerous. Take this time to get used to the axe it's the only thing that will keep you alive here."),
        ScriptEvent::Dialogue("Narrator","[A mass of dark, slimy forms materializes in front of Ali, their glistening, gelatinous bodies pulsing with a sickly light. They writhe and twitch, closing in on him with unnatural speed.]"),
        ScriptEvent::Dialogue("Commanding Voice","Let's see how you fare against my creations!"),
        ScriptEvent::Dialogue("Mysterious Voice","Slimes. They're weak, but there will be many more. Use the axe get ready!"),
        ScriptEvent::Dialogue("Tip","Hold or press X to attack. As your weapon chain reacts from hitting enemies to gain fury (Red), its swiftness increases. Attack again during the reset period to continue the chain reaction, or miss and go into the cooldown phase (blue)."),
        ScriptEvent::Wait(1.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,topleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,topright_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Wait(3.0),
        ScriptEvent::Dialogue("Mysterious Voice","Well done, Ali. You've survived the first wave."),
        ScriptEvent::Dialogue("Ali","Are you gonna tell me what's going on now?"),
        ScriptEvent::Dialogue("Mysterious Voice","My name is Asad. I sealed myself away with Zha'kthar, an ancient monster. The seal has weakened, and I've seen a vision you are the one who can stop him."),
        ScriptEvent::Dialogue("Ali","A vision? Why me?"),
        ScriptEvent::Dialogue("Asad","Your bloodline is the key. The seal brought you here because you are the only one who can defeat him."),
        ScriptEvent::Dialogue("Zha'kthar","You will fail!"),
        ScriptEvent::Dialogue("Asad","Stay focused, the real battle is just beginning."),
        ScriptEvent::Wait(1.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,topleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,topright_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Spawn(Enemy::BlackSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,botright_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Spawn(Enemy::BlackSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,botright_spawn),
        ScriptEvent::Wait(1.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,topleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,topright_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Wait(3.0),
        ScriptEvent::Dialogue("Asad","The seal weakens faster. Zha'kthar senses you now."),
        ScriptEvent::Dialogue("Ali","What do I do? I don't even know what's going on!"),
        ScriptEvent::Dialogue("Asad","You must stop him before he breaks free completely. It's your only choice."),
        ScriptEvent::Dialogue("Zha'kthar","You are nothing. I will destroy you!"),
        ScriptEvent::Dialogue("Asad","Focus! The next wave is worse."),
        ScriptEvent::Wait(1.0),
        ScriptEvent::Spawn(Enemy::RedSlime,topleft_sky_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topright_sky_spawn),
        ScriptEvent::Wait(5.0),
        ScriptEvent::Spawn(Enemy::RedSlime,topleft_sky_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topright_sky_spawn),
        ScriptEvent::Wait(5.0),
        ScriptEvent::Spawn(Enemy::RedSlime,topleft_sky_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topright_sky_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Wait(3.0),
        ScriptEvent::Dialogue("Ali","I can't keep this up!"),
        ScriptEvent::Dialogue("Asad","You can. The only way out is through him. Zha'kthar's power is growing."),
        ScriptEvent::Dialogue("Ali","I'm not ready!"),
        ScriptEvent::Dialogue("Asad","You are. It's your blood, your destiny. He must be stopped now."),
        ScriptEvent::Dialogue("Zha'kthar","You think you can stop me? You're weak!"),
        ScriptEvent::Dialogue("Asad","You're not weak, Ali. You have what it takes. Don't doubt yourself."),
        ScriptEvent::Wait(1.0),
        ScriptEvent::Spawn(Enemy::RedSlime,topleft_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topright_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,botright_spawn),
        ScriptEvent::Wait(10.0),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Spawn(Enemy::BlackSlime,topleft_sky_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,topright_sky_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topmiddle_spawn),
        ScriptEvent::Wait(3.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,botright_spawn),
        ScriptEvent::Wait(3.0),
        ScriptEvent::Spawn(Enemy::RedSlime,topmiddle_sky_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Wait(3.0),
        ScriptEvent::Dialogue("Asad","This is it. Zha'kthar's final form is coming."),
        ScriptEvent::Dialogue("Ali","I don't know if I can do this..."),
        ScriptEvent::Dialogue("Asad","You must. This is your moment."),
        ScriptEvent::Dialogue("Zha'kthar","You cannot defeat me. I will consume you!"),
        ScriptEvent::Dialogue("Asad","You've come this far. Now finish this."),
        ScriptEvent::Wait(1.0),
        ScriptEvent::Spawn(Enemy::Boss,boss_spawn),
        //TODO: maybe add to this part of the queue as the boss fight happens? A hacky way to do it would be to have the boss controller spawn slimes
        ScriptEvent::Wait(15.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,botright_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topright_spawn),
        ScriptEvent::Wait(10.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,topleft_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::Wait(5.0),
        ScriptEvent::Spawn(Enemy::RedSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::RedSlime,botright_spawn),
        ScriptEvent::Wait(5.0),
        ScriptEvent::Spawn(Enemy::BlackSlime,botleft_spawn),
        ScriptEvent::Spawn(Enemy::BlackSlime,botright_spawn),
        ScriptEvent::Wait(0.1),
        ScriptEvent::WaitForSlimesDead,
        ScriptEvent::WaitForBossDead,
        ScriptEvent::Wait(3.0),
        ScriptEvent::Dialogue("Ali","I... I did it. I stopped him."),
        ScriptEvent::Dialogue("Asad","You've slain Zha'kthar, but that was only part of the chain. The seal is weakened now. The storm is far from over."),
        ScriptEvent::Dialogue("Ali","What are you talking about? I stopped him. This nightmare should be over!"),
        ScriptEvent::Dialogue("Asad","You've triggered a reaction. Zha'kthar's death didn't end the threat. It's just the beginning of something much worse..."),
        ScriptEvent::Dialogue("Ali","What do you mean 'the beginning'?"),
        ScriptEvent::Dialogue("Asad","The seal that kept him bound is crumbling. You have unwittingly begun a chain reaction. The realm is unstable. But you still have a chance to escape it."),
        ScriptEvent::Dialogue("Ali","Escape? How?"),
        ScriptEvent::Dialogue("Asad","You can't. The realm will collapse soon. The destruction you've set in motion can't be stopped but you can leave. You were always meant to break the chain. Now, leave this place, and return to your world."),
        ScriptEvent::Dialogue("Ali","I don't know what's next, but I won't let this be in vain."),
        ScriptEvent::Dialogue("Asad","Then go, Ali. I can only guide you so far. The rest is yours to choose."),
        ScriptEvent::Dialogue("Ali","Goodbye, Asad. Thanks for showing me the way."),
        ScriptEvent::Dialogue("Asad","Choose wisely. Don't let the chain bind you."),
        ScriptEvent::Wait(3.0),
        ScriptEvent::EndTheGame,
        // this is for credits screen
        ScriptEvent::None,
    ]
    .into();
    ScriptEventQueue { queue }
}

fn process_script_events(
    mut commands: Commands,
    time: Res<Time>,
    slime_assets: Res<SlimeAssets>,
    eye_assets: Res<EyeAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut script_events: ResMut<ScriptEventQueue>,
    slimes: Query<&SlimeController>,
    bosses: Query<&BossController>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
    dialogue: Single<(&mut Dialogue, &mut Visibility)>,
) {
    let mut delta = time.delta_secs().adjust_precision();
    loop {
        if let Some(event) = script_events.queue.get_mut(0) {
            match event {
                ScriptEvent::Wait(remaining_time) => {
                    *remaining_time -= delta;
                    if *remaining_time > 0. {
                        break;
                    }
                    delta = -*remaining_time;
                }
                ScriptEvent::Spawn(enemy, position) => match enemy {
                    Enemy::Boss => {
                        commands.spawn(boss(
                            &eye_assets,
                            &mut texture_atlas_layouts,
                            position.extend(0.3),
                        ));
                    }
                    Enemy::BlackSlime => {
                        commands.spawn(slime(&slime_assets, position.extend(0.), false));
                    }
                    Enemy::RedSlime => {
                        commands.spawn(slime(&slime_assets, position.extend(0.), true));
                    }
                },
                ScriptEvent::WaitForSlimesDead => {
                    if !slimes.is_empty() {
                        break;
                    }
                }
                ScriptEvent::WaitForBossDead => {
                    if !bosses.is_empty() {
                        break;
                    }
                }
                ScriptEvent::Dialogue(speaker, spokage) => {
                    let (mut dialogue, mut visibility) = dialogue.into_inner();
                    dialogue.speaker = speaker.to_string();
                    dialogue.spokage = spokage.to_string();
                    *visibility = Visibility::Inherited;
                    break;
                }
                ScriptEvent::EndTheGame => {
                    next_menu.set(Menu::Results);
                }
                ScriptEvent::None => {
                    return;
                }
            }
            script_events.queue.remove(0); // Don't increment i — we just removed this item
        }
    }
}

fn progress_dialogue(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut script_events: ResMut<ScriptEventQueue>,
    mut dialogue: Single<&mut Visibility, With<Dialogue>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if !matches!(
            script_events.queue.get(0),
            Some(ScriptEvent::Dialogue(_, _))
        ) {
            // not a dialogue!
            return;
        }
        script_events.queue.remove(0); // Don't increment i — we just removed this item
        if !matches!(
            script_events.queue.get(0),
            Some(ScriptEvent::Dialogue(_, _))
        ) {
            **dialogue = Visibility::Hidden;
        }
    }
}

#[derive(Component)]
pub struct Dialogue {
    speaker: String,
    spokage: String,
}

impl Dialogue {
    fn new(speaker: String, spokage: String) -> Self {
        Dialogue { speaker, spokage }
    }
}

fn update_dialogue_text(dialogue: Single<&Dialogue>, text: Query<(&Name, &mut Text)>) {
    let Dialogue { speaker, spokage } = dialogue.into_inner();

    for (name, mut text) in text {
        match name.as_str() {
            "Speaker" => *text = Text(speaker.clone()),
            "Spokage" => *text = Text(spokage.clone()),
            _ => continue,
        }
    }
}

pub fn dialogue(dialogue_assets: &LevelAssets, title_assets: &TitleAssets) -> impl Bundle {
    (
        Name::new("Dialogue"),
        Visibility::Hidden,
        Dialogue::new(String::new(), String::new()),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(140.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        Pickable::IGNORE,
        children![
            (
                Name::new("Frame"),
                ImageNode::new(dialogue_assets.dialogue.clone()).with_flip_y(),
                Node {
                    width: Val::Px(800.),
                    height: Val::Px(800.),
                    position_type: PositionType::Absolute,
                    margin: UiRect::top(Val::Px(-190.)),
                    ..default()
                },
            ),
            (
                Name::new("Speaker"),
                Text("".into()),
                TextFont {
                    font_size: 30.0,
                    font: title_assets.crimson.clone(),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect::top(Val::Px(-390.)),
                    ..default()
                },
            ),
            (
                Name::new("Spokage"),
                Text("".into()),
                TextFont {
                    font_size: 23.0,
                    font: title_assets.crimson.clone(),
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(Color::WHITE),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect::top(Val::Px(-730.))
                        .with_left(Val::Px(300.))
                        .with_right(Val::Px(300.)),
                    ..default()
                },
            )
        ],
    )
}
