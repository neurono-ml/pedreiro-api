mod create;

use actix_web::{Scope, web};

use create::create_job;

pub fn make_job_scope() -> Scope {
    web::scope("/jobs")
        .service(web::resource("/")
            .route(web::post().to(create_job)))
}
