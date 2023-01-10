use bevy::prelude::{App, Plugin, Resource};

pub struct SpriteLayerSystem;

//2^19
const F32_TWO_POWER_NINTEEN: f32 = 524288.0;
const F32_ONE: f32 = 1.0;

impl Plugin for SpriteLayerSystem {
    fn build(&self, app: &mut App) {
        app.insert_resource(TopLayer { current: F32_ONE });
    }
}

#[derive(Resource)]
pub struct TopLayer {
    current: f32,
}

impl TopLayer {
    pub fn top(&mut self) -> f32 {
        let cur = self.current;
        //2^19, abitarily small power of 2. This acts as an "epsilon" expression to move the float up a small increment.
        self.current += self.current / F32_TWO_POWER_NINTEEN;
        cur
    }
}

#[test]
fn prove_layer_numerical_stability() {
    let mut current = F32_ONE;
    for _ in 0..1_000_000_000_000_usize {
        let accum = current + current / F32_TWO_POWER_NINTEEN;
        if accum.is_infinite() {
            break;
        }
        assert_ne!(current, accum);
        current = accum;
    }
}
