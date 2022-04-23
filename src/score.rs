use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Bgr565,
    prelude::*,
    text::Text,
};
use push2_display::DISPLAY_WIDTH;

pub struct Score {
    score_1: u32,
    score_2: u32,
    best_of: u32,
}

impl Score {
    pub fn new(best_of: u32) -> Score {
        Score {
            score_1: 0,
            score_2: 0,
            best_of,
        }
    }

    pub fn winner(&self) -> u32 {
        if self.score_1 >= self.best_of {
            1
        } else if self.score_2 >= self.best_of {
            2
        } else {
            0
        }
    }

    pub fn player(&mut self, player: u32) {
        match player {
            1 => self.score_1 += 1,
            2 => self.score_2 += 1,
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.score_1 = 0;
        self.score_2 = 0;
    }
}

impl Drawable for Score {
    type Color = Bgr565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Bgr565>,
    {
        let text_style = MonoTextStyle::new(&FONT_6X10, Bgr565::WHITE);
        let text = &format!("{:02} - {:02}", self.score_1, self.score_2);
        Text::new(text, Point::new(DISPLAY_WIDTH as i32 / 2, 9), text_style)
            .draw(display)?;

        Ok(())
    }
}
