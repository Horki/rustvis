extern crate rustvis;
extern crate time;
use rustvis::{Rgb, barchart, new_with_background};
use time::PreciseTime;
use rustvis::barchart::{Chart, draw_vertical_gradient_barchart};

fn main() {
    let start = PreciseTime::now();

    let white = Rgb { r: 255, g: 255, b: 255};
    let black = Rgb { r: 0, g: 0, b: 0};
    let mut img = rustvis::new_with_background(1500, 1500, &black);

    // Insert the data into a vec
    let data: Vec<u16> = vec![5, 10, 20, 40];

    // Create labels for the barchart
    let labels: Vec<String> = vec!["one".to_string(), "c_spec".to_string(), "d_spec".to_string(), "e_spec".to_string()];
    
    // Barchart bar color
    let blue = Rgb { r: 40, g: 50, b: 200};

    // Create a barchart struct
    let barchart = Chart::new("Earnings for 2019/2020".to_string(), blue, data, labels, 1500, 1500);

    draw_vertical_gradient_barchart(&mut img, &barchart, "lemongrass");
    rustvis::save_image(img, "barchart.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image in examples/example_output");
}