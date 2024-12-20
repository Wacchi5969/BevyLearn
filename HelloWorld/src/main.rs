use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 基本的なプラグインを追加
        .add_systems(Startup, setup) // 初期化のスケジュールにシステムを登録
        .run();
}

fn setup(mut commands: Commands) {
    // カメラを配置
    commands.spawn(Camera2d::default());
}
