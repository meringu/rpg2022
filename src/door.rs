use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Door(pub usize);

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(system.system())
            .add_system(display_events.system());
    }
}

fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}

#[allow(clippy::type_complexity)]
fn system(
    narrow_phase: Res<NarrowPhase>,
    query_set: QuerySet<(Query<Entity, With<Player>>, Query<(Entity, &Door)>)>,
) {
    for player_entity in query_set.q0().iter() {
        for (door_entity, _door) in query_set.q1().iter() {
            if narrow_phase.intersection_pair(player_entity.handle(), door_entity.handle())
                == Some(true)
            {
                // TODO: Ask the player if they'd like to enter door.0
            }
        }
    }
}
