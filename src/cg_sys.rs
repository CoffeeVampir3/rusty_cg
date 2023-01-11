pub use crate::*;
pub struct CGSys;

impl Plugin for CGSys {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteInteractionPlugin);
    }
}

/*
fn sprite_on_drop(
    mut sprites: Query<(Entity, &mut Transform), With<Sprite>>,
    immut_sprites: Query<(Entity, &GlobalTransform), With<Sprite>>,
    mut drop_ev: EventReader<DropEvent>,
    rapier_context: Res<RapierContext>,
) {
    for ev in drop_ev.iter() {
        let Ok((_, xform)) = sprites.get(ev.ent) else {continue};

        let mut max = f32::NEG_INFINITY;
        let mut res: Option<(Entity, Vec3)> = None;

        let filter: QueryFilter = QueryFilter::default().exclude_collider(ev.ent);

        rapier_context.intersections_with_point(xform.translation.truncate(), filter, |x| {
            let Ok((ent, inner_xform)) = immut_sprites.get(x) else {return true};

            let ord = inner_xform.translation().z;

            if ord >= max {
                res = Some((ent, inner_xform.translation()));
                max = ord;
            }
            true
        });

        let Ok((_, mut xform)) = sprites.get_mut(ev.ent) else {continue};
        let Some((drop_entity, drop_target)): Option<(Entity, Vec3)> =
        res
        else {
            xform.translation = ev.drag_info.start_pos;
            continue;
        };

        println!("Dropped on {:?}", drop_entity);
        //xform.translation = drop_target.truncate().extend(xform.translation.z);
    }
}
*/
