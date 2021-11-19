extern crate lazy_static;

pub mod models;

use actix_files::NamedFile;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use models::data::*;
use models::request::*;

use std::time::SystemTime;

#[get("/User")]
async fn init_user() -> impl Responder {
    HttpResponse::Ok().body(User::new().to_string())
}

#[post("/CreateRoom")]
async fn create_room() -> impl Responder {
    HttpResponse::Ok().body(Room::new().to_string())
}

#[post("/EnterRoom")]
async fn enter_room(req: web::Json<EnterRoomReq>) -> impl Responder {
    let mut lock = ROOMS.lock().unwrap();
    if let Some(r) = lock.get_mut(&req.old_room)
    {
        r.rm_user(req.user);
    }

    if let Some(r) = lock.get_mut(&req.room)
    {
        r.add_user(req.user);
        HttpResponse::Ok().body("")
    }
    else 
    {
        return HttpResponse::NotFound().body("not found room")
    }
    
}

#[post("/SendMessage")]
async fn send_message(req: web::Json<SendMessageReq>) -> impl Responder {
    let req = req.into_inner();
    let msg = Message::new(req.user, req.data);
    ROOMS.lock().unwrap().get_mut(&req.room).map(|r| {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        r.messages.push((msg, time));
    });
    HttpResponse::Ok().body("Good")
}

#[post("/Messages")]
async fn get_messages(req: web::Json<GetMessageReq>) -> impl Responder {
    let mut rooms = ROOMS.lock().unwrap();
    let r = match rooms.get_mut(&req.room) {
        Some(a) => a,
        None => return HttpResponse::NotFound().body("not found room"),
    };

    let time_user = match r.users.get(&req.user) {
        Some(a) => a,
        None => return HttpResponse::NotFound().body("not found user"),
    };

    let new_time_user = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let msgs = r
        .messages
        .iter()
        .filter(|(_, t)| t > time_user)
        .map(|(msg, _)| msg.clone())
        .collect::<Vec<Message>>();

    r.users.get_mut(&req.user).map(|u| *u = new_time_user);
    HttpResponse::Ok().body(serde_json::to_string(&msgs).unwrap())
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("front/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(init_user)
            .service(create_room)
            .service(enter_room)
            .service(create_room)
            .service(send_message)
            .service(get_messages)
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
