use skia_safe::{Canvas, Color, Paint};

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

pub struct Palette;

impl Palette {
    /// A yellow beige, usually used for background.
    pub const BEIGE: Color = Color::new(0xfffceccb);

    /// A red, for details.
    pub const RED: Color = Color::RED;

    /// Black as night (probably blacker).
    pub const BLACK: Color = Color::BLACK;

    /// Black -1.
    pub const DARK_GRAY: Color = Color::new(0xff191919);

    /// Black -2.
    pub const LIGHT_GRAY: Color = Color::new(0xff333333);

    /// White, like javelized snow.
    pub const WHITE: Color = Color::WHITE;
}
