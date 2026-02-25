use bevy::{color::palettes::basic::*, input_focus::InputFocus, prelude::*};
use crate::stats::Stats;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, button_system);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.85);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.95);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 1.00);

const BUTTON_ALIGN: AlignItems = AlignItems::Center;
const BUTTON_LAYOUT: JustifyContent = JustifyContent::Center;

#[derive(Component)]
struct PointButton;

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &PointButton
        ),
        Changed<Interaction>,
        >,
    mut stats_resource: ResMut<Stats>

) {
    for (entity, interaction, mut color, mut border_color, mut button, point_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(BLUE);

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
    commands.spawn(button());
}

fn button() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: BUTTON_ALIGN,
            justify_content: BUTTON_LAYOUT,
            ..default()
        },
        children![(
            Button,
            PointButton,
            Node {
                width: px(200),
                height: px(200),
                border: UiRect::all(px(5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
        )],
    )
}
