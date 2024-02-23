use bevy::math::Vec3;
use bevy::ecs::event::Event;

#[derive(Event)]
pub struct MoveEvent {
    pub origin: Option<Vec3>,
    pub destination: Option<Vec3>,
}

#[derive(Event)]
pub struct MoveLegal {
    pub legal_move: bool,
    pub destination: Option<Vec3>,
}

#[derive(Event)]
pub struct EdgeDetectionEvent {}