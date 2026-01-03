use bevy::math::Vec3;
//use bevy::ecs::event::Event;
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct MoveEvent {
    pub origin: Option<Vec3>,
    pub destination: Option<Vec3>,
}

#[derive(Message)]
pub struct MoveLegal {
    pub legal_move: bool,
    pub destination: Option<Vec3>,
}

#[derive(Message)]
pub struct EdgeDetectionEvent {}