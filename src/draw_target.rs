use embedded_graphics::{
    draw_target::DrawTarget, geometry::Dimensions, iterator::ContiguousIteratorExt,
    pixelcolor::BinaryColor, primitives::Rectangle, Pixel,
};

pub struct MultiMonoFontDrawTarget<'a, T, C> {
    parent: &'a mut T,
    text_color: C,
    background_color: Option<C>,
}

impl<'a, T: DrawTarget, C> MultiMonoFontDrawTarget<'a, T, C> {
    pub fn new(parent: &'a mut T, text_color: C, background_color: Option<C>) -> Self {
        Self {
            parent,
            text_color,
            background_color,
        }
    }
}

impl<T: DrawTarget> DrawTarget for MultiMonoFontDrawTarget<'_, T, T::Color> {
    type Color = BinaryColor;
    type Error = T::Error;

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.parent.draw_iter(
            colors
                .into_iter()
                .into_pixels(area)
                .filter(|Pixel(_, color)| color.is_on() || self.background_color.is_some())
                .map(|Pixel(pos, pixel_color)| {
                    let color = if pixel_color.is_on() {
                        self.text_color
                    } else {
                        if let Some(background_color) = self.background_color {
                            background_color
                        } else {
                            self.text_color
                        }
                    };
                    Pixel(pos, color)
                }),
        )
    }

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        unreachable!()
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        match color {
            BinaryColor::On => self.parent.fill_solid(area, self.text_color),
            BinaryColor::Off => {
                if let Some(background_color) = self.background_color {
                    self.parent.fill_solid(area, background_color)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn clear(&mut self, _color: Self::Color) -> Result<(), Self::Error> {
        unreachable!()
    }
}

impl<T: DrawTarget, C> Dimensions for MultiMonoFontDrawTarget<'_, T, C> {
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box()
    }
}
