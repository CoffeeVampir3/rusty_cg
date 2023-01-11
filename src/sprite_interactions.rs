pub use crate::*;
use bevy_rapier2d::prelude::RapierContext;
pub struct SpriteInteractionPlugin;

//Use state when dragging to avoid race conditions?

impl Plugin for SpriteInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_system(clear_drags.before(handle_mouse_interactions))
            .add_system(handle_mouse_interactions)

            .add_system(drag_sprite)
            .add_system(drop_sprite)

            .add_system(interaction_debugger)
            .add_system(click_debugger)
            
            ;
    }
}

fn click_debugger(
    mut click_reader: EventReader<ClickEvent>
) {
    for ev in click_reader.iter() {
        println!("Clicked: {:?}", ev.clicked_ent);
    }
}

fn interaction_debugger(interactables: Query<(Entity, &Interactable), Changed<Interactable>>) {
    for (e, interactable) in &interactables {
        println!(
            "Interacted with {e:?}: {:?} -> {:?}",
            interactable.previous(),
            interactable.current()
        );
    }
}

fn drag_sprite(
    mut interactables: Query<(Entity, &mut Transform, &Interactable)>,
    windows: Res<Windows>,
) {
    let Some(window) = windows.get_primary() else {return;};
    let Some(cursor_point_system) = window.cursor_position() else {return;};

    for (_, mut xform, interact) in interactables.iter_mut() {
        match interact.current() {
            Interaction::Dragging { offset, start_pos:_ } => {
                xform.translation = (cursor_point_system + *offset).extend(xform.translation.z);
            }
            _ => ()
        }
    }
}

fn drop_sprite(
    mut interactables: Query<(Entity, &mut Transform, &mut Interactable), Changed<Interactable>>,
) {
    for (_, mut xform, interact) in interactables.iter_mut() {
        match interact.previous() {
            Interaction::Dragging { start_pos,..} => {
                xform.translation = *start_pos;
            }
            _ => ()
        }
    }
}

pub struct ClickEvent {
    pub clicked_ent: Entity,
    pub clicked_pos: Vec2
}

#[derive(Reflect, Clone, Default, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Hovering,
    Dragging {
        offset: Vec2,
        start_pos: Vec3,
    },
}

impl std::fmt::Debug for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Hovering => write!(f, "Hovering"),
            Self::Dragging {..} => write!(f, "Dragging"),
        }
    }
}

#[derive(Reflect, Component, Clone, PartialEq, Debug)]
#[reflect(Component)]
pub struct Interactable {
    state: Interaction,
    previous_state: Interaction,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            state: Interaction::None,
            previous_state: Interaction::None,
        }
    }
}

impl Interactable {
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
                Interaction::Dragging { .. } => {
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
    mut click_writer: EventWriter<ClickEvent>
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

                            click_writer.send(ClickEvent { clicked_ent: ent, clicked_pos: cursor_point_game});

                            interactable.change(Interaction::Dragging {
                                offset: cursor_offset,
                                start_pos: position,
                            });
                        }
                    }

                    _ => (),
                }
            }
            _ => {
                match interactable.current() {
                    //This is the case where we were previously hovering over something but are no longer.
                    Interaction::Hovering => {
                        interactable.change(Interaction::None);
                    }

                    _ => (),
                }
            }
        }
    }
}
