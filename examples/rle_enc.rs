use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use multi_mono_font::{
    CharSize, GlyphData, MonoRleImage, MultiMonoFont, MultiMonoLineHeight, MultiMonoTextStyle,
    MultiMonoTextStyleBuilder, RLERaw, StaticText, mapping::StrGlyphMapping,
};

const UPPER_FONT: MultiMonoFont = MultiMonoFont {
    glyph_data: GlyphData::RLE(RLERaw::new(
        include_bytes!("fonts/upper_rle.bin"),
        &[
            13, 26, 37, 50, 63, 75, 87, 100, 109, 120, 133, 142, 157, 170, 182, 193, 205, 217, 228,
            239, 252, 264, 279, 291, 302,
        ],
    )),
    glyph_mapping: &StrGlyphMapping::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0),
    character_size: CharSize::new(6, 12),
    character_spacing: 2,
    baseline: 12,
};

const LOWER_FONT: MultiMonoFont = MultiMonoFont {
    glyph_data: GlyphData::RLE(RLERaw::new(
        include_bytes!("fonts/lower_rle.bin"),
        &[
            41, 84, 117, 166, 203, 239, 284, 327, 356, 388, 436, 469, 529, 569, 607, 655, 700, 732,
            768, 803, 845, 880, 933, 968, 1006,
        ],
    )),
    glyph_mapping: &StrGlyphMapping::new("abcdefghijklmnopqrstuvwxyz", 0),
    character_size: CharSize::new(16, 32),
    character_spacing: 3,
    baseline: 32,
};

const HZ_FONT: MultiMonoFont = MultiMonoFont {
    glyph_data: GlyphData::RLE(RLERaw::new(
        include_bytes!("fonts/HZ_rle.bin"),
        &[43, 107, 190],
    )),
    glyph_mapping: &StrGlyphMapping::new("字体测试", 0),
    character_size: CharSize::new(24, 24),
    character_spacing: 4,
    baseline: 24,
};

const ASCII_72X136_FONT: MultiMonoFont = MultiMonoFont {
    glyph_data: GlyphData::RLE(RLERaw::new(
        include_bytes!("fonts/ASCII_72X136.bin"),
        &[
            3, 155, 281, 586, 943, 1383, 1738, 1813, 1992, 2171, 2419, 2560, 2635, 2651, 2691,
            2851, 3152, 3334, 3589, 3828, 4087, 4342, 4628, 4836, 5130, 5415, 5478, 5556, 5702,
            5728, 5887, 6111, 6566, 6876, 7191, 7451, 7773, 8058, 8313, 8601, 8929, 9109, 9333,
            9643, 9854, 10339, 10729, 11030, 11280, 11624, 11946, 12210, 12446, 12752, 13029,
            13471, 13754, 13998, 14234, 14442, 14615, 14824, 14885, 14900, 14937, 15184, 15458,
            15651, 15924, 16124, 16325, 16605, 16875, 17038, 17230, 17497, 17682, 18008, 18243,
            18460, 18718, 18975, 19156, 19354, 19543, 19782, 19986, 20312, 20513, 20740, 20929,
            21108, 21278, 21457,
        ],
    )),
    glyph_mapping: &StrGlyphMapping::new(
        " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
        0,
    ),
    character_size: CharSize::new(72, 136),
    character_spacing: 4,
    baseline: 136,
};
const ASCII_STYLE0: MultiMonoTextStyle<Rgb565> = MultiMonoTextStyleBuilder::new(Rgb565::BLACK)
    .font(&[&ASCII_72X136_FONT], MultiMonoLineHeight::Max)
    .background_color(Rgb565::WHITE)
    .build();

const MULTI_STYLE0: MultiMonoTextStyle<Rgb565> = MultiMonoTextStyleBuilder::new(Rgb565::RED)
    .font(
        &[&UPPER_FONT, &HZ_FONT, &LOWER_FONT],
        MultiMonoLineHeight::Max,
    )
    .background_color(Rgb565::GREEN)
    .build();

const MULTI_STYLE1: MultiMonoTextStyle<Rgb565> = MultiMonoTextStyleBuilder::new(Rgb565::YELLOW)
    .font(
        &[&UPPER_FONT, &LOWER_FONT, &HZ_FONT],
        MultiMonoLineHeight::Specify(20),
    )
    .background_color(Rgb565::BLUE)
    .build();

// RLE 压缩率: 32.94%, 压缩前: 1567.88B, 压缩后: 515B
const RLE_IMG_FU: MonoRleImage<Rgb565> =
    MonoRleImage::new(include_bytes!("imgs/fu_111x113.bin"), 111, 113, Rgb565::RED);

// RLE 压缩率: 38.61%, 压缩前: 4350.00B, 压缩后: 1680B
const RLE_IMG_FERRIS: MonoRleImage<Rgb565> =
    MonoRleImage::new(include_bytes!("imgs/Ferris.bin"), 240, 145, Rgb565::RED);

fn main() -> Result<(), core::convert::Infallible> {
    simple_logger::init().ok();
    let mut disp = SimulatorDisplay::<Rgb565>::new(Size::new(520, 320));

    let _next = Text::new("测HElLo试OK\n", Point::new(0, 32), MULTI_STYLE0)
        .draw(&mut disp)
        .unwrap();

    let _next = Text::new("字WoRlD体ok\n", _next, MULTI_STYLE0)
        .draw(&mut disp)
        .unwrap();

    let _next = Text::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ\n", _next, MULTI_STYLE0)
        .draw(&mut disp)
        .unwrap();
    let _next = Text::new("abcdefghijklmnopqrstuvwxyz\n", _next, MULTI_STYLE0)
        .draw(&mut disp)
        .unwrap();
    let _next = Text::new("字体测试\n", _next, MULTI_STYLE0)
        .draw(&mut disp)
        .unwrap();

    Image::new(&RLE_IMG_FU, Point::new(250, 4))
        .draw(&mut disp)
        .unwrap();

    Image::new(&RLE_IMG_FERRIS, Point::new(0, 170))
        .draw(&mut disp)
        .unwrap();

    const RECT_STYLE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::MAGENTA)
        .stroke_width(1)
        .build();

    let rect = Rectangle::new(Point::new(52, 20), Size::new(64, 24));
    StaticText::with_style(
        "HE字LL",
        rect,
        MULTI_STYLE1,
        Alignment::Center,
        Baseline::Alphabetic,
    )
    .draw(&mut disp)
    .unwrap();
    rect.into_styled(RECT_STYLE).draw(&mut disp).unwrap();

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut win = Window::new("HelloWorld", &output_settings);

    for ch in ASCII_72X136_FONT.glyph_mapping.chars() {
        win.update(&disp);
        std::thread::sleep(std::time::Duration::from_millis(500));
        let _next = Text::new(
            &ch.to_string(),
            Point::new(520 - 80, 240 - 40),
            ASCII_STYLE0,
        )
        .draw(&mut disp)
        .unwrap();
        if win
            .events()
            .any(|e| e == embedded_graphics_simulator::SimulatorEvent::Quit)
        {
            return Ok(());
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if win
            .events()
            .any(|e| e == embedded_graphics_simulator::SimulatorEvent::Quit)
        {
            break;
        }
    }

    Ok(())
}
