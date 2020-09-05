use bevy::{app::AppExit, prelude::*};

struct Gabe {
    speed: f32,
}

fn setup_sprite_sheet_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/gabe_run.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(6.0),
            ..Default::default()
        })
        .with(Gabe { speed: 350. })
        .with(Timer::from_seconds(0.1, true));
}

fn animate_sprite_sheet_system(
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

fn keyboard_input_system(
    time: Res<Time>,
    window: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut query: Query<(&Gabe, &mut Translation)>,
) {
    let window = window.get_primary().unwrap();
    let width = ((window.width - 24 * 6) / 2) as f32;

    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit)
    }

    for (gabe, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        *translation.0.x_mut() += time.delta_seconds * direction * gabe.speed;

        // bound Gabe
        *translation.0.x_mut() = f32::max(-width, f32::min(width, translation.0.x()));
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup_sprite_sheet_system.system())
        .add_system(animate_sprite_sheet_system.system())
        .add_system(keyboard_input_system.system())
        .run();
}
