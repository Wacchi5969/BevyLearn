use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use std::env;

const VELOCITY_X: f32 = 100.0;

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

fn main() {
    // アセットのルートディレクトリを設定
    env::set_var("BEVY_ASSET_ROOT", "../../");
    App::new()
        .add_plugins(DefaultPlugins.set(create_window_plugin())) // 基本的なプラグインを追加
        .insert_resource(Time::<Fixed>::from_seconds(1.0)) // RunFixedMainLoopの更新間隔を設定
        .add_systems(Startup, setup) // 初期化のスケジュールにシステムを登録
        .add_systems(Update, (update_sprite_position, collision_screen))
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

    // ボールの生成
    commands.spawn((
        Sprite {
            image: asset_server.load("images/icon.png"),
            custom_size: Some(vec2(100., 100.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Direction::Right,
    ));
}

fn update_sprite_position(time: Res<Time>, mut sprite: Query<(&Direction, &mut Transform)>) {
    for (direction, mut transform) in &mut sprite {
        match direction {
            Direction::Right => transform.translation.x += VELOCITY_X * time.delta_secs(),
            Direction::Left => transform.translation.x -= VELOCITY_X * time.delta_secs(),
        }
    }
}

fn collision_screen(
    orthographic_projection: Query<(&OrthographicProjection)>,
    mut query_sprite: Query<(&Sprite, &mut Direction, &Transform)>,
) {
    if let Err(_) = orthographic_projection.get_single() {
        return;
    }

    let area = orthographic_projection.get_single().unwrap().area;
    for (sprite, mut direction, transform) in query_sprite.iter_mut() {
        if let None = sprite.custom_size {
            continue;
        }
        let offset = sprite.custom_size.unwrap().x / 2.0;
        match *direction {
            Direction::Right => {
                let right = transform.translation.x + offset;
                if area.max.x <= right {
                    *direction = Direction::Left;
                }
            }
            Direction::Left => {
                let left = transform.translation.x - offset;
                if left <= area.min.x {
                    *direction = Direction::Right;
                }
            }
        }
    }
}
