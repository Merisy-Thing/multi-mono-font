use embedded_graphics::{
    draw_target::DrawTarget, geometry::OriginDimensions, image::ImageDrawable,
    primitives::Rectangle, transform::Transform,
};

/// Sub image.
///
/// A sub image is rectangular subsection of an [`ImageDrawable`]. It can, for example, be used to
/// draw individual sprites from a larger sprite atlas.
///
/// To create a sub image call the [`sub_image`] method on the parent [`ImageDrawable`]. See the
/// [module-level documentation] for an example.
///
/// [`sub_image`]: trait.ImageDrawableExt.html#tymethod.sub_image
/// [module-level documentation]: super#sub-images
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct SubImage<'a, T> {
    parent: &'a T,
    area: Rectangle,
}

impl<'a, T> SubImage<'a, T>
where
    T: ImageDrawable,
{
    pub(crate) const fn new_unchecked(parent: &'a T, area: Rectangle) -> Self {
        Self { parent, area }
    }
}

impl<T> OriginDimensions for SubImage<'_, T> {
    fn size(&self) -> embedded_graphics::prelude::Size {
        self.area.size
    }
}

impl<'a, T> ImageDrawable for SubImage<'a, T>
where
    T: ImageDrawable,
{
    type Color = T::Color;

    fn draw<DT>(&self, target: &mut DT) -> Result<(), DT::Error>
    where
        DT: DrawTarget<Color = Self::Color>,
    {
        self.parent.draw_sub_image(target, &self.area)
    }

    fn draw_sub_image<DT>(&self, target: &mut DT, area: &Rectangle) -> Result<(), DT::Error>
    where
        DT: DrawTarget<Color = Self::Color>,
    {
        let area = area.translate(self.area.top_left);

        self.parent.draw_sub_image(target, &area)
    }
}
