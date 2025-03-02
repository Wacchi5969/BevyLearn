use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use std::env;

fn main() {
    // アセットのルートディレクトリを設定
    env::set_var("BEVY_ASSET_ROOT", "../../");
    App::new()
        .add_plugins(DefaultPlugins.set(create_window_plugin())) // 基本的なプラグインを追加
        .insert_resource(Time::<Fixed>::from_seconds(1.0)) // RunFixedMainLoopの更新間隔を設定
        .add_systems(Startup, setup) // 初期化のスケジュールにシステムを登録
        .run();
}

fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Moving ball".to_string(),
            name: Some("Bevy learn".into()),
            resolution: (400., 400.).into(),
            present_mode: PresentMode::AutoVsync,
            // ウィンドウにフォーカス中のF5やCtr+Rなどのブラウザショートカットを抑制します
            prevent_default_event_handling: true,
            window_theme: Some(WindowTheme::Light),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..default()
            },
            resizable: false,
            ..default()
        }),
        ..default()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラを配置
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite::from_image(asset_server.load("images/icon.png")),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}
