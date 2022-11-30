use bevy::math::Vec3;

pub struct MoveEvent {
    pub origin: Option<Vec3>,
    pub destination: Option<Vec3>,
}

pub struct MoveLegal {
    pub legal_move: bool,
    pub destination: Option<Vec3>,
}