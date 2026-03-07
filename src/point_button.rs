use bevy::{input_focus::InputFocus, prelude::*};
use crate::stats::Stats;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, button_system);
    }
}

const BUTTON_ALIGN: AlignItems = AlignItems::Center;
const BUTTON_LAYOUT: JustifyContent = JustifyContent::Center;

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut commands: Commands,//for spawning audio
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut Button,
        ),
        Changed<Interaction>,
        >,
    mut stats_resource: ResMut<Stats>

) {
    for (entity, interaction, mut button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                stats_resource.gain_points();

                button.set_changed();
                commands.spawn((
                AudioPlayer::new(asset_server.load("audio/LemonClick.ogg")),
                PlaybackSettings::DESPAWN,
                ));
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(button(asset_server));
}

fn button(asset_server: Res<AssetServer>) -> impl Bundle {
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
            ImageNode::new(asset_server.load("sprites\\Lemon.png")),
            Node {
                width: px(200),
                height: px(200),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )],
    )
}
