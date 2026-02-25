use bevy::prelude::*;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Stats {
                points_per_click: 1,
                points_per_second: 1,
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

    pub fn gain_points(&mut self)
    {
        self.points += self.points_per_click;
    }
    //function for the timer
    pub fn gain_points_per_second(&mut self)
    {
        self.points += self.points_per_second;
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
        Node {
            top: px(50),
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children!
        [(
            //The actual text that displays
            Text::new(format!("Points: {}", stats_resource.points)),

            //Text font and sizing
            TextFont {
                font: stats_font,
                font_size: stats_font_size,
                ..default()
            },

            //what struct holds the data!
            StatsText
        )]
    ));
}

fn draw_stats (
    stats_resource: Res<Stats>,
    mut stats_text: Query<&mut Text, With<StatsText>>
)
{
    for mut entity in stats_text.iter_mut() {
        entity.0 = format!("Points: {}", stats_resource.points);
    }
}