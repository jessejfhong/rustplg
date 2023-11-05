use crate::routes::{health_check, home};
use crate::sayhi_middleware::SayHi;
use actix_web::dev::Server;
use actix_web::dev::Service;
use actix_web::web::{get, scope, Data};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures_util::future::FutureExt;
use std::io::Result;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

struct AppState {
    app_name: String,
}

async fn get_app_name(data: Data<AppState>) -> impl Responder {
    let name = format!("App name is {}", &data.app_name);
    HttpResponse::Ok().body(name)
}

async fn dummy() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn view_user(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(user_id)
}

pub fn run(listener: TcpListener) -> Result<Server> {
    // state, especially large state should created outside the following closure,
    // otherwise each thread will created a copy of the actual data instead of the
    // copy of the reference of the data.
    // mutable state must be crated outside the closure, otherwise it will be out
    // of sync
    let app_state = Data::new(AppState {
        app_name: String::from("lineup"),
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(app_state.clone())
            .wrap_fn(|req, srv| {
                println!("Hello from Scrat. requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hello from response.");
                    res
                })
            })
            .wrap(SayHi)
            .route("/", get().to(home))
            .route("/health_check", get().to(health_check))
            .route("/get_app_name", get().to(get_app_name))
            .service(scope("/user").route("/view/{user_id}", get().to(view_user)))
            .service(
                scope("/queue")
                    .route("/enqueue", get().to(dummy))
                    .route("/dequeue", get().to(dummy)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
