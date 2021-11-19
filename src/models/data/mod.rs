#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::*;
use std::sync::Mutex;



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
    pub static ref ROOMS: Mutex<HashMap<Id, Room>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<Id, User>> = Mutex::new(HashMap::new());
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Id,
}
impl User {
    pub fn new() -> Id {
        let id = get_user_id();
        USERS.lock().unwrap().insert(id, User { id });
        id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
}

impl Room {
    pub fn new() -> Id {
        let id = get_room_id();
        ROOMS.lock().unwrap().insert(
            id,
            Room {
                id: id,
                users: HashMap::new(),
                messages: vec![],
            },
        );
        id
    }

    pub fn add_user(&mut self, user_id: Id) {
        self.users.insert(user_id, 0);
    }
    pub fn rm_user(&mut self, user_id: Id) {
        self.users.remove(&user_id);
    }
}
