extern crate hdrhistogram;
extern crate image;
extern crate nannou;
extern crate text_colorizer;

use hdrhistogram::Histogram;
use image::{DynamicImage, GenericImageView, SubImage};
use nannou::color::named;
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

fn render_hist(draw: &Draw, data: &[u64], bounds: &Rect, bar_color: Rgb<u8>) {
    // draw background
    draw.rect()
        .xy(bounds.xy())
        .wh(bounds.wh())
        .color(named::GAINSBORO);

    // draw border around background
    let border_width = 2.0;
    let border_rect = Rect::from_xy_wh(bounds.xy(), bounds.wh() + border_width);
    let border_corner_points: Vec<_> = border_rect
        .corners_iter()
        .map(|c: [f32; 2]| (pt2(c[0], c[1]), named::DARKGRAY))
        .collect();
    draw.polyline()
        .weight(border_width)
        .points_colored_closed(border_corner_points);

    // draw bars
    let mut bar_x_offset = 0.0f32;
    let bar_height_scale_down_by = 8.0f32;
    let bar_width = 7.0f32;
    let bar_spacing = 8.0f32;
    for v in data {
        let mut height = *v as f32 / bar_height_scale_down_by;
        // boost a non-zero value so it is clearly visible as non-zero
        if height > 0f32 {
            height = height + 1.0f32;
        }
        let bar = Rect::from_w_h(bar_width, height)
            .bottom_left_of(*bounds)
            .shift_x(bar_x_offset);
        draw.rect().xy(bar.xy()).wh(bar.wh()).color(bar_color);
        bar_x_offset = bar_x_offset + bar_spacing;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHEAT);

    let padding = 10f32;

    let win = app.window_rect().pad(padding);
    let r_bounds = Rect::from_w_h(128.0f32, 128.0f32).top_left_of(win);

    render_hist(&draw, &model.values.0, &r_bounds, RED);

    let g_bounds = r_bounds.right_of(r_bounds).shift_x(padding);

    render_hist(&draw, &model.values.1, &g_bounds, GREEN);

    let b_bounds = g_bounds.right_of(g_bounds).shift_x(padding);

    render_hist(&draw, &model.values.2, &b_bounds, BLUE);

    draw.to_frame(app, &frame).unwrap();

    print_hist('R', &model.hist.0, &model.values.0);
    print_hist('G', &model.hist.1, &model.values.1);
    print_hist('B', &model.hist.2, &model.values.2);
}
