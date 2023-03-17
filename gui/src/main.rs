mod bg_image;

use crate::bg_image::BgImage;
use eframe::{
    egui::{CentralPanel, Frame, Sense},
    epaint::ColorImage,
};
use polygon_filler::{fill_polygon, measure_time, scale, Board, Polygon, Shape};

fn main() {
    let mut poly = Polygon {
        vertices: vec![[30., 5.], [10., 20.], [15., 30.], [50., 25.]],
    };

    scale(&mut poly, 512. / 64.);

    let shape = (512, 512);
    let mut board = vec![false; shape.0 * shape.1];

    let (_, time) = measure_time(|| fill_polygon(&mut board, shape, &poly, false));
    println!("Fill triangle time: {}ms", time * 1e3);

    let app_data = AppData { board, shape };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "swarm-rs application in eframe",
        native_options,
        Box::new(move |_cc| {
            Box::new(App {
                img: BgImage::new(),
                app_data,
            })
        }),
    )
    .unwrap();
}

#[derive(Default)]
pub struct App {
    img: BgImage,
    app_data: AppData,
}

#[derive(Default)]
struct AppData {
    board: Board,
    shape: Shape,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
                self.img
                    .paint(&response, &painter, &self.app_data, |app_data| {
                        let image: Vec<_> = app_data
                            .board
                            .iter()
                            .map(|b| [if *b { 255u8 } else { 0u8 }; 3])
                            .flatten()
                            .collect();
                        ColorImage::from_rgb([app_data.shape.0, app_data.shape.1], &image)
                    })
            });
        });
    }
}
