#![allow(unused_imports)]//允许未使用的导入
#![allow(unused_variables)]//允许未使用的变量
#![allow(dead_code)]//允许未使用的代码

#[macro_use]
extern crate lazy_static;

pub mod domain;
pub mod dao;
pub mod controller;
pub mod service;
pub mod config;
pub mod util;


use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use fast_log::log::RuntimeType;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
use serde_json::json;
use crate::controller::{res, user};
use config::CONFIG;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //初始化日志
    fast_log::log::init_log(&CONFIG.log_path, &RuntimeType::Std).unwrap();
    //初始化rbatis
    dao::RB.link(&CONFIG.mysql_url).await.unwrap();
    //初始化路由，启动http服务
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/res_page", web::post().to(res::res_page))
            .route("/login",web::post().to(user::user_login))
            .route("/admin_user_add",web::post().to(user::user_add))
            .route("/admin_user_page",web::post().to(user::user_page))
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}


