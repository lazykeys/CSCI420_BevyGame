use bevy::prelude::*;
use crate::point_button::PointButtonClicked;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Stats {
                points_per_click: 1,
                ..default()
            });
        app.add_systems(Startup, build_stats);
        app.add_systems(Update, draw_stats);
    }
}

#[derive(Resource, Default)]
pub struct Stats
{
    points: u32,
    points_per_click: u32,
    points_per_second: u32,
    seconds_elapsed: u32
}

impl Stats {

    pub fn gain_points(&mut self, mut point_button_click_reader: MessageReader<PointButtonClicked>)
    {
        for button_click in point_button_click_reader.read() {
            self.points += self.points_per_click;
        }
    }
    //function for the timer
    pub fn gain_points_per_second(&mut self)
    {
        self.points += self.points_per_click;
    }

    pub fn increase_points_per_click(&mut self, increase: u32)
    {
        if self.points >= 10
        {
            self.points -= 10;
            self.points_per_click += increase;
        }
    }

    pub fn increase_points_per_second(&mut self, increase: u32)
    {
        if self.points >= 30
        {
            self.points -= 30;
            self.points_per_second += increase;
        }
    }
}

#[derive(Component)]
pub struct StatsText;

//this function draws the player's points to the screen
fn build_stats (
    mut commands: Commands,
    stats_resource: Res<Stats>
)
{
    //Text Settings!!
    let stats_font: Handle<Font> = default();
    let stats_font_size: f32 = 32.0;

    commands.spawn((
        //The actual text that displays
        Text2d::new(format!("Points: {}", stats_resource.points)),

        //Text font and sizing
        TextFont {
            font: stats_font,
            font_size: stats_font_size,
            ..default()
        },

        //Text justification
        TextLayout::new_with_justify(Justify::Center),

        //Text positioning
        Transform::from_xyz(0.0, 300.0, 0.0),

        //what struct holds the data!
        StatsText
    ));
}

fn draw_stats (
    stats_resource: Res<Stats>,
    mut stats_text: Query<&mut Text2d, With<StatsText>>
)
{
    for mut entity in stats_text.iter_mut() {
        entity.0 = format!("Points: {}", stats_resource.points);
    }
}