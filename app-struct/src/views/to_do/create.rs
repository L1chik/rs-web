use serde_json::{Map, value::Value};
use actix_web::HttpRequest;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");
    let title = req.match_info().get("title").unwrap();
    let title_ref = title.clone();
    let item = to_do::to_do_factory(
        "pending",
        title).expect("create");

    process_input(item, "create".to_string(), &state);

    format!("{title_ref} created")
}