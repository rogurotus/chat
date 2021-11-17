#![allow(dead_code)]

use actix_web::rt::time::Instant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::*;
use std::sync::RwLock;
use std::time::Duration;
use std::time::SystemTime;

pub type Id = usize;

static USER_ID: AtomicUsize = AtomicUsize::new(1);
fn get_user_id() -> Id {
    USER_ID.fetch_add(1, Ordering::Relaxed)
}

static ROOM_ID: AtomicUsize = AtomicUsize::new(1);
fn get_room_id() -> Id {
    ROOM_ID.fetch_add(1, Ordering::Relaxed)
}

use lazy_static::*;
lazy_static! {
    pub static ref ROOMS: RwLock<HashMap<Id, Room>> = RwLock::new(HashMap::new());
    pub static ref USERS: RwLock<HashMap<Id, User>> = RwLock::new(HashMap::new());
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Id,
}
impl User {
    pub fn new() -> Id {
        let id = get_user_id();
        USERS.write().unwrap().insert(id, User { id });
        id
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    from: Id,
    data: String,
}
impl Message {
    pub fn new<T: Into<String>>(id: Id, data: T) -> Message {
        Message {
            from: id,
            data: data.into(),
        }
    }
}

pub struct Room {
    pub id: Id,
    pub users: HashMap<Id, u64>,
    pub messages: Vec<(Message, u64)>,
    pub last_message_time: u64,
}

impl Room {
    pub fn new() -> Id {
        let id = get_room_id();
        ROOMS.write().unwrap().insert(
            id,
            Room {
                id: get_room_id(),
                users: HashMap::new(),
                messages: vec![],
                last_message_time: 0,
            },
        );
        id
    }

    pub fn add_user(&mut self, user_id: Id) {
        self.users.insert(user_id, 0);
    }
}
