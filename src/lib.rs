#[macro_use]
extern crate pest_derive;

mod parser;
mod operation;
mod evaluator;

pub use evaluator::evaluate;
