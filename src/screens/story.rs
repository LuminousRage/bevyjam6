//! The title screen that appears after the splash screen.

use bevy::{prelude::*, ui::Val::*};

use crate::screens::Screen;
use crate::screens::title::TitleAssets;
use crate::theme::prelude::*;
use crate::theme::widget::ui_root;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Story),
        (spawn_background, spawn_settings_menu),
    );
    app.add_systems(Update, run_story.run_if(in_state(Screen::Story)));
}

fn spawn_background(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Background Screen"),
        BackgroundColor(Color::BLACK),
        StateScoped(Screen::Story),
    ));
}

#[derive(Component)]
pub struct StoryScreen(usize);

fn spawn_settings_menu(mut commands: Commands, title_assets: Res<TitleAssets>) {
    commands.spawn((
        ui_root("Story"),
        GlobalZIndex(2),
        StateScoped(Screen::Story),
        children![
            (Name::new("Story"),
            Node {
                display: Display::Flex,
                position_type: PositionType::Relative,
                width: Percent(80.0),
                height: Percent(80.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Px(20.0),
                ..default()
            },
            StoryScreen(0),
            children![widget::text("[Ali, young and often overlooked by his family of skilled magicians, has always felt like an outsider. While they hone their craft and perfect their magic, he longs for something more power. Power that will earn their respect and prove he's more than just a boy. He's heard rumors of a hidden room deep within the estate, a forgotten chamber holding the family's most guarded secrets.]", &title_assets),

            ]),
            widget::text("Press X to continue", &title_assets),
        ],
    ));
}

fn run_story(
    story_screen: Single<(Entity, &mut StoryScreen)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    title_assets: ResMut<TitleAssets>,
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let (entity, mut story) = story_screen.into_inner();

    let next = match story.0 {
        0 => Some(widget::text(
            "[Inside, the room is filled with shelves, each piled high with books some ancient, some newly bound, all forgotten with time. But one catches his eye. A book, untouched, as if it has never seen the passing years. ]",
            &title_assets,
        )),
        1 => Some(widget::text(
            "[As his fingers brush its cover, the air around him shifts. The other books begin to crumble, turning to dust, one after another, as if drawn into an inevitable collapse. A chain reaction. ]",
            &title_assets,
        )),
        2 => Some(widget::text(
            "[The book grows heavier in his hands, reluctant to release him. Something unseen tugs at him, pulling him in. He opens it. A sharp pain lances through his skull.]",
            &title_assets,
        )),
        3 => Some(widget::text("[And then...darkness.]", &title_assets)),
        _ => None,
    };
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        if let Some(widget) = next {
            let child = commands.spawn(widget).id();
            commands.entity(entity).add_child(child);
            story.0 += 1;
        } else {
            if story.0 == 4 {
                commands.entity(entity).despawn_related::<Children>();
                let child = commands
                    .spawn(widget::text(
                        "Press Z to Jump. Press X to Attack. Press C to Dash.",
                        &title_assets,
                    ))
                    .id();
                let child2 = commands
                    .spawn(widget::text(
                        "Use Arrow keys to move. And press P to pause.",
                        &title_assets,
                    ))
                    .id();
                commands.entity(entity).add_child(child);
                commands.entity(entity).add_child(child2);

                story.0 += 1;
            } else {
                next_screen.set(Screen::Gameplay)
            }
        }
    }
}
