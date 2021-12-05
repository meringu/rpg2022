use std::collections::HashSet;

use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug)]
pub enum DoorEvents {
    EnteredDoorRange(usize),
    LeftDoorRagne(usize),
}

struct DoorsInRange(HashSet<usize>);

pub struct Door(pub usize);

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DoorEvents>()
            .add_startup_system(setup.system())
            .add_system(system.system())
            .add_system(display_events.system());
    }
}

fn display_events(mut door_events: EventReader<DoorEvents>) {
    for _door_event in door_events.iter() {
        // TODO: process option to enter door
        // println!("Received door event: {:?}", door_event);
    }
}

fn system(
    narrow_phase: Res<NarrowPhase>,
    player_query: Query<Entity, With<Player>>,
    door_query: Query<(Entity, &Door)>,
    mut door_events: EventWriter<DoorEvents>,
    mut doors_in_range_query: Query<&mut DoorsInRange>,
) {
    let mut doors_in_range_this_frame = HashSet::<usize>::new();

    for player_entity in player_query.iter() {
        for (door_entity, door) in door_query.iter() {
            if narrow_phase.intersection_pair(player_entity.handle(), door_entity.handle())
                == Some(true)
            {
                doors_in_range_this_frame.insert(door.0);
            }
        }
    }

    for mut doors_in_range in doors_in_range_query.iter_mut() {
        // Add doors that weren't in range last frame
        for door_id in doors_in_range_this_frame.iter() {
            if !doors_in_range.0.contains(door_id) {
                doors_in_range.0.insert(*door_id);
                door_events.send(DoorEvents::EnteredDoorRange(*door_id));
            }
        }
        // Remove doors that were in range last frame
        for door_id in doors_in_range
            .0
            .difference(&doors_in_range_this_frame)
            .into_iter()
            .cloned()
            .collect::<Vec<usize>>()
        {
            doors_in_range.0.remove(&door_id);
            door_events.send(DoorEvents::LeftDoorRagne(door_id));
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(DoorsInRange(HashSet::<usize>::new()));
}
