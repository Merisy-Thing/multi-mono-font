use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    image::Image,
    pixelcolor::{BinaryColor, PixelColor},
    prelude::OriginDimensions,
    primitives::Rectangle,
    text::{
        renderer::{CharacterStyle, TextMetrics, TextRenderer},
        Baseline,
    },
    Drawable,
};

use crate::{
    char_size::CharSize,
    draw_target::{Background, Both, Foreground, MonoFontDrawTarget},
    ChSzTy, MultiMonoFont,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MultiMonoLineHeight {
    Max,
    Min,
    Specify(ChSzTy),
}

const fn get_line_height<'a>(
    fonts_height: MultiMonoLineHeight,
    fonts: &'a [&'a MultiMonoFont<'a>],
) -> ChSzTy {
    let mut idx = 0;
    match fonts_height {
        MultiMonoLineHeight::Max => {
            let mut max = ChSzTy::MIN;
            while idx < fonts.len() {
                let h = fonts[idx].character_size.height as ChSzTy;
                idx += 1;
                if h > max {
                    max = h;
                }
            }
            max
        }
        MultiMonoLineHeight::Min => {
            let mut min = ChSzTy::MAX;
            while idx < fonts.len() {
                let h = fonts[idx].character_size.height as ChSzTy;
                idx += 1;
                if h < min {
                    min = h;
                }
            }
            min
        }
        MultiMonoLineHeight::Specify(h) => h,
    }
}

/// Style properties for text using a monospaced font.
///
/// A `MultiMonoTextStyle` can be applied to a [`Text`] object to define how the text is drawn.
///
/// Because `MultiMonoTextStyle` has the [`non_exhaustive`] attribute, it cannot be created using a
/// struct literal. To create a `MultiMonoTextStyle` with a given text color and transparent
/// background, use the [`new`] method. For more complex text styles, use the
/// [`MultiMonoTextStyleBuilder`].
///
/// [`Text`]: crate::text::Text
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`new`]: MultiMonoTextStyle::new()
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
#[non_exhaustive]
pub struct MultiMonoTextStyle<'a, C> {
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Font.
    pub fonts: &'a [&'a MultiMonoFont<'a>],

    ///Line height
    pub line_height: ChSzTy,
}

impl<'a, C> MultiMonoTextStyle<'a, C>
where
    C: PixelColor,
{
    /// Creates a text style with transparent background.
    pub const fn new(
        font_list: &'a [&'a MultiMonoFont<'a>],
        line_height: MultiMonoLineHeight,
        text_color: C,
    ) -> Self {
        MultiMonoTextStyleBuilder::new()
            .font(font_list, line_height)
            .text_color(text_color)
            .build()
    }

    /// Returns `true` if the style is transparent.
    ///
    /// Drawing a `Text` with a transparent `MultiMonoTextStyle` will not draw any pixels.
    ///
    /// [`Text`]: super::text::Text
    pub fn is_transparent(&self) -> bool {
        self.text_color.is_none() && self.background_color.is_none()
    }

    fn get_font_info(&self, c: char) -> &MultiMonoFont<'a> {
        for font in self.fonts {
            if font.glyph_mapping.contains(c) {
                return font;
            }
        }

        self.fonts[0]
    }

    fn draw_string_binary<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        mut target: D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut next_pos = position;
        let mut draw_pos;

        for c in text.chars() {
            let font = self.get_font_info(c);
            let glyph = font.glyph(c);
            draw_pos = next_pos - Point::new(0, self.baseline_offset(baseline, font));
            Image::new(&glyph, draw_pos).draw(&mut target)?;
            next_pos.x += font.character_size.width as i32;
            if font.character_spacing > 0 {
                if self.background_color.is_some() {
                    target.fill_solid(
                        &Rectangle::new(
                            next_pos,
                            CharSize::new(font.character_spacing, self.line_height).size(),
                        ),
                        BinaryColor::Off,
                    )?;
                }
                next_pos.x += font.character_spacing as i32;
            }
        }

        Ok(next_pos)
    }

    /// Returns the vertical offset between the line position and the top edge of the bounding box.
    fn baseline_offset(&self, baseline: Baseline, font: &MultiMonoFont<'a>) -> i32 {
        match baseline {
            Baseline::Top => 0,
            Baseline::Bottom => font.character_size.height.saturating_sub(1) as i32,
            Baseline::Middle => (font.character_size.height.saturating_sub(1) / 2) as i32,
            Baseline::Alphabetic => font.baseline as i32,
        }
    }
}

