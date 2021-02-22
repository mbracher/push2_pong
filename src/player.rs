use embedded_graphics::{pixelcolor::Bgr565, prelude::*, primitives::Rectangle};

use embedded_graphics::primitives::PrimitiveStyle;

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub position: Point,
    pub size: Size,
}

impl Player {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Player {
        Player {
            position: Point::new(x, y),
            size: Size::new(w, h),
        }
    }
}

impl Drawable for Player {
    type Color = Bgr565;

    fn draw<D>(&self, display: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Bgr565>,
    {
        Rectangle::new(self.position, self.size)
            .into_styled(PrimitiveStyle::with_fill(Bgr565::WHITE))
            .draw(display)
    }
}
