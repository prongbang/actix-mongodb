use std::str::FromStr;
use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use crate::api::devices::model::{Device, Params};
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
async fn create(json: web::Json<Device>, container: web::Data<Container>) -> HttpResponse {
    let device = json.into_inner();

    let result = devices::create_device_usecase::execute(&device, &container.device_repo);
    match result {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::BadRequest().json(json!({"message": e}))
    }
}

#[put("/device/{id}")]
async fn update_by_id(path: web::Path<Params>, json: web::Json<Device>, container: web::Data<Container>) -> impl Responder {
    let params = path.into_inner();
    let mut device = json.into_inner();
    device.id = Some(ObjectId::from_str(params.id.as_str()).unwrap());

    let result = devices::update_by_id_usecase::execute(&device, &container.device_repo);
    match result {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::BadRequest().json(json!({"message": e}))
    }
}

#[delete("/device/{id}")]
async fn delete_by_id(path: web::Path<Params>, container: web::Data<Container>) -> impl Responder {
    let params = path.into_inner();
    let result = devices::delete_by_id_usecase::execute(params.id, &container.device_repo);
    if result {
        HttpResponse::Ok().json(json!({"message": "success"}))
    } else {
        HttpResponse::BadRequest().json(json!({"message": "cannot delete device by id"}))
    }
}

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(create);
    cfg.service(update_by_id);
    cfg.service(delete_by_id);
}