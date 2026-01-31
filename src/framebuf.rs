use core::ops::Sub;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;

pub trait BulkFlushTarget {
    type Color: PixelColor;
    type Error;

    /// Flushes the buffer to the target.
    fn flush(&mut self, area: &Rectangle, data: &[Self::Color]) -> Result<(), Self::Error>;
}

pub struct Framebuffer<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL> + BulkFlushTarget> {
    target: &'a mut DRAW,
    fb: FramebufferInner<'a, COL>,
}

impl<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL> + BulkFlushTarget>
    Framebuffer<'a, COL, DRAW>
{
    /// Creates a new [Framebuffer] instance based on the provided [DrawTarget].
    pub fn new(target: &'a mut DRAW, buffer: &'a mut [COL]) -> Self {
        Self {
            target,
            fb: FramebufferInner::new(buffer),
        }
    }
}

impl<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL> + BulkFlushTarget<Color = COL>> Dimensions
    for Framebuffer<'a, COL, DRAW>
{
    fn bounding_box(&self) -> Rectangle {
        self.target.bounding_box()
    }
}

impl<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL> + BulkFlushTarget<Color = COL>> DrawTarget
    for Framebuffer<'a, COL, DRAW>
{
    type Color = COL;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.target.draw_iter(pixels).map_err(|_| ())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        if self.fb.start_drawing(area) {
            self.fb.fill_contiguous(area, colors)?;
            self.target
                .flush(area, self.fb.get_buffer())
                .map_err(|_| ())?;
            self.fb.finalize();
            Ok(())
        } else {
            self.target.fill_contiguous(area, colors).map_err(|_| ())
        }
    }
}

struct FramebufferInner<'a, C: PixelColor> {
    buf: &'a mut [C],
    size: Size,
    position: Point,
    len: usize,
}

impl<'a, C: PixelColor> FramebufferInner<'a, C> {
    pub fn new(buf: &'a mut [C]) -> Self {
        Self {
            buf,
            size: Size::zero(),
            position: Point::zero(),
            len: 0,
        }
    }

    pub fn start_drawing(&mut self, area: &Rectangle) -> bool {
        if self.len > 0 {
            return false;
        }

        let size = area.size;
        let position = area.top_left;
        let len = size.width as usize * size.height as usize;

        if len <= self.buf.len() {
            self.size = size;
            self.position = position;
            self.len = len;
            true
        } else {
            false
        }
    }

    pub fn get_buffer(&mut self) -> &[C] {
        &self.buf[0..self.len]
    }

    pub fn finalize(&mut self) {
        self.len = 0;
    }
}

impl<C: PixelColor> Dimensions for FramebufferInner<'_, C> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.position, self.size)
    }
}

impl<C: PixelColor> DrawTarget for FramebufferInner<'_, C> {
    type Color = C;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let pt = pixel.0.sub(self.position);
            let pos = pt.y * self.size.width as i32 + pt.x;
            if pos < 0 || pos >= self.len as i32 {
                continue;
            }
            self.buf[pos as usize] = pixel.1;
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let drawable_area = area.intersection(&self.bounding_box());
        if drawable_area.is_zero_sized() {
            return Ok(());
        }

        let top_skip = drawable_area.top_left.y - area.top_left.y;
        let left_skip = drawable_area.top_left.x - area.top_left.x;
        let right_skip = area.size.width as i32 - (left_skip + drawable_area.size.width as i32);
        // let bottom_skip = area.size.height as i32 - (top_skip + drawable_area.size.height as i32);

        let mut color_iter = colors.into_iter();

        // skip all un-drawable rows
        for _ in 0..top_skip {
            for _ in 0..area.size.width as usize {
                color_iter.next();
            }
        }
        for y in drawable_area.top_left.y as usize
            ..drawable_area.top_left.y as usize + drawable_area.size.height as usize
        {
            for _ in 0..left_skip {
                color_iter.next();
                // skip all left
            }
            for x in drawable_area.top_left.x as usize
                ..drawable_area.top_left.x as usize + drawable_area.size.width as usize
            {
                let pos = (y as i32 - self.position.y) as usize * self.size.width as usize
                    + (x as i32 - self.position.x) as usize;
                match color_iter.next() {
                    Some(color) => self.buf[pos] = color,
                    None => return Ok(()),
                }
            }
            for _ in 0..right_skip {
                color_iter.next();
                // skip all right
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // Clamp area to drawable part of the display target
        let drawable_area = area.intersection(&self.bounding_box());

        // Draw the rectangle
        for y in drawable_area.top_left.y as usize
            ..drawable_area.top_left.y as usize + drawable_area.size.height as usize
        {
            for x in drawable_area.top_left.x as usize
                ..drawable_area.top_left.x as usize + drawable_area.size.width as usize
            {
                let pos = (y as i32 - self.position.y) as usize * self.size.width as usize
                    + (x as i32 - self.position.x) as usize;
                self.buf[pos] = color;
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buf[0..(self.size.width * self.size.height) as usize].fill(color);
        Ok(())
    }
}

impl<C: PixelColor> Drawable for FramebufferInner<'_, C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.fill_contiguous(
            &Rectangle::new(self.position, self.size),
            self.buf.iter().cloned(),
        )
    }
}
