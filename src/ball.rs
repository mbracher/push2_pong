use embedded_graphics::{
    pixelcolor::Bgr565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub position: Point,
    diameter: u32,
    pub speed_x: i32,
    pub speed_y: i32,
}

impl Ball {
    pub fn new(x: i32, y: i32, diameter: u32) -> Ball {
        Ball {
            position: Point::new(x, y),
            diameter,
            speed_x: 0,
            speed_y: 0,
        }
    }
}

impl Drawable for Ball {
    type Color = Bgr565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Bgr565>,
    {
        Circle::new(self.position, self.diameter)
            .into_styled(PrimitiveStyle::with_stroke(Bgr565::WHITE, 1))
            .draw(display)
    }
}
