use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::base::Base;

 pub struct Done {
     pub base: Base
 }

impl Done {
    pub fn new(title: &str) -> Self {
        let status = String::from("done");
        let base= Base::new(title, "done");

        Done {
            base,
        }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}