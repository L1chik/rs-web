use serde_json::{Map, json, Value};
use crate::{Create, ItemTypes};
use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;
use crate::to_do::structs::traits::delete::Delete;
use crate::to_do::structs::traits::edit::Edit;
use crate::to_do::structs::traits::get::Get;

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.base.title, &state),
        "create" => item.create(&item.base.title, &item.base.status, &mut state),
        "delete" => item.delete(&item.base.title, &mut state),
        "edit" => item.set_to_done(&item.base.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.base.title, &state),
        "delete" => item.delete(&item.base.title, &mut state),
        "edit" => item.set_to_pending(&item.base.title, &mut state),
        _ => println!("command: <{}> not supported", command),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}