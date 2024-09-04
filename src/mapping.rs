//! Glyph mapping.
//!
//! A glyph mapping defines the position of characters in a [`MultiMonoFont`] image. This module provides
//! predefined mappings for common glyph subsets, but custom mappings are also supported.
//!
//! # Custom mappings
//!
//! Custom mappings can be defined in three different ways:
//! * The [`StrGlyphMapping`] type can be used to specify a character mapping by encoding the
//!   mapping as a string.
//!
//! # `StrGlyphMapping` encoding
//!
//! Strings without a `\0` character can be used to directly map a character to its position in
//! the mapping string:
//!
//! ```
//! use multi_mono_font::mapping::StrGlyphMapping;
//!
//! let mapping = StrGlyphMapping::new("abcdef1234", 0);
//! assert_eq!(mapping.index('a'), 0);
//! assert_eq!(mapping.index('b'), 1);
//! assert_eq!(mapping.index('1'), 6);
//! assert_eq!(mapping.index('2'), 7);
//! ```
//!
//! This direct mapping is inefficient for mappings that map consecutive ranges of characters to
//! consecutive index ranges. To define a range of characters a `\0` character followed by the
//! start and end characters of the inclusive range can be used. This way the mapping in the previous
//! example can be abbreviated to:
//!
//! ```
//! use multi_mono_font::mapping::StrGlyphMapping;
//!
//! let mapping = StrGlyphMapping::new("\0af\014", 0);
//! assert_eq!(mapping.index('a'), 0);
//! assert_eq!(mapping.index('b'), 1);
//! assert_eq!(mapping.index('1'), 6);
//! assert_eq!(mapping.index('2'), 7);
//! ```
//!
//! [`MultiMonoFont`]: super::MultiMonoFont

use core::ops::RangeInclusive;

/// Glyph mapping stored as a UTF-8 string.
///
/// See the [module-level documentation] for more details.
///
/// [module-level documentation]: self
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrGlyphMapping<'a> {
    data: &'a str,
    replacement_index: usize,
}

impl<'a> StrGlyphMapping<'a> {
    /// Creates a new glyph mapping.
    pub const fn new(data: &'a str, replacement_index: usize) -> Self {
        Self {
            data,
            replacement_index,
        }
    }

    /// Returns an iterator over the character ranges.
    pub fn ranges(&self) -> impl Iterator<Item = (usize, RangeInclusive<char>)> + '_ {
        let mut chars = self.data.chars();
        let mut index = 0;

        core::iter::from_fn(move || {
            let start_index = index;

            let range = match chars.next()? {
                '\0' => {
                    let start = chars.next()?;
                    let end = chars.next()?;

                    index += end as usize - start as usize + 1;

                    start..=end
                }
                c => {
                    index += 1;

                    c..=c
                }
            };

            Some((start_index, range))
        })
    }

    /// Returns an iterator over the characters in this mapping.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        let mut chars = self.data.chars();

        core::iter::from_fn(move || {
            let range = match chars.next()? {
                '\0' => {
                    let start = chars.next()?;
                    let end = chars.next()?;

                    start..=end
                }
                c => c..=c,
            };

            Some(range)
        })
        .flatten()
    }

    /// Returns if the mapping contains the given char.
    pub fn contains(&self, c: char) -> bool {
        self.chars().any(|v| v == c)
    }

    pub fn index(&self, c: char) -> usize {
        // PERF: use ranges instead of chars iter
        self.chars()
            .enumerate()
            .find(|(_, v)| c == *v)
            .map(|(index, _)| index)
            .unwrap_or(self.replacement_index)
    }
}

macro_rules! impl_mapping {
    ($( $(#[$meta:meta])* ($enum_variant:ident, $constant:ident, $mapping:expr), )*) => {
        /// Mapping.
        ///
        /// This enum lists all mappings that are included in embedded-graphics. It is used
        /// to automatically generate font data for all mappings and isn't normally used in
        /// applications.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Mapping {
            $(
                $(#[$meta])*
                $enum_variant,
            )*
        }

        impl Mapping {
            /// Returns an iterator over all mappings.
            pub fn iter() -> impl Iterator<Item = Self> {
                const ALL: &[Mapping] = &[$(Mapping::$enum_variant),*];

                ALL.iter().copied()
            }

            /// Returns the MIME identifier for this mapping.
            pub const fn mime(self) -> &'static str {
                match self {
                    $(Mapping::$enum_variant => stringify!($constant)),*
                }
            }

            /// Returns a glyph mapping for this mapping.
            pub const fn glyph_mapping(self) -> &'static StrGlyphMapping<'static> {
                match self {
                    $(Self::$enum_variant => &$constant),*
                }
            }
        }

        $(
            $(#[$meta])*
            pub const $constant: StrGlyphMapping = StrGlyphMapping::new($mapping, '?' as usize - ' ' as usize);
        )*
    };
}

impl_mapping!(
    /// ASCII.
    (Ascii, ASCII, "\0\u{20}\u{7f}"),
);
