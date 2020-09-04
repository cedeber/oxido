use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_fps_system.system())
            .add_system(fps_update_system.system());
    }
}

fn fps_update_system(diagnostics: Res<Diagnostics>, mut text: Mut<Text>) {
    if let Some((Some(fps), Some(average))) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .map(|x| (x.value(), x.average()))
    {
        text.value = format!("{:<3.3} ({:<3.3})", fps, average);
    }
}

fn setup_fps_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            text: Text {
                value: "FPS".to_string(),
                font: asset_server.load("assets/fonts/bit5x3.ttf").unwrap(),
                style: TextStyle {
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            },
            transform: Transform::new(Mat4::from_translation(Vec3::new(0.0, 0.0, 2.0))),
            ..Default::default()
        });
}
