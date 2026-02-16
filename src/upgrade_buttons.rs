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

enum UpgradeButtonType {
    IncreasePointsPerClick,
    IncreasePointsPerSecond
}

fn upgrade_buttons_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        Changed<Interaction>,
        >,
    mut stats_resource: ResMut<Stats>
) {
    for (entity, interaction, mut color, mut border_color, mut button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(BLACK);

                //TODO: add point gain on button click
                stats_resource.gain_points();

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

    commands.spawn(upgrade_button(0, UpgradeButtonType::IncreasePointsPerClick));
    commands.spawn(upgrade_button(1, UpgradeButtonType::IncreasePointsPerSecond));
}

fn upgrade_button(index: u32, button_type: UpgradeButtonType) -> impl Bundle {

    let button_text: &str = match button_type{
        UpgradeButtonType::IncreasePointsPerClick => "Cost: 10 points -> Increase Points Per Click By 3",
        UpgradeButtonType::IncreasePointsPerSecond => "Cost: 30 points -> Increase Points Per Second By 1"
    };

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
            Node {
                width: px(500),
                height: px(100),
                top: px(200 + (index * 300)),
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
