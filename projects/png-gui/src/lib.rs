use eta::{Eta, TimeAcc};

pub use self::{errors::TinyResult, workspace::TinyWorkspace};

mod cli;
mod errors;
pub mod utils;
mod workspace;

fn calculate_square(number: usize) -> usize {
    number * number
}

#[test]
fn test() {
    let count = 100;
    let numbers = Vec::from_iter(0..count);
    let mut eta = Eta::new(count, TimeAcc::MILLI);

    for number in numbers {
        calculate_square(number);
        eta.step();
        if (number % 10) == 0 {
            println!("{}", eta);
        }
    }
}
