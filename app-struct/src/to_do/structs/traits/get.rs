use serde_json::{Map, value::Value};
use crate::ItemTypes::Pending;

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item = state.get(title);

        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result)
            },
            None => println!("Item: {} was not found", title)
        }
    }
}