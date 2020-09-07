use actix_web::web;

pub mod form;
pub mod persistent_value;
pub mod routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_persistent_value);
    cfg.service(routes::submit_persistent_value);
    cfg.service(routes::cleanup_persistent_storage);
}
