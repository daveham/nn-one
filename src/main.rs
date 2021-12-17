extern crate hdrhistogram;
extern crate image;
extern crate nannou;
extern crate text_colorizer;

use hdrhistogram::Histogram;
use image::{DynamicImage, GenericImageView, SubImage};
use nannou::prelude::*;
use text_colorizer::*;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::NTimes {
            number_of_updates: 1,
        })
        // .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    hist: (Histogram<u32>, Histogram<u32>, Histogram<u32>),
    values: (Vec<u64>, Vec<u64>, Vec<u64>),
}

fn print_hist(label: char, hist: &Histogram<u32>, data: &[u64]) {
    for v in hist.iter_linear(16) {
        println!(
            "{}: {}",
            label,
            format!(
                "{} {}",
                v.value_iterated_to(),
                v.count_since_last_iteration()
            )
            .green()
        );
        // v.quantile(), v.quantile_iterated_to(),
        // v.count_at_value());
    }
    println!(
        "{} stats: {}",
        label,
        format!(
            "Total Count {} min {}, max {}, mean {}, stdev {}",
            hist.len(),
            hist.min(),
            hist.max(),
            hist.mean(),
            hist.stdev()
        )
        .red()
    );

    println!("{} values: {}", label, format!("{:?}", data).blue());

    // for v in hist.iter_quantiles(1) {
    //     println!("Q: {} {} {} {} {}",
    //          v.quantile(), v.quantile_iterated_to(),
    //          v.value_iterated_to(),
    //          v.count_since_last_iteration(),
    //          v.count_at_value());
    // }
}

fn build_data_from_slice(
    slice: &SubImage<&DynamicImage>,
) -> (Histogram<u32>, Histogram<u32>, Histogram<u32>) {
    let mut r = Histogram::<u32>::new_with_bounds(1, 256, 1).unwrap();
    let mut g = Histogram::<u32>::new_with_bounds(1, 256, 1).unwrap();
    let mut b = Histogram::<u32>::new_with_bounds(1, 256, 1).unwrap();

    for (_x, _y, pixel) in slice.pixels() {
        r.record(pixel[0] as u64 + 1).unwrap();
        g.record(pixel[1] as u64 + 1).unwrap();
        b.record(pixel[2] as u64 + 1).unwrap();
    }

    (r, g, b)
}

const PAD: [u64; 16] = [0; 16];

fn get_values_from_hist(hist: &Histogram<u32>) -> Vec<u64> {
    let mut v: Vec<_> = hist
        .iter_linear(16)
        .map(|v| v.count_since_last_iteration())
        .collect();
    v.reserve_exact(16 - v.len());
    assert_eq!(v.capacity() >= 16, true);
    if v.len() < 16 {
        // pad using extend method
        v.extend_from_slice(&PAD[v.len()..16]);
        // pad using loop
        // for _number in v.len()..16 {
        //     v.push(0);
        // }
    }
    v
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("BrightAndEarly.tif");
    let img = image::open(img_path).unwrap();

    println!(
        "{} {:?}",
        "image dimensions: ".cyan().bold(),
        img.dimensions()
    );
    println!("{} {:?}", "image color: ".cyan().bold(), img.color());

    let slice = img.view(0, 0, 32, 32);

    println!(
        "{} {:?}",
        "slice dimensions: ".cyan().bold(),
        slice.dimensions()
    );

    let hist = build_data_from_slice(&slice);
    println!("slice after call to build is {:?}", slice.dimensions());
    let values = (
        get_values_from_hist(&hist.0),
        get_values_from_hist(&hist.1),
        get_values_from_hist(&hist.2),
    );

    Model { hist, values }
}

// fn update(app: &App, _model: &mut Model, _update: Update) {
//     app.set_loop_mode(LoopMode::loop_once());
// }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLUE);

    draw.ellipse()
        .x_y(0.0, 0.0)
        .w_h(200.0, 200.0)
        .color(STEELBLUE);

    let r = Rect::from_w_h(100.0f32, 100.0f32);

    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .z_degrees(45.0)
        .color(PLUM);

    let r2 = r.below(r).shift_y(-50.0);

    draw.ellipse().xy(r2.xy()).wh(r2.wh()).color(SALMON);

    draw.to_frame(app, &frame).unwrap();

    print_hist('R', &model.hist.0, &model.values.0);
    print_hist('G', &model.hist.1, &model.values.1);
    print_hist('B', &model.hist.2, &model.values.2);
}
