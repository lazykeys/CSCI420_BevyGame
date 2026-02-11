use bevy::{color::palettes::basic::*, input_focus::InputFocus, prelude::*};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, button_system);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const BUTTON_ALIGN: AlignItems = AlignItems::Baseline;
const BUTTON_LAYOUT: JustifyContent = JustifyContent::FlexStart;

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, children) in
        &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(RED);

                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    //commands.spawn(Camera2d);
    commands.spawn(button(&assets));
}

fn button(_asset_server: &AssetServer) -> impl Bundle {
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
            Node {
                width: px(150),
                height: px(65),
                top: px(15),
                left: px(15),
                border: UiRect::all(px(5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(Text::new("Button"), TextColor(Color::srgb(0.9, 0.9, 0.9)))]
        )],
    )
}
