use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::day_night::DayCycleSet;
use crate::player::Player;

const MELT_INTERVAL: f32 = 3.;

pub struct MeltingPlugin;

impl Plugin for MeltingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeUnderSun>().add_systems(
            Update,
            (increase_time_under_sun, increase_melt_stage).in_set(DayCycleSet),
        );
    }
}

#[derive(Resource, Default)]
pub struct TimeUnderSun(pub f32);

impl TimeUnderSun {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }
}

pub enum MeltStage {
    None,
    Partial,
    Half,
    Mostly,
    Completely,
}

impl MeltStage {
    pub fn get_speed_multiplier(&self) -> f32 {
        match self {
            MeltStage::None => 1.,
            MeltStage::Partial => 0.75,
            MeltStage::Half => 0.5,
            MeltStage::Mostly => 0.25,
            MeltStage::Completely => 0.,
        }
    }
}

fn increase_time_under_sun(
    player_query: Query<(Entity, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut time_under_sun: ResMut<TimeUnderSun>,
    time: Res<Time>,
) {
    let (entity, transform) = player_query.single();

    let ray_start = transform.translation.truncate();
    let ray_dir = Vec2::Y;
    let max_time_of_impact = 5000.;
    let solid = true; // doesn't matter in this case, ray will ALWAYS start in the player's collider, therefore must be excluded by filter below
    let filter = QueryFilter::default().exclude_collider(entity);

    if rapier_context
        .cast_ray(ray_start, ray_dir, max_time_of_impact, solid, filter)
        .is_some()
    {
        // hit a collider above, reset time under sun
        time_under_sun.reset();
    } else {
        // nothing above, keep increasing
        time_under_sun.0 += time.delta_seconds();
    }
}

fn increase_melt_stage(
    mut player_query: Query<&mut Player>,
    mut time_under_sun: ResMut<TimeUnderSun>,
) {
    let mut player = player_query.single_mut();

    if time_under_sun.0 >= MELT_INTERVAL {
        time_under_sun.reset();

        let next_melt_stage = match player.melt_stage {
            MeltStage::None => MeltStage::Partial,
            MeltStage::Partial => MeltStage::Half,
            MeltStage::Half => MeltStage::Mostly,
            MeltStage::Mostly => MeltStage::Completely,
            MeltStage::Completely => MeltStage::Completely,
        };

        player.melt_stage = next_melt_stage;
    }
}
