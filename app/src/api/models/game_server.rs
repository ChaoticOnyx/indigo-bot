use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServerId(pub String);

#[derive(Debug, Clone)]
pub struct ByondServer {
    pub id: GameServerId,
}

#[derive(Debug, Clone)]
pub struct SS14Server {
    pub id: GameServerId,
}

#[derive(Debug, Clone)]
pub enum AnyGameServer {
    Byond(ByondServer),
    SS14(SS14Server),
}
