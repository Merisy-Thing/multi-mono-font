use crate::draw_target::MultiMonoFontDrawTarget;
use embedded_graphics::{
    Drawable,
    geometry::{OriginDimensions, Point},
    image::{Image, ImageDrawable, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, PixelColor, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
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
    /// Creates a text drawable with foreground color.
    pub const fn new(image: &'a ImageRaw<'a, BinaryColor>, foreground_color: C) -> Self {
        Self {
            image,
            foreground_color,
            background_color: None,
        }
    }

    /// Creates a text drawable with foreground color and background color.
    pub const fn new_with_background_color(
        image: &'a ImageRaw<'a, BinaryColor>,
        foreground_color: C,
        background_color: C,
    ) -> Self {
        Self {
            image,
            foreground_color,
            background_color: Some(background_color),
        }
    }

    /// Sets the background color.
    pub fn with_background_color(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color);
        self
    }

    /// Clears the background color.
    pub fn clear_background_color(&mut self) {
        self.background_color = None;
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

/// A static binary run-length encoded font drawable.
///
/// The image is drawn with the foreground color where the pixel is set to `BinaryColor::On`,
/// and with the background color where the pixel is set to `BinaryColor::Off`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct MonoRleImage<C> {
    glyph: &'static [u8],
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    foreground_color: C,
    background_color: Option<C>,
}

impl<C> MonoRleImage<C> {
    /// Creates a text drawable with foreground color.
    pub const fn new(glyph: &'static [u8], width: u16, height: u16, foreground_color: C) -> Self {
        Self {
            glyph,
            x: 0,
            y: 0,
            width,
            height,
            foreground_color,
            background_color: None,
        }
    }

    /// Creates a text drawable with foreground color and background color.
    pub const fn new_with_background_color(
        glyph: &'static [u8],
        width: u16,
        height: u16,
        foreground_color: C,
        background_color: C,
    ) -> Self {
        Self {
            glyph,
            x: 0,
            y: 0,
            width,
            height,
            foreground_color,
            background_color: Some(background_color),
        }
    }

    pub const fn with_offset(mut self, x: i16, y: i16) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the background color.
    pub fn with_background_color(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color);
        self
    }

    /// Clears the background color.
    pub fn clear_background_color(&mut self) {
        self.background_color = None;
    }

    pub fn rectangle(&self) -> Rectangle {
        Rectangle::new(
            Point::new(self.x as i32, self.y as i32),
            Size::new(self.width as u32, self.height as u32),
        )
    }
}

impl<C> OriginDimensions for MonoRleImage<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl<C> ImageDrawable for MonoRleImage<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let mut target =
            MultiMonoFontDrawTarget::new(target, self.foreground_color, self.background_color);

        let glyph_rder = crate::glyph_reader::GlyphReader::new(self.glyph);
        crate::glyph_reader::render_glyph_as_box_fill(&self.rectangle(), glyph_rder, &mut target)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut target =
            MultiMonoFontDrawTarget::new(target, self.foreground_color, self.background_color);

        let glyph_rder = crate::glyph_reader::GlyphReader::new(self.glyph);
        crate::glyph_reader::render_glyph_as_box_fill(area, glyph_rder, &mut target)
    }
}

/// A stack of images.
///
/// The images are drawn in the stack in the order they are in the vector.
pub struct MonoImageStack<'a, T: ImageDrawable> {
    images: &'a [&'a T],
    position: Point,
    size: Size,
    background_color: Option<T::Color>,
}

impl<'a, T: ImageDrawable> MonoImageStack<'a, T> {
    pub const fn new(images: &'a [&'a T], position: Point) -> Self {
        Self {
            images,
            position,
            size: Size::zero(),
            background_color: None,
        }
    }

    pub const fn new_with_background_color(
        images: &'a [&'a T],
        area: Rectangle,
        background_color: T::Color,
    ) -> Self {
        Self {
            images,
            position: area.top_left,
            size: area.size,
            background_color: Some(background_color),
        }
    }

    pub fn draw<D: DrawTarget<Color = T::Color>>(&self, target: &mut D) -> Result<(), D::Error> {
        if let Some(bg_color) = self.background_color {
            let area = Rectangle::new(self.position, self.size);
            area.into_styled(PrimitiveStyle::with_fill(bg_color))
                .draw(target)?;
        }
        for image in self.images {
            Image::new(*image, self.position).draw(target)?
        }

        Ok(())
    }
}
