// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


use std::f32;

fn main() {
    // const pi: f32 = 3.14f32;
    const RADIUS: f32 = 5.00f32;
    let pi = f32::consts::PI;

    let area = pi * f32::powi(RADIUS, 2);

    println!(
        "The area of a circle with RADIUS {:.2} is {:.5}!",
        RADIUS, area
    )
}
