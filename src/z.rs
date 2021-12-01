use bevy::prelude::*;

#[derive(Default)]
pub struct ZSync(pub f32);

pub struct ZPlugin;

impl Plugin for ZPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PostUpdate, system.system());
    }
}

fn system(mut query: Query<(&mut Transform, &mut GlobalTransform, &ZSync)>) {
    for (mut transform, global, z) in query.iter_mut() {
        transform.translation.z =
            (transform.translation.y - global.translation.y - z.0) / 1000.0 + 10.0;
    }
}
