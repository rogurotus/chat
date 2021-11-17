use super::data::Id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnterRoomReq {
    pub user: Id,
    pub room: Id,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageReq {
    pub user: Id,
    pub room: Id,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessageReq {
    pub user: Id,
    pub room: Id,
}
