extern crate lazy_static;

pub mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::data::*;
use models::request::*;
use std::time::Instant;

#[get("/")]
async fn init_user() -> impl Responder {
    HttpResponse::Ok().body(User::new().to_string())
}

#[post("/CreateRoom")]
async fn create_room() -> impl Responder {
    HttpResponse::Ok().body(Room::new().to_string())
}

#[post("/EnterRoom")]
async fn enter_room(req: web::Json<EnterRoomReq>) -> impl Responder {
    let res = USERS.write().unwrap().get_mut(&req.user).and_then(|u| {
        ROOMS.write().unwrap().get_mut(&req.room).map(|r| {
            r.add_user(u.id);
        })
    });

    if let None = res {
        return HttpResponse::NotFound().body("not found");
    }
    HttpResponse::Ok().body("Good")
}

#[post("/SendMessage")]
async fn send_message(req: web::Json<SendMessageReq>) -> impl Responder {
    println!("{:?}", req);
    let req = req.into_inner();
    let msg = Message::new(req.user, req.data);
    ROOMS.write().unwrap().get_mut(&req.room).map(|r| {
        let time = Instant::now().elapsed().as_secs();
        r.messages.push((msg, time));
        r.last_message_time = time;
    });
    HttpResponse::Ok().body("Good")
}

#[post("/Messages")]
async fn get_messages(req: web::Json<GetMessageReq>) -> impl Responder {

    let msgs = ROOMS
        .read()
        .and_then(|a| {
            let r = a.get(&req.room).unwrap();
            let time_user = r.users.get(&req.user).unwrap();
            Ok(r.messages
                .iter()
                .filter(|(_, t)| t > time_user)
                .map(|(msg, _)| msg.clone())
                .collect::<Vec<Message>>())
        })
        .unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&msgs).unwrap())
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
