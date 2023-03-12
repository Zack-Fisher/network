use std::collections::VecDeque;

use bevy::prelude::*;

use crate::ui::UIState;

pub struct ChatProcessingPlugin;

impl Plugin for ChatProcessingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Messages::default())

            .add_system(handle_typing_input)
            .add_system(message_mgmt)
            ;
    }
}

/// the ui just renders this data structure. this is the shared chat state
/// that propagates and is read from by the rest of the app.
#[derive(Resource, Default)]
pub struct Messages {
    /// use a vecdeque for more flexibility, we'll likely want to pop from the 
    /// front of the vector.
    pub vec: VecDeque<Message>,
}

pub struct Message {
    pub name: String,
    pub text: String,
}

fn handle_typing_input (
    ui_state: Res<State<UIState>>,

    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,

    mut messages: ResMut<Messages>,
)
{
    if ui_state.current() != &UIState::ChatFocus {return;}

    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        messages.vec.push_front(
            Message {
                name: String::from("player"),
                text: *string,
            }
        )
    }
}

/// clean up the Messages resource's vector
fn message_mgmt (
    mut messages: ResMut<Messages>,
)
{
    if messages.vec.len() > 10 {
        messages.vec.pop_back();
    }
}
