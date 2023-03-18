mod bg_image;

use crate::bg_image::BgImage;
use eframe::{
    egui::{CentralPanel, Frame, Painter, Response, Sense, SidePanel, Ui},
    emath,
    epaint::{pos2, Color32, ColorImage, Pos2, Rect, Vec2},
};
use polygon_filler::{fill_naive, fill_polygon, measure_time, scale, Board, Polygon, Shape};

fn main() {
    let mut poly = Polygon {
        vertices: vec![[30., 5.], [10., 20.], [15., 30.], [30., 23.], [50., 30.]],
    };

    const IMG_SIZE: usize = 512;

    scale(&mut poly, IMG_SIZE as f64 / 64.);

    let shape = (IMG_SIZE, IMG_SIZE);
    let mut board = vec![false; shape.0 * shape.1];

    let (_, time) = measure_time(|| fill_polygon(&mut board, shape, &poly, false));
    println!("Fill triangle time: {}ms", time * 1e3);

    let app_data = AppData {
        board,
        shape,
        poly,
        mouse_pos: None,
        selected_vertex: None,
        naive: false,
    };

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

pub struct App {
    img: BgImage,
    app_data: AppData,
}

struct AppData {
    board: Board,
    shape: Shape,
    poly: Polygon,
    mouse_pos: Option<Pos2>,
    selected_vertex: Option<usize>,
    naive: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::right("side_panel").show(ctx, |ui| {
            ui.label(format!("mouse: {:?}", self.app_data.mouse_pos));
            ui.label(format!("vertex: {:?}", self.app_data.selected_vertex));
            if ui.checkbox(&mut self.app_data.naive, "Naive").changed() {
                self.img.clear();
            };
        });

        CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
                self.draw(ui, &response, &painter);
            });
        });
    }
}

const MARKER_SIZE: f32 = 5.5;

impl App {
    fn draw(&mut self, ui: &mut Ui, response: &Response, painter: &Painter) {
        struct UiResult {
            scroll_delta: f32,
            zoom_delta: f32,
            pointer: bool,
            delta: Vec2,
            interact_pos: Option<Pos2>,
            hover_pos: Option<Pos2>,
            clicked: bool,
            mouse_down: bool,
            mouse_up: bool,
        }

        let ui_result = ui.input(|input| {
            let interact_pos = input.pointer.interact_pos();

            UiResult {
                scroll_delta: input.scroll_delta[1],
                zoom_delta: if input.multi_touch().is_some() {
                    input.zoom_delta()
                } else {
                    1.
                },
                pointer: input.pointer.primary_down(),
                delta: input.pointer.delta(),
                interact_pos,
                hover_pos: input.pointer.hover_pos(),
                clicked: input.pointer.primary_released(),
                mouse_down: input.pointer.primary_pressed(),
                mouse_up: input.pointer.primary_released(),
            }
        });

        self.app_data.mouse_pos = ui_result.interact_pos;

        let mouse_pos = ui_result.hover_pos.map(|pos| {
            let from_screen = emath::RectTransform::from_to(
                response.rect,
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            );
            let pos = from_screen.transform_pos(pos);
            pos
        });

        if let Some((mouse_pos, i)) = mouse_pos.zip(self.app_data.selected_vertex) {
            self.app_data.poly.vertices[i] = [mouse_pos[0] as f64, mouse_pos[1] as f64];
            self.img.clear();
        }

        self.img
            .paint(response, painter, &mut self.app_data, |app_data| {
                app_data.board.fill(false);
                let (_, time) = measure_time(|| {
                    if app_data.naive {
                        fill_naive(&mut app_data.board, app_data.shape, &app_data.poly)
                    } else {
                        fill_polygon(&mut app_data.board, app_data.shape, &app_data.poly, false)
                    }
                });
                println!("Fill triangle time: {}ms", time * 1e3);

                let image: Vec<_> = app_data
                    .board
                    .iter()
                    .map(|b| [if *b { 255u8 } else { 0u8 }; 3])
                    .flatten()
                    .collect();
                ColorImage::from_rgb([app_data.shape.0, app_data.shape.1], &image)
            });

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        for (i, v) in self.app_data.poly.vertices.iter().enumerate() {
            let rect = Rect {
                min: Pos2::new(v[0] as f32 - MARKER_SIZE, v[1] as f32 - MARKER_SIZE),
                max: Pos2::new(v[0] as f32 + MARKER_SIZE, v[1] as f32 + MARKER_SIZE),
            };

            let vpos = to_screen.transform_pos(pos2(v[0] as f32, v[1] as f32));
            let mut hover = false;
            if let Some(mpos) = ui_result.hover_pos {
                let delta = mpos - vpos;
                if delta.length_sq() < MARKER_SIZE.powf(2.) {
                    if ui_result.mouse_down {
                        self.app_data.selected_vertex = Some(i);
                    } else {
                        hover = true;
                    }
                }
            }

            let color = if self.app_data.selected_vertex == Some(i) {
                Color32::RED
            } else if hover {
                Color32::GREEN
            } else {
                Color32::BLUE
            };

            painter.rect_stroke(to_screen.transform_rect(rect), 0., (1., color));
        }

        if ui_result.mouse_up {
            self.app_data.selected_vertex = None;
        }
    }
}
