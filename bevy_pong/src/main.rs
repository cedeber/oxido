use bevy::{
    app::AppExit,
    input::keyboard::{ElementState, KeyboardInput},
    prelude::*,
    render::pass::ClearColor,
};

// --- State ---
#[derive(Default)]
struct State {
    event_reader: EventReader<KeyboardInput>,
}

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

// --- Sprite Sheet ---
struct Gabe;

fn setup_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/gabe_idle_run.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(6.0),
            translation: Translation(Vec3::new(0.0, -215.0, 0.0)),
            ..Default::default()
        })
        .with(Gabe)
        .with(Timer::from_seconds(0.1, true));
}

fn animate_sprite_sheet(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_sprite_sheet.system())
            .add_system(animate_sprite_sheet.system());
    }
}

// --- Keyboard ---
/// This system prints 'A' key state
fn keyboard_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Gabe, &mut Translation)>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("'A' just released");
    }

    for (gabe, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        *translation.0.x_mut() += time.delta_seconds * direction * 350.;
    }
}

fn print_keyboard_event(
    mut state: ResMut<State>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    for event in state.event_reader.iter(&keyboard_input_events) {
        println!("{:?}", event);

        if let KeyboardInput {
            key_code: Some(key_code),
            state,
            ..
        } = event
        {
            match key_code {
                KeyCode::A => match state {
                    ElementState::Pressed => println!("=> 'A' just pressed"),
                    ElementState::Released => println!("=> 'A' just released"),
                },
                KeyCode::Escape => app_exit_events.send(AppExit),
                _ => {}
            }
        }
    }
}

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_input.system())
            .add_system(print_keyboard_event.system());
    }
}

// --- Main Application ---
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            // width: 300,
            // height: 300,
            resizable: false,
            // mode: WindowMode::Fullscreen { use_size: false },
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_default_plugins()
        .init_resource::<State>()
        .add_plugin(HelloPlugin)
        .add_plugin(SpriteSheetPlugin)
        .add_plugin(SpritePlugin)
        .add_plugin(KeyboardPlugin)
        .run();
}
