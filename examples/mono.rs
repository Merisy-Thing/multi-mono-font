use embedded_graphics::{
    image::ImageRaw,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Baseline, Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use multi_mono_font::{
    mapping::StrGlyphMapping, CharSize, MultiMonoFont, MultiMonoLineHeight, MultiMonoTextStyle,
    MultiMonoTextStyleBuilder, StaticText,
};

const UPPER_FONT: MultiMonoFont = MultiMonoFont {
    image: ImageRaw::new(include_bytes!("fonts/upper.bin"), 96),
    glyph_mapping: &StrGlyphMapping::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 26),
    character_size: CharSize::new(6, 12),
    character_spacing: 0,
    baseline: 12,
};

const LOWER_FONT: MultiMonoFont = MultiMonoFont {
    image: ImageRaw::new(include_bytes!("fonts/lower.bin"), 128),
    glyph_mapping: &StrGlyphMapping::new("abcdefghijklmnopqrstuvwxyz", 0),
    character_size: CharSize::new(16, 32),
    character_spacing: 0,
    baseline: 32,
};

const HZ_FONT: MultiMonoFont = MultiMonoFont {
    image: ImageRaw::new(include_bytes!("fonts/HZ.bin"), 192),
    glyph_mapping: &StrGlyphMapping::new("字体测试", 0),
    character_size: CharSize::new(24, 24),
    character_spacing: 0,
    baseline: 24,
};

const MULTI_STYLE: MultiMonoTextStyle<Rgb565> = MultiMonoTextStyleBuilder::new()
    .font(
        &[&UPPER_FONT, &HZ_FONT, &LOWER_FONT],
        MultiMonoLineHeight::Max,
    )
    .text_color(Rgb565::WHITE)
    .background_color(Rgb565::BLACK)
    .build();

fn main() -> Result<(), core::convert::Infallible> {
    let mut disp = SimulatorDisplay::<Rgb565>::new(Size::new(160, 80));

    let _next = Text::new("测HElLo试OK\n", Point::new(0, 32), MULTI_STYLE)
        .draw(&mut disp)
        .unwrap();

    let _next = Text::new("字WoRlD体ok", _next, MULTI_STYLE)
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
        MULTI_STYLE,
        Alignment::Center,
        Baseline::Alphabetic,
    )
    .draw(&mut disp)
    .unwrap();
    rect.into_styled(RECT_STYLE).draw(&mut disp).unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        .theme(BinaryColorTheme::LcdGreen)
        .build();
    let mut win = Window::new("HelloWorld", &output_settings);

    win.update(&disp);

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
