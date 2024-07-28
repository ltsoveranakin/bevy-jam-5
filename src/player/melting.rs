use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::day_night::DayCycleSet;
use crate::player::{CAST_COLLIDER_SCALE, Player, PlayerSprite};

const MELT_INTERVAL: f32 = 3.;

pub struct MeltingPlugin;

impl Plugin for MeltingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeUnderSun>()
            .add_event::<SetMeltStageEvent>()
            .add_systems(
                Update,
                (
                    (increase_time_under_sun, increase_melt_stage).in_set(DayCycleSet),
                    update_set_melt_stage,
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct TimeUnderSun(pub f32);

#[derive(Event)]
pub struct SetMeltStageEvent(pub MeltStage);

impl TimeUnderSun {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }
}

#[derive(Default, Copy, Clone)]
pub enum MeltStage {
    #[default]
    None,
    Partial,
    Half,
    Mostly,
}

impl MeltStage {
    pub fn get_speed_multiplier(&self) -> f32 {
        match self {
            MeltStage::None => 1.,
            MeltStage::Partial => 0.8,
            MeltStage::Half => 0.7,
            MeltStage::Mostly => 0.6,
        }
    }

    pub fn get_tile_sprite_offset(&self) -> Vec2 {
        match self {
            MeltStage::None => Vec2::splat(16.),
            MeltStage::Partial => Vec2::new(48., 16.),
            MeltStage::Half => Vec2::new(16., 48.),
            MeltStage::Mostly => Vec2::splat(48.),
        }
    }

    pub fn get_collider_dimensions(&self) -> Vec2 {
        match self {
            MeltStage::None => Vec2::new(4.5, 6.),
            MeltStage::Partial => Vec2::new(3., 6.),
            MeltStage::Half => Vec2::new(1.5, 6.),
            MeltStage::Mostly => Vec2::new(0.5, 6.),
        }
    }

    pub fn compute_cast_collider_dimensions(dimensions: Vec2) -> Vec2 {
        dimensions * CAST_COLLIDER_SCALE
    }

    pub fn get_cast_collider_dimensions(&self) -> Vec2 {
        Self::compute_cast_collider_dimensions(self.get_collider_dimensions())
    }
}

fn update_set_melt_stage(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    mut player_sprite_query: Query<&mut Sprite, With<PlayerSprite>>,
    mut set_melt_stage_event: EventReader<SetMeltStageEvent>,
) {
    let (entity, mut player) = player_query.single_mut();
    let mut sprite = player_sprite_query.single_mut();

    if let Some(set_melt_stage) = set_melt_stage_event.read().next() {
        let collider_dimensions = set_melt_stage.0.get_collider_dimensions();
        let cast_collider_dimensions =
            MeltStage::compute_cast_collider_dimensions(collider_dimensions);

        sprite.rect = Some(Rect::from_center_half_size(
            set_melt_stage.0.get_tile_sprite_offset(),
            Vec2::splat(16.),
        ));

        commands
            .entity(entity)
            .remove::<Collider>()
            .insert(Collider::capsule_y(
                collider_dimensions.x,
                collider_dimensions.y,
            ));

        player.melt_stage = set_melt_stage.0;
        player.cast_collider =
            Collider::capsule_y(cast_collider_dimensions.x, cast_collider_dimensions.y);
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
    mut player_query: Query<&Player>,
    mut time_under_sun: ResMut<TimeUnderSun>,
    mut set_melt: EventWriter<SetMeltStageEvent>,
) {
    let player = player_query.single_mut();

    if time_under_sun.0 >= MELT_INTERVAL {
        time_under_sun.reset();

        let next_melt_stage = match player.melt_stage {
            MeltStage::None => MeltStage::Partial,
            MeltStage::Partial => MeltStage::Half,
            MeltStage::Half => MeltStage::Mostly,
            MeltStage::Mostly => MeltStage::Mostly,
        };

        set_melt.send(SetMeltStageEvent(next_melt_stage));
    }
}
