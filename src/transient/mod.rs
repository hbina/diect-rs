use actix_web::web;

pub mod error;
mod form;
mod routes;
pub mod transient_dictionary;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_transient_value);
    cfg.service(routes::submit_transient_value);
    cfg.service(routes::cleanup_transient_storage);
}
