use bevy::{color::palettes::basic::*, input_focus::InputFocus, prelude::*};
use crate::stats::Stats;

pub struct UpgradeButtonsPlugin;

impl Plugin for UpgradeButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, upgrade_buttons_system);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

const UPGRADE_BUTTON_ALIGN: AlignItems = AlignItems::Baseline;
const UPGRADE_BUTTON_LAYOUT: JustifyContent = JustifyContent::SpaceBetween;


#[derive(Component)]
enum UpgradeButtonType {
    IncreasePointsPerClick,
    IncreasePointsPerSecond
}

//this is what runs when the button is clicked
fn upgrade_buttons_system(
    mut commands: Commands,//for spawning audio
    asset_server: Res<AssetServer>,


    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &UpgradeButtonType
        ),
        Changed<Interaction>,
        >,
    mut stats_resource: ResMut<Stats>
) {
    for (entity, interaction, mut color, mut border_color, mut button, button_type) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(BLACK);

                match button_type
                {
                    UpgradeButtonType::IncreasePointsPerClick => stats_resource.increase_points_per_click(1),
                    UpgradeButtonType::IncreasePointsPerSecond => stats_resource.increase_points_per_second(1),
                }

                commands.spawn((
                AudioPlayer::new(asset_server.load("audio/LemonClick.ogg")),
                PlaybackSettings::DESPAWN,
                ));

                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}

fn setup(mut commands: Commands) {

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Baseline,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        children![(
            upgrade_button(
                String::from("Cost: 10 points -> Increase Points Per Click By 3"),
                UpgradeButtonType::IncreasePointsPerClick
            ),
            upgrade_button(
                String::from("Cost: 30 points -> Increase Points Per Second By 1"),
                UpgradeButtonType::IncreasePointsPerSecond
            )
        )],
    ));
}

fn upgrade_button(button_text: String, button_type: UpgradeButtonType) -> impl Bundle {

    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: UPGRADE_BUTTON_ALIGN,
            justify_content: UPGRADE_BUTTON_LAYOUT,
            ..default()
        },
        children![(
            Button,
            button_type,
            Node {
                width: px(500),
                height: px(100),
                //top: px(200 + (index * 300)),
                border: UiRect::all(px(3)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(Text::new(button_text), TextColor(Color::srgb(0.9, 0.9, 0.9)))]
        )],
    )
}