impl<C> TextRenderer for MultiMonoTextStyle<'_, C>
where
    C: PixelColor,
{
    type Color = C;

    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let next = match (self.text_color, self.background_color) {
            (Some(text_color), Some(background_color)) => self.draw_string_binary(
                text,
                position,
                baseline,
                MonoFontDrawTarget::new(target, Both(text_color, background_color)),
            )?,
            (Some(text_color), None) => self.draw_string_binary(
                text,
                position,
                baseline,
                MonoFontDrawTarget::new(target, Foreground(text_color)),
            )?,
            (None, Some(background_color)) => self.draw_string_binary(
                text,
                position,
                baseline,
                MonoFontDrawTarget::new(target, Background(background_color)),
            )?,
            (None, None) => {
                let tm = self.measure_string(text, position, baseline);
                let dx = tm.bounding_box.size.width;

                position + Size::new(dx, 0)
            }
        };

        Ok(next)
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let (offet_y, height) = match baseline {
            Baseline::Top => (0, self.line_height),
            Baseline::Bottom => (self.line_height.saturating_sub(1) as i32, self.line_height),
            Baseline::Middle => (
                (self.line_height.saturating_sub(1) / 2) as i32,
                self.line_height,
            ),
            Baseline::Alphabetic => (
                self.fonts[0].baseline as i32,
                self.fonts[0].character_size.height,
            ),
        };
        let position = position - Point::new(0, offet_y);

        if width != 0 {
            if let Some(background_color) = self.background_color {
                target.fill_solid(
                    &Rectangle::new(position, Size::new(width, height as u32)),
                    background_color,
                )?;
            }
        }

        Ok(position + Point::new(width as i32, offet_y))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let mut bb_width = 0;
        let mut bb_height = 0;
        let mut baseline_max = 0;
        let mut font = self.fonts[0];
        for c in text.chars() {
            font = self.get_font_info(c);
            bb_width += (font.character_size.width + font.character_spacing) as u32;
            bb_height = bb_height.max(font.character_size.height as u32);

            baseline_max = baseline_max.max(self.baseline_offset(baseline, font));
        }
        bb_width = bb_width.saturating_sub(font.character_spacing as u32);

        let bb_size = Size::new(bb_width, bb_height);

        let bb_position = position - Point::new(0, baseline_max);
        TextMetrics {
            bounding_box: Rectangle::new(bb_position, bb_size),
            next_position: position + bb_size.x_axis(),
        }
    }

    fn line_height(&self) -> u32 {
        self.line_height as u32
    }
}

impl<C> CharacterStyle for MultiMonoTextStyle<'_, C>
where
    C: PixelColor,
{
    type Color = C;

    fn set_text_color(&mut self, text_color: Option<Self::Color>) {
        self.text_color = text_color;
    }

    fn set_background_color(&mut self, background_color: Option<Self::Color>) {
        self.background_color = background_color;
    }
}

