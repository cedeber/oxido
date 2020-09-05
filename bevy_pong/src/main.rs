use bevy::{app::AppExit, prelude::*, render::pass::ClearColor, window::WindowMode};

mod fps;

// --- Paddles --- //
#[derive(PartialEq)]
enum Side {
    Left,
    Right,
}

struct Paddle {
    speed: f32,
    side: Side,
}

fn setup_paddles_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let paddle_left_handle = asset_server.load("assets/images/bat00.png").unwrap();
    let paddle_right_handle = asset_server.load("assets/images/bat10.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(paddle_left_handle.into()),
            translation: Translation(Vec3::new(-360., 0., 1.0)),
            ..Default::default()
        })
        .with(Paddle {
            speed: 350.,
            side: Side::Left,
        })
        .spawn(SpriteComponents {
            material: materials.add(paddle_right_handle.into()),
            translation: Translation(Vec3::new(360., 0., 1.0)),
            ..Default::default()
        })
        .with(Paddle {
            speed: 350.,
            side: Side::Right,
        });
}

// --- Ball --- //
struct Ball {
    speed: f32,
}

fn setup_ball_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball_handle = asset_server.load("assets/images/ball.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(ball_handle.into()),
            translation: Translation(Vec3::new(0., 0., 1.0)),
            ..Default::default()
        })
        .with(Ball { speed: 500. });
}

// --- Background --- //
fn setup_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/images/table.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

// --- Keyboard --- //
fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut query: Query<(&Paddle, &mut Translation)>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit)
    }

    for (paddle, mut translation) in &mut query.iter() {
        let mut direction = 0.0;

        if paddle.side == Side::Right {
            if keyboard_input.pressed(KeyCode::J) {
                direction += 1.0;
            }

            if keyboard_input.pressed(KeyCode::N) {
                direction -= 1.0;
            }
        } else {
            if keyboard_input.pressed(KeyCode::F) {
                direction += 1.0;
            }

            if keyboard_input.pressed(KeyCode::V) {
                direction -= 1.0;
            }
        }

        *translation.0.y_mut() += time.delta_seconds * direction * paddle.speed;
        *translation.0.y_mut() = f32::max(-160., f32::min(160., translation.0.y()));
    }
}

// --- Main Application --- //
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("Bevy Boing!"),
            width: 800,
            height: 480,
            resizable: false,
            mode: WindowMode::Fullscreen { use_size: true },
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_default_plugins()
        .add_plugin(fps::FpsPlugin)
        .add_startup_system(setup_paddles_system.system())
        .add_startup_system(setup_ball_system.system())
        .add_startup_system(setup_background_system.system())
        .add_system(keyboard_input_system.system())
        .run();
}
