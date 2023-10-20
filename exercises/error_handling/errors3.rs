// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input).unwrap();
    // let cost = total_cost(pretend_user_input)?;
    // 这边使用?的话，就不需要使用unwrap了，因为?会自动处理Err的情况，
    // 但是这边的total_cost返回的是Result<i32, ParseIntError>，所以需要在main函数中使用unwrap
    // 如果返回的是Result<i32, &str>，则可以在total_cost中使用?，在main中使用unwrap

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;
    // ?表示如果是Err，则直接返回Err中的值，如果是Ok，则返回Ok中的值
    Ok(qty * cost_per_item + processing_fee)
}