/// Text style builder for monospaced fonts.
///
/// Use this builder to create [`MultiMonoTextStyle`]s for [`Text`].
///
/// # Examples
///
/// ## Render yellow text on a blue background
///
/// This uses the [`FONT_6X9`] font, but [other fonts] can also be used.
///
/// ```rust
/// use multi_mono_font::{ascii::FONT_6X9, MultiMonoTextStyle, MultiMonoTextStyleBuilder};
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MultiMonoTextStyleBuilder::new()
///     .font(&FONT_6X9)
///     .text_color(Rgb565::YELLOW)
///     .background_color(Rgb565::BLUE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0), style);
/// ```
///
/// ## Transparent background
///
/// If a property is omitted, it will remain at its default value in the resulting
/// `MultiMonoTextStyle` returned by `.build()`. This example draws white text with no background at
/// all.
///
/// ```rust
/// use multi_mono_font::{ascii::FONT_6X9, MultiMonoTextStyle, MultiMonoTextStyleBuilder};
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MultiMonoTextStyleBuilder::new()
///     .font(&FONT_6X9)
///     .text_color(Rgb565::WHITE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0), style);
/// ```
///
/// ## Modifying an existing style
///
/// The builder can also be used to modify an existing style.
///
/// ```
/// use multi_mono_font::{ascii::FONT_6X9, MultiMonoTextStyle, MultiMonoTextStyleBuilder};
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MultiMonoTextStyle::new(&FONT_6X9, Rgb565::YELLOW);
///
/// let style_larger = MultiMonoTextStyleBuilder::from(&style)
///     .font(&FONT_10X20)
///     .build();
/// ```
///
/// [`FONT_6X9`]: crate::::ascii::FONT_6X9
/// [other fonts]: super
/// [`Text`]: crate::text::Text
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct MultiMonoTextStyleBuilder<'a, C> {
    style: MultiMonoTextStyle<'a, C>,
}

impl<'a, C> MultiMonoTextStyleBuilder<'a, C>
where
    C: PixelColor,
{
    /// Creates a new text style builder.
    pub const fn new() -> Self {
        Self {
            style: MultiMonoTextStyle {
                fonts: &[&super::NULL_FONT],
                background_color: None,
                text_color: None,
                line_height: 0,
            },
        }
    }

    /// Sets the font.
    pub const fn font<'b>(
        self,
        font_list: &'b [&'b MultiMonoFont<'b>],
        line_height: MultiMonoLineHeight,
    ) -> MultiMonoTextStyleBuilder<'b, C> {
        let fonts = if font_list.len() == 0 {
            &[&crate::NULL_FONT]
        } else {
            font_list
        };
        let line_height = get_line_height(line_height, fonts);
        let style = MultiMonoTextStyle {
            fonts,
            background_color: self.style.background_color,
            text_color: self.style.text_color,
            line_height,
        };

        MultiMonoTextStyleBuilder { style }
    }

    /// Resets the text color to transparent.
    pub const fn reset_text_color(mut self) -> Self {
        self.style.text_color = None;

        self
    }

    /// Resets the background color to transparent.
    pub const fn reset_background_color(mut self) -> Self {
        self.style.background_color = None;

        self
    }

    /// Sets the text color.
    pub const fn text_color(mut self, text_color: C) -> Self {
        self.style.text_color = Some(text_color);

        self
    }

    /// Sets the line height.
    pub const fn line_height(mut self, line_height: MultiMonoLineHeight) -> Self {
        self.style.line_height = get_line_height(line_height, self.style.fonts);

        self
    }

    /// Sets the background color.
    pub const fn background_color(mut self, background_color: C) -> Self {
        self.style.background_color = Some(background_color);

        self
    }

    /// Builds the text style.
    ///
    /// This method can only be called after a font was set by using the [`font`] method. All other
    /// settings are optional and they will be set to their default value if they are missing.
    ///
    /// [`font`]: MultiMonoTextStyleBuilder::font()
    pub const fn build(self) -> MultiMonoTextStyle<'a, C> {
        self.style
    }
}

impl<'a, C> From<&MultiMonoTextStyle<'a, C>> for MultiMonoTextStyleBuilder<'a, C>
where
    C: PixelColor,
{
    fn from(style: &MultiMonoTextStyle<'a, C>) -> Self {
        Self { style: *style }
    }
}
