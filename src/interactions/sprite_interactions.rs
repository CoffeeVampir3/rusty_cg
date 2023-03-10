pub use crate::*;
use super::helpers;

pub struct SpriteInteractionPlugin;

impl Plugin for SpriteInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<DropEvent>()

            .add_system(clear_drags.before(handle_mouse_interactions))
            .add_system(handle_mouse_interactions)
            .add_system(drag)
            .add_system(handle_dragging_changes)
            .add_system(process_hovering)
            ;

        #[cfg(DEBUG_ENABLED)]
        app
            .add_system(click_debugger)
            .add_system(interaction_debugger)
        ;
    }
}

#[cfg(DEBUG_ENABLED)]
fn click_debugger(mut click_reader: EventReader<ClickEvent>) {
    for ev in click_reader.iter() {
        println!("Clicked: {:?}", ev.clicked_ent);
    }
}

#[cfg(DEBUG_ENABLED)]
fn interaction_debugger(interactables: Query<(Entity, &Interactable), Changed<Interactable>>) {
    for (e, interactable) in &interactables {
        println!(
            "Interacted with {e:?}: {:?} -> {:?}",
            interactable.previous(),
            interactable.current()
        );
    }
}

fn process_hovering(
    main_window: Query<&Window, With<PrimaryWindow>>,
    interactables: Query<(Entity, &GlobalTransform, &Interactable)>,
    mut egui_context: EguiContexts,
) {
    let Ok(window) = main_window.get_single() else {return;};
    let Some(cursor_point_system) = window.cursor_position() else {return;};

    for (_, _, interact) in &interactables {
        let Interaction::Hovering = interact.current() else {continue};
        egui::Window::new("Hello")
        .title_bar(false)
        .resizable(false)
        .fixed_pos(egui::Pos2::new(cursor_point_system.x, window.height() - cursor_point_system.y))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("This is a card description peepo poggers");
        });
    }
}

fn drag(
    mut interactables: Query<(Entity, &mut Transform, &Interactable)>,
    main_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = main_window.get_single() else {return;};
    let Some(cursor_point_system) = window.cursor_position() else {return;};

    for (_, mut xform, interact) in interactables.iter_mut() {
        let Interaction::Dragging{offset,..} = interact.current() else {continue};
        xform.translation = (cursor_point_system + *offset).extend(xform.translation.z);
    }
}

fn handle_dragging_changes(
    mut interactables: Query<(Entity, &mut Transform, &mut Interactable), Changed<Interactable>>,
    sprites: Query<(Entity, &Sprite, &GlobalTransform)>,
    main_window: Query<&Window, With<PrimaryWindow>>,
    mut layer_sys: ResMut<TopLayer>,
    mut drop_writer: EventWriter<DropEvent>,
) {
    let Ok(window) = main_window.get_single() else {return;};
    let cursor_point_system_opt = window.cursor_position();
    let cursor_point_game_opt = helpers::get_window_relative_cursor_pos(&window);

    for (ent, mut xform, interact) in interactables.iter_mut() {
        if let Interaction::Dragging{..} = interact.current() {
            //Begin drag
            xform.translation = xform.translation.truncate().extend(layer_sys.top());
        } else 
        if let Interaction::Dragging{start_pos,offset} = interact.previous() {
            //If the cursor is in our window, and we hit something that isin't what we dropped, drop it onto the cursor.
            if let (Some(cursor_point_game), Some(cursor_point_system)) = (cursor_point_game_opt, cursor_point_system_opt) {
                let hit_result = helpers::pointcast_2d(cursor_point_game, &sprites, Some(ent));
                if let Some((hit_ent, ..)) = hit_result {
                    let drop_pos = (cursor_point_system+*offset).extend(xform.translation.z);

                    drop_writer.send(DropEvent {
                        dragged: ent,
                        dropped_on: hit_ent,
                        drop_pos,
                        start_pos: *start_pos,
                    });
                    
                    //TODO:: @Z, Possibly a temporary measure. 
                    xform.translation = *start_pos; 
                    continue;
                }
            }
            //Failed to drop on something, return to previous position.
            xform.translation = *start_pos;
        }
    }
}

pub struct ClickEvent {
    pub clicked_ent: Entity,
    pub clicked_pos: Vec2,
}

pub struct DropEvent {
    pub dragged: Entity,
    pub dropped_on: Entity,
    pub drop_pos: Vec3,
    pub start_pos: Vec3,
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
            Self::Dragging { .. } => write!(f, "Dragging"),
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

fn clear_drags(button_input: Res<Input<MouseButton>>, mut interactables: Query<&mut Interactable>) {
    if button_input.just_released(MouseButton::Left) {
        for mut interactable in interactables.iter_mut() {
            let Interaction::Dragging{..} = interactable.current() else {continue};
            interactable.change(Interaction::None);
        }
    }
}

fn handle_mouse_interactions(
    button_input: Res<Input<MouseButton>>,
    main_window: Query<&Window, With<PrimaryWindow>>,
    mut interactables: Query<(Entity, &mut Interactable)>,
    sprites: Query<(Entity, &Sprite, &GlobalTransform)>,
    mut click_writer: EventWriter<ClickEvent>,
) {
    let Ok(window) = main_window.get_single() else {return;};
    let Some(cursor_point_system) = window.cursor_position() else {return;};
    let Some(cursor_point_game) = helpers::get_window_relative_cursor_pos(&window) else {return};

    let hit_result = helpers::pointcast_2d(cursor_point_game, &sprites, None);

    let left_just_pressed = button_input.just_pressed(MouseButton::Left);
    for (ent, mut interactable) in interactables.iter_mut() {
        //Because clear_drags runs first we don't want to change the previous state
        //twice, so we simpy guard against this.
        if interactable.is_changed() {
            continue;
        }
        match hit_result {
            //Did we hit an entity and it matches our current iterator?
            Some((hit_ent, _, hit_xform)) if hit_ent == ent => match interactable.current() {
                Interaction::None => {
                    interactable.change(Interaction::Hovering);
                }

                Interaction::Hovering => {
                    if left_just_pressed {
                        let position = hit_xform.translation();
                        let sprite_position = position.truncate();
                        let cursor_offset = sprite_position - cursor_point_system;

                        click_writer.send(ClickEvent {
                            clicked_ent: ent,
                            clicked_pos: cursor_point_game,
                        });

                        interactable.change(Interaction::Dragging {
                            offset: cursor_offset,
                            start_pos: position,
                        });
                    }
                }

                _ => (),
            },
            _ => {
                let Interaction::Hovering = interactable.current() else {continue};
                interactable.change(Interaction::None);
            }
        }
    }
}