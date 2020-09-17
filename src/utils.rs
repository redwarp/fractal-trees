use skia_safe::{Canvas, Paint};

pub trait Bounded {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
}

impl Bounded for Canvas {
    fn width(&self) -> f32 {
        self.base_layer_size().width as f32
    }

    fn height(&self) -> f32 {
        self.base_layer_size().height as f32
    }
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas);

    #[allow(unused_variables)]
    fn draw_with_paint(&self, canvas: &mut Canvas, paint: &mut Paint) {
        self.draw(canvas);
    }
}
