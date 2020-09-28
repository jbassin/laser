extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod transform;

pub fn test() -> String {
    "hilo".to_owned()
}