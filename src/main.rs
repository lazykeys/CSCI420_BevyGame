use bevy::prelude::*;

mod point_button;
mod upgrade_buttons;
mod camera;
mod stats;
mod time_incrementer;

fn main() {
    let mut app: App = App::new();

    app.add_plugins((DefaultPlugins, camera::CameraPlugin));
    app.add_plugins(point_button::ButtonPlugin);
    app.add_plugins(stats::StatsPlugin);
    app.add_plugins(upgrade_buttons::UpgradeButtonsPlugin);
    app.add_plugins(time_incrementer::TimerPlugin);
    app.run();
}