//! Prelude module for convenient re-exports.
//!
//! This module re-exports all public items from the crate, allowing users
//! to import everything at once with `use multi_mono_font::prelude::*;`.

pub use crate::{
    BulkFlushTarget, CharSize, ChSzTy, Framebuffer, GlyphData, MonoImage, MonoImageStack,
    MonoRleImage, MultiMonoFont, MultiMonoFontList, MultiMonoLineHeight, MultiMonoTextStyle,
    MultiMonoTextStyleBuilder, RleIdxTy, Scalable, StaticText,
};

#[cfg(feature = "font-rle")]
pub use crate::RLERaw;

#[cfg(feature = "font-rawimg")]
pub use crate::{ascii, iso_8859_10};

pub use crate::mapping::{ASCII, ISO_8859_10, Mapping, StrGlyphMapping};
