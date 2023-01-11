pub use crate::*;
use bevy_rapier2d::prelude::RapierContext;
pub struct SpriteInteractionPlugin;

//Use state when dragging to avoid race conditions?

impl Plugin for SpriteInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(clear_drags.before(handle_mouse_interactions))
            .add_system(handle_mouse_interactions)
            .add_system(test_interactions)
            
            ;
    }
}

fn test_interactions(interactables: Query<(Entity, &Interactable), Changed<Interactable>>) {
    for (e, interactable) in &interactables {
        println!(
            "{e:?} changed interaction state to {:?} from {:?}",
            interactable.current(),
            interactable.previous()
        );
    }
}

#[derive(Reflect, Clone, Default, PartialEq, Debug)]
pub enum Interaction {
    #[default]
    None,
    Hovering,
    Dragging {
        offset: Vec2,
        start_pos: Vec3,
    },
}

#[derive(Reflect, Component, Clone, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct Interactable {
    state: Interaction,
    previous_state: Interaction,
}

impl Interactable {
    pub fn default() -> Self {
        Self {
            state: Interaction::None,
            previous_state: Interaction::None,
        }
    }

    pub fn current(&self) -> &Interaction {
        &self.state
    }
    pub fn previous(&self) -> &Interaction {
        &self.previous_state
    }
    fn change(&mut self, new_state: Interaction) {
        std::mem::swap(&mut self.previous_state, &mut self.state);
        self.state = new_state;
    }
}

fn clear_drags(
    button_input: Res<Input<MouseButton>>,
    mut interactables: Query<&mut Interactable>,
) {
    if button_input.just_released(MouseButton::Left) {
        for mut interactable in interactables.iter_mut() {
            match interactable.current() {
                Interaction::Dragging {offset: _,start_pos: _} => {
                    interactable.change(Interaction::None);
                }
                _ => (),
            }
        }
    }
}

fn handle_mouse_interactions(
    button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut interactables: Query<(Entity, &mut Interactable)>,
    sprites: Query<(Entity, &GlobalTransform), With<Sprite>>,
    rapier_context: Res<RapierContext>,
) {
    let Some(window) = windows.get_primary() else {return;};
    let Some(cursor_point_system) = window.cursor_position() else {return;};
    let cursor_point_game = helpers::get_window_relative_cursor_pos(&window);

    let hit_result = helpers::pointcast_2d(&rapier_context, cursor_point_game, &sprites);

    let left_just_pressed = button_input.just_pressed(MouseButton::Left);
    for (ent, mut interactable) in interactables.iter_mut() {
        //Because clear_drags runs first we don't want to change the previous state
        //twice, so we simpy guard against this.
        if interactable.is_changed() {
            continue;
        }
        match hit_result {
            //Did we hit an entity and it matches our current iterator?
            Some((hit_ent, hit_xform)) if hit_ent == ent => {
                match interactable.current() {
                    Interaction::None => {
                        interactable.change(Interaction::Hovering);
                    }

                    Interaction::Hovering => {
                        if left_just_pressed {
                            let position = hit_xform.translation();
                            let sprite_position = position.truncate();
                            let cursor_offset = sprite_position - cursor_point_system;

                            interactable.change(Interaction::Dragging {
                                offset: cursor_offset,
                                start_pos: position,
                            });
                        }
                    }

                    _ => (), //Dragging Action
                }
            }
            _ => {
                match interactable.current() {
                    //This is the case where we were previously hovering over something but are no longer.
                    Interaction::Hovering => {
                        println!("Hover Ended {ent:?}");
                        interactable.change(Interaction::None);
                    }

                    _ => (),
                }
            }
        }
    }
}
