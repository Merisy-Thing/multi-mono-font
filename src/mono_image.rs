use crate::draw_target::MultiMonoFontDrawTarget;
use embedded_graphics::{
    geometry::OriginDimensions,
    image::{ImageDrawable, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, PixelColor, Size},
    primitives::Rectangle,
};

/// A static binary image drawable.
///
/// The image is drawn with the foreground color where the pixel is set to `BinaryColor::On`,
/// and with the background color where the pixel is set to `BinaryColor::Off`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct MonoImage<'a, C> {
    image: &'a ImageRaw<'a, BinaryColor>,
    foreground_color: C,
    background_color: Option<C>,
}

impl<'a, C> MonoImage<'a, C> {
    /// Creates a text drawable with the default text style.
    pub const fn new(image: &'a ImageRaw<'a, BinaryColor>, foreground_color: C) -> Self {
        Self {
            image,
            foreground_color,
            background_color: None,
        }
    }

    /// Sets the background color.
    pub fn with_background_color(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color);
        self
    }
}

impl<C> OriginDimensions for MonoImage<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn size(&self) -> Size {
        self.image.size()
    }
}

impl<'a, C> ImageDrawable for MonoImage<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let mut bin_target =
            MultiMonoFontDrawTarget::new(target, self.foreground_color, self.background_color);

        self.image.draw(&mut bin_target)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut bin_target =
            MultiMonoFontDrawTarget::new(target, self.foreground_color, self.background_color);

        self.image.draw_sub_image(&mut bin_target, area)
    }
}
