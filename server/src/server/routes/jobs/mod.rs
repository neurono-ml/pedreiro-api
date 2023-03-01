mod watch;
mod create;

use actix_web::{Scope, web};

use self::create::create_job;
use self::watch::watch_job;

pub fn make_job_scope() -> Scope {
    web::scope("/jobs")
        .service(web::resource("/")
            .route(web::post().to(create_job)))
        .service(web::resource("/watch/{job_name}")
            .route(web::get().to(watch_job)))
}
