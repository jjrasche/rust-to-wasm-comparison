use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}


fn main() {
    let start_time = std::time::Instant::now();
    let result = factorial(10000); // Calculate factorial for a large number
    let end_time = std::time::Instant::now();
    let execution_time = end_time.duration_since(start_time);
    println!("Factorial: {}", result);
    println!("Execution time (Rust): {:?}", execution_time);
}