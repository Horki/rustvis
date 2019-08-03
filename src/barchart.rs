extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage};
use imageproc::drawing::*;
use crate::{Rgb};
use crate::text::draw_text;
use crate::drawing::*;

/// Chart type, containing data, labels, and other metadata about a chart.
#[derive(Debug)]
pub struct Chart {
    pub title: String,
    pub color: Rgb,
    pub data: Vec<u16>,
    pub labels: Vec<String>,
    pub width: u32,
    pub height: u32
}

impl Chart {

    /// Create a new chart.
    pub fn new(title: String, color: Rgb, data: Vec<u16>, labels: Vec<String>, height: u32, width: u32) -> Chart {
        return Chart { title: title, color: color, data: data, labels: labels, width: width, height: height};
    }
}

/// Draw a horizontal barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_horizontal_barchart(img: &mut DynamicImage, barchart: &Chart) {
    draw_horizontal_bars(img, &barchart, "barchart");
}

/// Draw a vertical barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_barchart(img: &mut DynamicImage, barchart: &Chart) {

    draw_vertical_bars(img, barchart, "barchart");
}

/// Draw a histogram with a specified title, and data.
/// 
/// /// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_horizontal_histogram(img: &mut DynamicImage, barchart: &Chart) {
    draw_horizontal_bars(img, &barchart, "histogram");

}


/// Draw a vertical barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `histogram` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_histogram(img: &mut DynamicImage, histogram: &Chart) {

    draw_vertical_bars(img, histogram, "histogram");
}


/// Draw a vertical barchart, where the bars are filled with a gradient.
///
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
/// * `preset` - Preset name for the gradient. Can be: "pinkblue", "pastel_pink", "pastel_mauve", "lemongrass"
pub fn draw_vertical_gradient_barchart(img: &mut DynamicImage, barchart: &Chart, preset: &str) {
    draw_labels(img, barchart);
    let mut start_x: u32 = 20;
    let start_y: u32 = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: f32 = barchart.height as f32 - 2.0 * (barchart.height as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.width / num_bars) as f32 * 0.8) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};
    let white = Rgb { r: 255, g: 255, b: 255};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_height: f32 = max_bar_height / div as f32;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y - bar_height as u32, preset);

        start_x += bar_width + 30;    
    }    

    draw_text(img, &barchart.title, barchart.height, start_y as u32, "Lato-Regular", 50.0, &white);
}

// Draw vertical bars, either as a histogram or bar chart. 
// This is a private function, but may become public in the future.
fn draw_vertical_bars(img: &mut DynamicImage, barchart: &Chart, chart_type: &str) {

    let bar_gap = match chart_type {
        "barchart" => 30,
        "histogram" => 0,
        _ => 30
    };

    let mut start_x: u32 = 20;
    let start_y: u32 = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: f32 = barchart.height as f32 - 2.0 * (barchart.height as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    println!("num_bars {}", barchart.data.len());
    let bar_width: u32 = ((barchart.width / num_bars) as f32 * 0.8) as u32;
    
    for item in &barchart.data {
        let mut bar_height: f32 = 0.0;

        if *item > 0 as u16 {
            let div: f32 =  *max_item as f32 / *item as f32;
            bar_height = max_bar_height / div;

        }

        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x as i32, (start_y as f32 - bar_height) as i32);
        start_x += bar_width + bar_gap;    
    }

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, where the bars are filled with a gradient.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
/// * `preset` - Preset name for the gradient. Can be: "pinkblue", "pastel_pink", "pastel_mauve", "lemongrass"
pub fn draw_horizontal_gradient_barchart(img: &mut DynamicImage, barchart: &Chart, preset: &str) {

    let start_x: u32 = 20;
    let mut start_y: u32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_width: u32 = barchart.width - 2 * (barchart.width / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    let bar_gap = 30;
    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_width: f32 = max_bar_width as f32 / div;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y , preset);
        start_y += bar_height + bar_gap;
    }    

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

// Draw a horizontal chart, either as a histogram or as a barchart,
// with horizontal bars.
fn draw_horizontal_bars(img: &mut DynamicImage, barchart: &Chart, chart_type: &str) {
    let bar_gap = match chart_type {
        "barchart" => 30,
        "histogram" => 0,
        _ => 30
    };

    let start_x: i32 = 20;
    let mut start_y: i32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_width: f32 = barchart.width as f32 - 2.0 * (barchart.width as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: f32 = ((barchart.height / num_bars) as f32 * 0.8);

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_width: f32 = max_bar_width / div;
        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x, start_y);
        start_y += (bar_height) as i32 + bar_gap;    

        if chart_type == "histogram" {
            draw_line_segment_mut(img, (start_x as f32, start_y as f32), ((start_x + bar_width as i32) as f32, start_y as f32), 
            Rgba([255u8, 255u8, 255u8, 255u8]))
        }
    }    
    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, where each bar is denoted by an image.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `bar_img` - Image the bars should contain.
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_image_barchart(img: &mut DynamicImage, bar_img: &DynamicImage, barchart: &Chart) {
    draw_labels(img, barchart);
    let mut start_x: u32 = 20;
    let start_y = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: f32 = barchart.height as f32 - 2.0 * (barchart.height as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.width / num_bars) as f32 * 0.8) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_height: f32 = max_bar_height / div;

        let sampling_filter = image::FilterType::Nearest;
  
        let resized_img = image::ImageRgba8(image::imageops::resize(bar_img, bar_width as u32, bar_height as u32, sampling_filter));

        image::imageops::overlay(img, &resized_img, start_x, start_y - bar_height as u32);        

        start_x += bar_width + 30;    
    }    

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `bar_img` - Image the bars should contain.
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_horizontal_image_barchart(img: &mut DynamicImage, bar_img: &DynamicImage, barchart: &Chart) {

    let start_x: u32 = 20;
    let mut start_y: u32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_width: f32 = barchart.width as f32 - 2.0 * (barchart.width as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: f32 = ((barchart.height / num_bars) as f32 * 0.8);

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_width: f32 = max_bar_width / div;

        draw_image_as_bar(img, bar_img, bar_width as u32, bar_height as u32, start_x, start_y);
        
        start_y += bar_height as u32 + 30;
    }    
    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}


