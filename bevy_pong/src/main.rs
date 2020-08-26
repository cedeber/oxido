use bevy::prelude::*;

// --- Person ---
struct Person;

struct Name(String);

fn setup_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Person, &Name)>,
) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup_people.system())
            .add_system(greet_people.system());
    }
}

// --- Sprite ---
fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/gabe_idle_run.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            scale: Scale(6.0),
            ..Default::default()
        });
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_sprite.system());
    }
}

// --- Main ---

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .add_plugin(SpritePlugin)
        .run();
}
