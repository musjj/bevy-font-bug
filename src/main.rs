use bevy::{prelude::*, text::FontSmoothing};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_ui)
        .run()
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, UiAntiAlias::Off));

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(90.0),
                        height: Val::Px(28.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::MAX,
                    BackgroundColor(Color::BLACK),
                ))
                .with_child((
                    Text::new("Hello World"),
                    TextFont {
                        font: asset_server.load("Silver.ttf"),
                        font_size: 19.0,
                        font_smoothing: FontSmoothing::None,
                    },
                    TextColor(Color::WHITE),
                ));
        });
}
