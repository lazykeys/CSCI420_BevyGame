use bevy::{prelude::*, window::{CursorIcon, CustomCursor, CustomCursorImage}};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cursor_setup);
    }
}

fn cursor_setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<Entity, With<Window>>)
{
    commands.entity(*window).insert(
        CursorIcon::Custom(CustomCursor::Image(
            CustomCursorImage { 
                handle: asset_server.load("sprites//Cursor.png"),
                ..default()
            }
        ))
    );
}