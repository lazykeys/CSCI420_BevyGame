use bevy::{image::ImageSamplerDescriptor, prelude::*};

mod point_button;
mod upgrade_buttons;
mod camera;
mod stats;
mod time_incrementer;
mod cursor;

fn main() {
    let mut app: App = App::new();
    app.add_plugins((DefaultPlugins.set(ImagePlugin {default_sampler: ImageSamplerDescriptor::nearest() }), camera::CameraPlugin));
    app.add_plugins(point_button::ButtonPlugin);
    app.add_plugins(stats::StatsPlugin);
    app.add_plugins(upgrade_buttons::UpgradeButtonsPlugin);
    app.add_plugins(time_incrementer::TimerPlugin);
    app.add_plugins(cursor::CursorPlugin);
    app.run();
}