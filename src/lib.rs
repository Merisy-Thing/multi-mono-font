mod char_size;
mod draw_target;
mod generated;
pub mod mapping;
mod multi_mono_text_style;
mod sub_image;

use core::fmt;

pub use char_size::CharSize;
pub use generated::*;
use mapping::StrGlyphMapping;
pub use multi_mono_text_style::{
    MultiMonoLineHeight, MultiMonoTextStyle, MultiMonoTextStyleBuilder,
};

use embedded_graphics::{
    geometry::{OriginDimensions, Point},
    image::ImageRaw,
    pixelcolor::BinaryColor,
    primitives::Rectangle,
};
use sub_image::SubImage;

#[cfg(not(feature = "big-character-size"))]
pub type ChSzTy = u8;
#[cfg(feature = "big-character-size")]
pub type ChSzTy = u16;

/// Monospaced bitmap font.
///
/// See the [module documentation] for more information about using fonts.
///
/// [module documentation]: self
#[derive(Clone, Copy)]
pub struct MultiMonoFont<'a> {
    /// Raw image data containing the font.
    pub image: ImageRaw<'a, BinaryColor>,

    /// Size of a single character in pixel.
    pub character_size: CharSize,

    /// Spacing between characters.
    ///
    /// The spacing defines how many empty pixels are added horizontally between adjacent characters
    /// on a single line of text.
    pub character_spacing: ChSzTy,

    /// The baseline.
    ///
    /// Offset from the top of the glyph bounding box to the baseline.
    pub baseline: ChSzTy,

    /// Glyph mapping.
    pub glyph_mapping: &'a StrGlyphMapping<'a>,
}

impl MultiMonoFont<'_> {
    /// Returns a subimage for a glyph.
    pub(crate) fn glyph(&self, c: char) -> SubImage<'_, ImageRaw<BinaryColor>> {
        if self.character_size.width == 0
            || self.image.size().width < self.character_size.width as u32
        {
            return SubImage::new_unchecked(&self.image, Rectangle::zero());
        }

        let glyphs_per_row = self.image.size().width / self.character_size.width as u32;

        // Char _code_ offset from first char, most often a space
        // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
        let glyph_index = self.glyph_mapping.index(c) as u32;
        let row = glyph_index / glyphs_per_row;

        // Top left corner of character, in pixels
        let char_x = (glyph_index - (row * glyphs_per_row)) * self.character_size.width as u32;
        let char_y = row * self.character_size.height as u32;

        SubImage::new_unchecked(
            &self.image,
            Rectangle::new(
                Point::new(char_x as i32, char_y as i32),
                self.character_size.size(),
            ),
        )
    }
}

impl PartialEq for MultiMonoFont<'_> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.image == other.image
            && self.character_size == other.character_size
            && self.character_spacing == other.character_spacing
            && self.baseline == other.baseline
            && core::ptr::eq(self.glyph_mapping, other.glyph_mapping)
    }
}

impl fmt::Debug for MultiMonoFont<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MultiMonoFont")
            .field("image", &self.image)
            .field("character_size", &self.character_size)
            .field("character_spacing", &self.character_spacing)
            .field("baseline", &self.baseline)
            .field("glyph_mapping", &"?")
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "defmt")]
impl ::defmt::Format for MultiMonoFont<'_> {
    fn format(&self, f: ::defmt::Formatter) {
        ::defmt::write!(
            f,
            "MultiMonoFont {{ image: {}, character_size: {}, character_spacing: {}, baseline: {}, strikethrough: {}, underline: {}, .. }}",
            &self.image,
            &self.character_size,
            &self.character_spacing,
            &self.baseline,
            &self.strikethrough,
            &self.underline,

        )
    }
}

const NULL_FONT: MultiMonoFont = MultiMonoFont {
    image: ImageRaw::new(&[], 1),
    character_size: CharSize::zero(),
    character_spacing: 0,
    baseline: 0,
    glyph_mapping: &StrGlyphMapping::new("", 0),
};
