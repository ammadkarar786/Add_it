use actix_web::{web, Result};
use actix_web::{App, HttpServer};
use serde::Deserialize;




#[derive(Deserialize)]
struct Add10 {
    num: u32,
    
}

async fn add_it(info: web::Path<Add10>) -> Result<String> {
    Ok(format!("welcome you are using Add_it function your answer is {}", info.num +10 ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
 
    HttpServer::new(|| {
        App::new().route(
            "/addit/{num}", // <- define path parameters
            web::get().to(add_it),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}