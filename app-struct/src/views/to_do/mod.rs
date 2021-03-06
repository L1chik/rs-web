mod create;

use actix_web::web;
use super::path::Path;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{ prefix: String::from("/item") };
    app.route(&base_path.define(String::from(
        "/create/{title}")),
                web::get().to(create::create));
}