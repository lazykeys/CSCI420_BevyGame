use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;
use crate::stats::Stats;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, tick.run_if(on_timer(Duration::from_secs(1))));
    }
}

fn tick(mut stats_resource: ResMut<Stats>) {
    //Do every second here
    stats_resource.gain_points_per_second();
}
