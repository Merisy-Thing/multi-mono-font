use embedded_graphics::prelude::Size;

use crate::ChSzTy;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct CharSize {
    /// The width.
    pub width: ChSzTy,

    /// The height.
    pub height: ChSzTy,
}
impl CharSize {
    /// Creates a size from a width and a height.
    pub const fn new(width: ChSzTy, height: ChSzTy) -> Self {
        CharSize { width, height }
    }

    /// Creates a size with width and height set to an equal value.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Size;
    ///
    /// let size = Size::new_equal(11);
    ///
    /// assert_eq!(
    ///     size,
    ///     Size {
    ///         width: 11,
    ///         height: 11
    ///     }
    /// );
    /// ```
    pub const fn new_equal(value: ChSzTy) -> Self {
        CharSize {
            width: value,
            height: value,
        }
    }

    /// Creates a size with width and height equal to zero.
    pub const fn zero() -> Self {
        CharSize {
            width: 0,
            height: 0,
        }
    }
}

impl embedded_graphics::prelude::OriginDimensions for CharSize {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}
