use bevy::prelude::*;

/// the handling of button-related events, inextricably tied to the analysis system, of course.
pub struct ButtonEvPlugin;

impl Plugin for ButtonEvPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ButtonEvent>()        
            ;
    }
}

/// check for matching payloads to handle events.
/// this isn't pretty, but it's better than passing a ref of the door ent to the button, because that would have 
/// to be at runtime.
#[derive(Default)]
pub struct ButtonEvent {
    pub payload: String,
}