/// Draw a linechart and accentuate the points, with a specified title and data.
// pub fn draw_linechart_points(mut img: &mut DynamicImage, chart: &Chart) {
//     draw_labels(&mut img, chart);
//     let axis_len = (chart.width as f32 * 0.8);
//     let y_origin = 20.0 + axis_len;

//     let x_inc = axis_len / chart.data.len() as f32;

//     let mut start_x = 20.0;
//     let line_pixel = image::Rgba([255, 167, 90, 255]);
//     let max_item = chart.data.iter().max().unwrap();

//     let mut start_y = y_origin;
    
//     for item in &chart.data {
//         let div: f32 = *max_item as f32 / *item as f32;

//         let y_dist = y_origin - (axis_len / div);
//         draw_line_segment_mut(img, (start_x as f32, start_y as f32), (start_x + x_inc, y_dist), line_pixel);

//         // draw circle at end coordinate.
//         // let circle_radius = ;
//         // let white_pixel = ;
//         draw_filled_circle_mut(img, (start_x + x_inc, y_dist), circle_radius, white_pixel);

//         start_x += x_inc;
//         start_y = y_dist;
//     }

// }

// Draw labels onto the axes of a chart, typically for bar or line charts.
fn draw_labels(img: &mut DynamicImage, chart: &Chart) {
    draw_axes(img, chart);
    let axis_len = chart.width as f32 * 0.8;
    let x_inc = axis_len / chart.data.len() as f32 * 1.05;

    let num_bars: u32 = chart.data.len() as u32;
    let bar_width: f32 = ((chart.width / num_bars) as f32 * 0.8);

    let white = Rgb { r: 255, g: 255, b: 255};
    let mut start_x = bar_width / 2.0;

    let y_pos_label = chart.height - 30;
    
    for label in &chart.labels {
        draw_text(img, label, start_x as u32, y_pos_label as u32, "Roboto-Regular", 30.0, &white);

        start_x += bar_width;
    }    
}

// Draw x and y-axes to the image, mainly for bar charts and line charts.
fn draw_axes(img: &mut DynamicImage, chart: &Chart) {
    let line_pixel = image::Rgba([255, 167, 90, 255]);

    let axis_len = chart.width as f32 * 0.8;

    // Origin point
    let start_x = 20.0;
    let start_y: f32 = chart.height as f32 - 40.0;

    // End point on y-axis 
    let end_y_yaxis: f32 = start_y - axis_len;

    // End point on x-axis
    let end_x_xaxis: f32 = start_x + axis_len;
    
    // Draw y-axis
    draw_line_segment_mut(img, (start_x as f32, start_y as f32), (start_x, end_y_yaxis), line_pixel);

    // Draw x-axis 
    draw_line_segment_mut(img, (start_x as f32, start_y as f32), (end_x_xaxis, start_y), line_pixel);
}

// Draw an image as a bar component of a bar chart. 
// This can be used for custom bar colours or custom bar images within bar charts.
fn draw_image_as_bar(img: &mut DynamicImage, bar_img: &DynamicImage, bar_width: u32, bar_height: u32, start_x: u32, start_y: u32) {
    let sampling_filter = image::FilterType::Nearest;
    let resized_img = image::ImageRgba8(image::imageops::resize(bar_img, bar_width as u32, bar_height as u32, sampling_filter));
    image::imageops::overlay(img, &resized_img, start_x, start_y);        
}