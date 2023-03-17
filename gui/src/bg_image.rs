use eframe::{
    egui::{Painter, Response, TextureFilter, TextureOptions},
    emath,
    epaint::{Color32, ColorImage, Pos2, Rect, TextureHandle},
};

#[derive(Default)]
pub(crate) struct BgImage {
    texture: Option<eframe::egui::TextureHandle>,
}

impl BgImage {
    pub fn new() -> Self {
        Self { texture: None }
    }

    pub fn _clear(&mut self) {
        self.texture.take();
    }

    pub fn paint<T>(
        &mut self,
        response: &Response,
        painter: &Painter,
        app_data: &T,
        img_getter: impl Fn(&T) -> ColorImage,
    ) {
        let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
            let image = img_getter(app_data);
            // Load the texture only once.
            painter.ctx().load_texture(
                "my-image",
                image,
                TextureOptions {
                    magnification: TextureFilter::Nearest,
                    minification: TextureFilter::Linear,
                },
            )
        });

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let rect = Rect {
            min: Pos2::new(0., 0.),
            max: Pos2::new(512., 512.),
        };
        const UV: Rect = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
        painter.image(
            texture.id(),
            to_screen.transform_rect(rect),
            UV,
            Color32::WHITE,
        );
    }
}
