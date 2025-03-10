use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use std::env;

const VELOCITY: f32 = 100.0;

fn main() {
    // アセットのルートディレクトリを設定
    env::set_var("BEVY_ASSET_ROOT", "../../");
    App::new()
        .add_plugins(DefaultPlugins.set(create_window_plugin())) // 基本的なプラグインを追加
        .insert_resource(Time::<Fixed>::from_seconds(1.0)) // RunFixedMainLoopの更新間隔を設定
        .add_systems(Startup, setup) // 初期化のスケジュールにシステムを登録
        .add_systems(Update, update_position)
        .run();
}

fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Input key event".to_string(),
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

    // ボールの生成
    commands.spawn((
        Sprite {
            image: asset_server.load("images/icon.png"),
            custom_size: Some(vec2(100., 100.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_position(
    input_event: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut sprites: Query<(&Sprite, &mut Transform)>,
) {
    let speed = get_movement_speed(input_event);
    if let Ok((_, mut transform)) = sprites.get_single_mut() {
        transform.translation.x += speed.x * time.delta_secs();
        transform.translation.y += speed.y * time.delta_secs();
    }
}

fn get_movement_speed(input_event: Res<ButtonInput<KeyCode>>) -> Vec2 {
    let mut direction = Vec2::ZERO;
    for &keyCode in input_event.get_pressed() {
        // キー入力応じた移動方向を加算
        match keyCode {
            KeyCode::ArrowUp => {
                direction.y += 1.0;
            }
            KeyCode::ArrowDown => {
                direction.y -= 1.0;
            }
            KeyCode::ArrowLeft => {
                direction.x -= 1.0;
            }
            KeyCode::ArrowRight => {
                direction.x += 1.0;
            }
            _ => {}
        }
    }
    // 移動方向を正規化
    if let Some(normalized) = direction.try_normalize() {
        normalized * VELOCITY
    } else {
        Vec2::ZERO
    }
}
