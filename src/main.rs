// Declare the modules corresponding to each task
mod task1;
mod task2;
mod task3;
mod task5;

fn main() {
    println!("Running Task 1:");
    task1::run(); // Call the `run` function from task1.rs

    println!("\nRunning Task 2:");
    task2::run(); // Call the `run` function from task2.rs

    println!("\nRunning Task 3:");
    task3::run(); // Call the `run` function from task3.rs

    println!("\nRunning Task 5:");
    task5::run(); // Call the `run` function from task5.rs
}
