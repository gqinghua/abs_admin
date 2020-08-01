use actix_web::{HttpResponse, Responder, web};
use rbatis::plugin::page::{Page};
use crate::dao::RB;
use crate::service::SYS_RES_SERVICE;
use crate::service::CACHE_SERVICE;
use crate::domain::dto::ResPageDTO;
use crate::domain::vo::RespVO;

/// 资源分页(json请求)
pub async fn res_page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.page(&page.0).await;
    RespVO::from_result(&data).to_json_resp()
    // CACHE_SERVICE.put_json("res_page", &data.as_ref().unwrap().to_string()).await;
    // let cached_res: String = CACHE_SERVICE.get_json("res_page").await.unwrap();
}