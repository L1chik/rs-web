mod to_do;
mod state;
mod processes;
mod views;

use std::env;
use actix_web::{App, HttpServer};
use to_do::{ItemTypes, to_do_factory, structs::traits::create::Create};
use state::{write_to_file, read_file};
use serde_json::{Map, json, value::Value};
use processes::process_input;


// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let command = &args[1];
//     let title = &args[2];
//
//     let state = read_file("./state.json");
//     let status: String;
//
//     match &state.get(title) {
//         Some(result) => { status = result.to_string().replace('\"', ""); }
//         None => { status = String::from("pending") }
//     }
//
//     let item = to_do_factory(&status, title)
//         .expect(&status);
//
//     process_input(item, command.to_string(), &state);
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);

        app
    })
        .bind("127.0.0.1:8999")?
        .run()
        .await
}