use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Baseline},
    transform::Transform,
    Drawable,
};

/// StaticText drawable.
///
/// A text drawable can be used to draw text to a draw target.
///
/// See the [module-level documentation](super) for more information about text drawables and examples.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StaticText<'a, S> {
    /// The string.
    pub text: &'a str,

    /// The position.
    pub rectangle: Rectangle,

    /// The character style.
    pub character_style: S,

    /// Horizontal text alignment.
    pub alignment: Alignment,

    /// Text baseline.
    pub baseline: Baseline,
}

impl<'a, S> StaticText<'a, S> {
    /// Creates a text drawable with the default text style.
    pub const fn new(text: &'a str, rectangle: Rectangle, character_style: S) -> Self {
        Self {
            text,
            rectangle,
            character_style,
            alignment: Alignment::Left,
            baseline: Baseline::Alphabetic,
        }
    }

    /// Creates a text drawable with the given alignment and baseline.
    pub const fn with_style(
        text: &'a str,
        rectangle: Rectangle,
        character_style: S,
        alignment: Alignment,
        baseline: Baseline,
    ) -> Self {
        Self {
            text,
            rectangle,
            character_style,
            alignment,
            baseline,
        }
    }
}

impl<S: Clone> Transform for StaticText<'_, S> {
    fn translate(&self, by: Point) -> Self {
        Self {
            rectangle: Rectangle::new(self.rectangle.top_left + by, self.rectangle.size),
            ..self.clone()
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.rectangle.top_left += by;

        self
    }
}

impl<S: TextRenderer> StaticText<'_, S> {
    fn lines(&self) -> impl Iterator<Item = (&str, Point)> {
        let line_feed = self.text.matches('\n').count() as i32;

        let offset_y = self.character_style.line_height() as i32 * line_feed;
        let mut position = self.rectangle.top_left;
        let height = self.rectangle.size.height as i32;
        match self.baseline {
            Baseline::Top => {}
            Baseline::Bottom | Baseline::Alphabetic => position.y += height - 1 - offset_y,
            Baseline::Middle => position.y += (height - 1 - offset_y) / 2,
        }

        self.text.split('\n').map(move |line| {
            let p = match self.alignment {
                Alignment::Left => position,
                Alignment::Right => {
                    let metrics =
                        self.character_style
                            .measure_string(line, Point::zero(), self.baseline);
                    position + Point::new(self.rectangle.size.width as i32, 0)
                        - (metrics.next_position - Point::new(1, 0))
                }
                Alignment::Center => {
                    let metrics =
                        self.character_style
                            .measure_string(line, Point::zero(), self.baseline);
                    position + Point::new(self.rectangle.size.width as i32 / 2, 0)
                        - (metrics.next_position - Point::new(1, 0)) / 2
                }
            };

            position.y += self.character_style.line_height() as i32;

            // remove trailing '\r' for '\r\n' line endings
            let len = line.len();
            if len > 0 && line.as_bytes()[len - 1] == b'\r' {
                (&line[0..len - 1], p)
            } else {
                (line, p)
            }
        })
    }
}

impl<S: TextRenderer> Drawable for StaticText<'_, S> {
    type Color = S::Color;
    type Output = Point;

    fn draw<D>(&self, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut next_position = Point::zero();
        let size = &self.rectangle.size;
        let left_x = self.rectangle.top_left.x;
        let right_x = left_x + size.width as i32;

        for (line, position) in self.lines() {
            if position.x > left_x {
                self.character_style.draw_whitespace(
                    (position.x - left_x) as u32,
                    Point::new(left_x, position.y),
                    self.baseline,
                    target,
                )?;
            }

            next_position =
                self.character_style
                    .draw_string(line, position, self.baseline, target)?;

            if next_position.x < right_x {
                self.character_style.draw_whitespace(
                    (right_x - next_position.x) as u32,
                    next_position,
                    self.baseline,
                    target,
                )?;
                next_position.x = right_x
            }
        }

        Ok(next_position)
    }
}
