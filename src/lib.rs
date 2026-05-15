#![no_std]

mod char_size;
mod draw_target;
mod framebuf;
mod glyph_reader;
pub mod mapping;
mod mono_image;
mod multi_mono_text_style;
mod static_text;

pub use char_size::CharSize;
pub use framebuf::{BulkFlushTarget, Framebuffer};
use mapping::StrGlyphMapping;
pub use mono_image::{MonoImage, MonoImageStack, MonoRleImage};
pub use multi_mono_text_style::{
    MultiMonoLineHeight, MultiMonoTextStyle, MultiMonoTextStyleBuilder,
};
pub use static_text::StaticText;

pub type MultiMonoFontList<'a> = &'a [&'a MultiMonoFont<'a>];

cfg_if::cfg_if! {
    if #[cfg(feature = "font-rawimg")] {
        mod generated;
        mod sub_image;

        pub use generated::*;
        use sub_image::SubImage;
        use embedded_graphics::{
            geometry::{OriginDimensions, Point},
            image::ImageRaw,
            pixelcolor::BinaryColor,
            primitives::Rectangle,
        };
    }
}

#[cfg(not(any(feature = "font-rawimg", feature = "font-rle")))]
compile_error!("At least one of the features 'font-rawimg' or 'font-rle' must be enabled");

cfg_if::cfg_if! {
    if #[cfg(feature = "big-character-size")] {
        pub type ChSzTy = u16;
    } else {
        pub type ChSzTy = u8;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "rle-index-ty-32bit")] {
        pub type RleIdxTy = u32;
    } else {
        pub type RleIdxTy = u16;
    }
}

/// A trait for objects that can be scaled.
pub trait Scalable {
    type T;
    fn set_scale(&mut self, scale: Self::T);
    fn get_scale(&self) -> Self::T;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "font-rle")] {
        #[derive(Clone, Copy, PartialEq)]
        pub struct RLERaw {
            glyphs_data: &'static [u8],
            glyphs_index: &'static [RleIdxTy],
        }

        impl RLERaw {
            pub const fn new(glyphs_data: &'static [u8], glyphs_index: &'static [RleIdxTy]) -> Self {
                Self {
                    glyphs_data,
                    glyphs_index,
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GlyphData {
    /// Raw image data.
    #[cfg(feature = "font-rawimg")]
    ImgRaw(ImageRaw<'static, BinaryColor>),
    /// RLE compressed image data.
    #[cfg(feature = "font-rle")]
    RLE(RLERaw),
}

/// Monospaced bitmap font.
///
/// See the [module documentation] for more information about using fonts.
///
/// [module documentation]: self
#[derive(Clone, Copy)]
pub struct MultiMonoFont<'a> {
    /// Font data.
    pub glyph_data: GlyphData,

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
    #[cfg(feature = "font-rawimg")]
    pub(crate) fn glyph_img<'b>(
        &self,
        c: char,
        image: &'b ImageRaw<'static, BinaryColor>,
    ) -> SubImage<'b, ImageRaw<'_, BinaryColor>> {
        if self.character_size.width == 0 || image.size().width < self.character_size.width as u32 {
            return SubImage::new_unchecked(image, Rectangle::zero());
        }

        let glyphs_per_row = image.size().width / self.character_size.width as u32;

        // Char _code_ offset from first char, most often a space
        // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
        let glyph_index = self.glyph_mapping.index(c) as u32;
        let row = glyph_index / glyphs_per_row;

        // Top left corner of character, in pixels
        let char_x = (glyph_index - (row * glyphs_per_row)) * self.character_size.width as u32;
        let char_y = row * self.character_size.height as u32;

        SubImage::new_unchecked(
            image,
            Rectangle::new(
                Point::new(char_x as i32, char_y as i32),
                self.character_size.size(),
            ),
        )
    }

    #[cfg(feature = "font-rle")]
    pub(crate) fn glyph_rle(&self, c: char, rle_raw: &RLERaw) -> crate::glyph_reader::GlyphReader {
        let idx = self.glyph_mapping.index(c);
        let glyph = if idx == 0 {
            rle_raw.glyphs_data
        } else {
            let start = rle_raw.glyphs_index.get(idx - 1).copied().unwrap_or(0) as usize;
            rle_raw
                .glyphs_data
                .get(start..)
                .unwrap_or(rle_raw.glyphs_data)
        };
        crate::glyph_reader::GlyphReader::new(glyph)
    }
}

impl PartialEq for MultiMonoFont<'_> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &Self) -> bool {
        self.glyph_data == other.glyph_data
            && self.character_size == other.character_size
            && self.character_spacing == other.character_spacing
            && self.baseline == other.baseline
            && core::ptr::eq(self.glyph_mapping, other.glyph_mapping)
    }
}

impl core::fmt::Debug for GlyphData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "font-rawimg")]
            GlyphData::ImgRaw(_) => f.write_str("ImageRaw"),
            #[cfg(feature = "font-rle")]
            GlyphData::RLE(_) => f.write_str("RLERaw"),
        }
    }
}
impl core::fmt::Debug for MultiMonoFont<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MultiMonoFont")
            .field("glyph_data", &self.glyph_data)
            .field("character_size", &self.character_size)
            .field("character_spacing", &self.character_spacing)
            .field("baseline", &self.baseline)
            .field("glyph_mapping", &"?")
            .finish_non_exhaustive()
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "font-rawimg")] {
        const NULL_FONT: MultiMonoFont = MultiMonoFont {
            glyph_data: GlyphData::ImgRaw(ImageRaw::new(&[], 1)),
            character_size: CharSize::zero(),
            character_spacing: 0,
            baseline: 0,
            glyph_mapping: &StrGlyphMapping::new("", 0),
        };
    } else {
        const NULL_FONT: MultiMonoFont = MultiMonoFont {
            glyph_data: GlyphData::RLE(RLERaw::new(&[], &[])),
            character_size: CharSize::zero(),
            character_spacing: 0,
            baseline: 0,
            glyph_mapping: &StrGlyphMapping::new("", 0),
        };
    }
}
