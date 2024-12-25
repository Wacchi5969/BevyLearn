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

    // テキストを作成
    commands
        .spawn((
            // センタリング指定のノードを作成
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            // テキストを親ノードに配置
            parent.spawn((
                Text::new("Hello Bevy!"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}
