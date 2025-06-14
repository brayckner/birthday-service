use actix_web::get;
use crate::service::hello_service::hello_message;

#[get("/")]
async fn hello_world() -> &'static str {
    hello_message()
}