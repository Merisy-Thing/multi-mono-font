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

    /// ISO/IEC 8859 Part 10: Latin-6, Nordic.
    (Iso8859_10, ISO_8859_10, "\0\u{20}\u{7f}\u{a0}\u{104}\u{112}\u{122}\u{12a}\u{128}\u{136}\u{a7}\u{13b}\u{110}\u{160}\u{166}\u{17d}\u{ad}\u{16a}\u{14a}\u{b0}\u{105}\u{113}\u{123}\u{12b}\u{129}\u{137}\u{b7}\u{13c}\u{111}\u{161}\u{167}\u{17e}\u{2015}\u{16b}\u{14b}\u{100}\0\u{c1}\u{c6}\u{12e}\u{10c}\u{c9}\u{118}\u{cb}\u{116}\0\u{cd}\u{d0}\u{145}\u{14c}\0\u{d3}\u{d6}\u{168}\u{d8}\u{172}\0\u{da}\u{df}\u{101}\0\u{e1}\u{e6}\u{12f}\u{10d}\u{e9}\u{119}\u{eb}\u{117}\0\u{ed}\u{f0}\u{146}\u{14d}\0\u{f3}\u{f6}\u{169}\u{f8}\u{173}\0\u{fa}\u{fe}\u{138}"),
);
