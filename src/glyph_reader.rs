use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, primitives::Rectangle};

pub(crate) fn render_glyph_as_box_fill<D>(
    bounding_box: &Rectangle,
    mut glyph: GlyphReader,
    display: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let color_iter = {
        let mut num_zeros_leftover = glyph.read_runlength_0();
        let mut num_ones_leftover = glyph.read_runlength_1();
        move || -> Option<BinaryColor> {
            if num_zeros_leftover == 0 && num_ones_leftover == 0 {
                num_zeros_leftover = glyph.read_runlength_0();
                num_ones_leftover = glyph.read_runlength_1();
            }

            let color = if num_zeros_leftover > 0 {
                num_zeros_leftover -= 1;
                BinaryColor::Off
            } else if num_ones_leftover > 0 {
                num_ones_leftover -= 1;
                BinaryColor::On
            } else {
                return None;
            };

            Some(color)
        }
    };

    display.fill_contiguous(bounding_box, core::iter::from_fn(color_iter))
}

#[derive(Clone, Debug)]
pub(crate) struct GlyphReader {
    data: &'static [u8],
    bit_pos: u8,
    current_byte: u8,
    bitcount_0: u8,
    bitcount_1: u8,
}

impl GlyphReader {
    pub fn new(data: &'static [u8]) -> Self {
        let bitcount = data.first().copied().unwrap_or(0);

        Self {
            data: data.get(1..).unwrap_or(&[]),
            // Start at 8 to mark current_byte as invalid
            bit_pos: 8,
            current_byte: 0,
            bitcount_0: (bitcount >> 4) + 1,  //0~15 => 1~16
            bitcount_1: (bitcount & 0xF) + 1, //0~15 => 1~16
        }
    }

    pub fn read_unsigned(&mut self, bits: u8) -> u8 {
        if self.bit_pos == 8 {
            self.current_byte = self.data.first().copied().unwrap_or(0);
            self.data = self.data.get(1..).unwrap_or(&[]);
            self.bit_pos = 0;
        }

        let available = 8 - self.bit_pos;

        if bits <= available {
            let value = self.current_byte >> (available - bits);
            self.bit_pos += bits;
            value & (((1u16 << bits) - 1) as u8)
        } else {
            let first = self.current_byte & ((1 << available) - 1);
            let second_bits = bits - available;
            let next_byte = self.data.first().copied().unwrap_or(0);
            self.data = self.data.get(1..).unwrap_or(&[]);
            self.current_byte = next_byte;
            self.bit_pos = second_bits;
            (first << second_bits) | (next_byte >> (8 - second_bits))
        }
    }

    pub fn read_runlength(&mut self, bitcount: u8) -> u16 {
        if bitcount > 8 {
            ((self.read_unsigned(8) as u16) << (bitcount - 8))
                | self.read_unsigned(bitcount - 8) as u16
        } else {
            self.read_unsigned(bitcount) as u16
        }
    }

    pub fn read_runlength_0(&mut self) -> u16 {
        self.read_runlength(self.bitcount_0)
    }

    pub fn read_runlength_1(&mut self) -> u16 {
        self.read_runlength(self.bitcount_1)
    }
}
