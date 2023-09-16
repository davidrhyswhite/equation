#[macro_use]
extern crate pest_derive;

mod evaluator;
mod operation;
mod parser;

pub use evaluator::evaluate;
