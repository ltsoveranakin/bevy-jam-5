use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::respawn::RespawnPlayerEvent;

pub struct HazardPlugin;

impl Plugin for HazardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_contact_hazard);
    }
}

fn player_contact_hazard(
    mut collision_event: EventReader<CollisionEvent>,
    mut respawn_player: EventWriter<RespawnPlayerEvent>,
) {
    let collided = if let Some(collision) = collision_event.read().next() {
        if let CollisionEvent::Started(_, _, _) = collision {
            respawn_player.send_default();
        }
        true
    } else {
        false
    };

    if collided {
        collision_event.clear();
    }
}
