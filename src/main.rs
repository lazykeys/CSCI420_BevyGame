use bevy::prelude::*;

mod button;
mod camera;

fn main() {
    let mut app: App = App::new();

    app.add_plugins((DefaultPlugins, camera::CameraPlugin));
    app.add_plugins(button::ButtonPlugin);
    app.run();
}
