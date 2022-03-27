use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use serde_json::json;
use crate::api::devices::model::Device;
use crate::di::provider::Container;
use crate::api;
use crate::api::devices;

#[get("/device")]
async fn get_all(container: web::Data<Container>) -> impl Responder {
    format!("I am {}", "mongodb")
}

#[get("/device/{id}")]
async fn get_by_id(container: web::Data<Container>) -> impl Responder {
    format!("I am {}", "mongodb")
}

#[post("/device")]
async fn create(container: web::Data<Container>, json: web::Json<Device>) -> HttpResponse {
    // let result = devices::create_device_usecase::execute();
    let mut device = json.into_inner();
    println!("form: {}", device.name);
    let result = container.device_ds.create(&mut device);
    match result {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::BadRequest().json(json!({"message": e}))
    }
}

#[put("/device/{id}")]
async fn update_by_id(container: web::Data<Container>) -> impl Responder {
    format!("I am {}", "mongodb")
}

#[delete("/device/{id}")]
async fn delete_by_id(container: web::Data<Container>) -> impl Responder {
    format!("I am {}", "mongodb")
}

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(create);
    cfg.service(update_by_id);
    cfg.service(delete_by_id);
}