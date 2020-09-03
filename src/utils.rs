use skia_safe::Canvas;

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
