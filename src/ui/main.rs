use bevy::{prelude::*, ecs::component, render::view::RenderLayers};

pub struct MainUIPlugin;

impl Plugin for MainUIPlugin {
    fn build(&self, app: &mut App) {
       app
        .add_event::<UIAddEvent>()
        .add_startup_system_to_stage(StartupStage::PreStartup, ui_init)
        .add_system(text_color_system)
        .add_system(ui_add_process)
        ;
    }
}
#[derive(Component)]
pub struct Flashing {
    pub speed: f32,
}

//split the ui into sections, have public methods to add entities as children.
//pass access to bundles? or ent_ids directly? all depends on how it works out
//recall that .id() on an entity returns the Entity object itself, which is just an id with
//a bunch of methods on it.

//have a bunch of entities on the resource line, pull them and add the requested e_ids as children.
#[derive(Resource)]
pub struct TimerUIMain {
    e: Entity,
}

//be sure to init these resources in their own init systems, if they don't exist and we try to request they
//will panic.

fn ui_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ui_layer = RenderLayers::layer(2);

    let timer_e_id = commands
        .spawn(
            NodeBundle {
                ..default()
            },
        )
        .insert(ui_layer)
        .id()
        ;

    let ui_mains = UIMains {
        e_timer: timer_e_id,
    };

    commands.insert_resource(ui_mains);
}

//register all the main component ids to attach to, then query it in the adder system
#[derive(Component, Resource)]
struct UIMains {
    e_timer: Entity,
}

pub enum UIType {
    Timer,
}

//pass e_id and ui main type
pub struct UIAddEvent {
    pub t: UIType,
    pub entity: Entity,
}

//how can i pass a reference to the UIAddEvent Reader to the callers?
//i need react-style useContext hooks that pass down props to child systems.
fn ui_add_process (
    mut add_evr: EventReader<UIAddEvent>,
    mut commands: Commands,

    ui_mains: Res<UIMains>,
)
{
    for ev in add_evr.iter()
    {
        let target: Entity;
        match ev.t {
            UIType::Timer => {
                target = ui_mains.e_timer;
            }
        }

        //push_children to the entity commands object in particular.
        let mut e_commands = commands.entity(target);

        e_commands.push_children(&[ev.entity]);
    }
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