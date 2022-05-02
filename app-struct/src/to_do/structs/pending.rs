use super::traits::create::Create;
use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::base::Base;

pub struct Pending {
    pub base: Base
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base::new(title, "pending");

        Pending {
            base,
        }
    }
}

impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
impl Delete for Pending {}
