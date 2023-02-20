use bevy::prelude::*;

pub struct MainUIPlugin;

impl Plugin for MainUIPlugin {
    fn build(&self, app: &mut App) {
       app
        .add_startup_system(ui_init)
        .add_system(text_color_system);
    }
}
#[derive(Component)]
pub struct Flashing {
    pub speed: f32,
}

fn ui_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 1.0, 0.1)),
            ..Default::default()
        })
        .with_children(|commands| {
            commands
                .spawn(TextBundle::from_section(
                    "time",
                    //recall, asset server loads from /assets
                    TextStyle { font: asset_server.load("fonts/party.otf"), font_size: 100.0, color: Color::WHITE },
                ))
                .insert(Flashing { speed: 1.0 });
        })
        .insert(Name::new("mainUI"));
    
    commands
        .spawn(
            TextBundle::from_section(
                "time",
                //recall, asset server loads from /assets
                TextStyle { font: asset_server.load("fonts/party.otf"), font_size: 100.0, color: Color::WHITE },
            )
            .with_text_alignment(TextAlignment::CENTER_RIGHT)
        );
        
}

//make all the damn text flash different colors
fn text_color_system(
    time: Res<Time>, 
    mut query: Query<(&mut Text, &Flashing)>,
) {
    for (mut text, flashing) in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds * flashing.speed).sin() / 2.0 + 0.5,
            green: (0.75 * seconds * flashing.speed).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds * flashing.speed).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}