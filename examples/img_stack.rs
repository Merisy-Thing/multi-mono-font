use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use multi_mono_font::{MonoImageStack, MonoRleImage};

#[macro_export]
macro_rules! rgb565 {
    ($rgb:expr) => {
        Rgb565::new(
            (($rgb >> (3 + 16)) & 0xFF) as u8,
            (($rgb >> (2 + 8)) & 0xFF) as u8,
            (($rgb >> 3) & 0xFF) as u8,
        )
    };
}

// RLE 压缩率: 19.93%, 压缩前: 4350.00B, 压缩后: 867B
const RLE_IMG_FERRIS_BODY: MonoRleImage<Rgb565> = MonoRleImage::new_with_background_color(
    include_bytes!("imgs/Ferris_body.bin"),
    240,
    145,
    Rgb565::RED,
    Rgb565::WHITE,
);
// RLE 压缩率: 4.32%, 压缩前: 4350.00B, 压缩后: 188B
const RLE_IMG_FERRIS_EYE: MonoRleImage<Rgb565> = MonoRleImage::new(
    include_bytes!("imgs/Ferris_eye.bin"),
    240,
    145,
    Rgb565::BLACK,
);
// RLE 压缩率: 4.26%, 压缩前: 4350.00B, 压缩后: 186B
const RLE_IMG_FERRIS_FOOT: MonoRleImage<Rgb565> = MonoRleImage::new(
    include_bytes!("imgs/Ferris_foot.bin"),
    240,
    145,
    Rgb565::new(0x85 >> 3, 0x14 >> 2, 0x02 >> 3),
);

const FERRIS_STACK: MonoImageStack<MonoRleImage<Rgb565>> = MonoImageStack::new(
    &[
        &RLE_IMG_FERRIS_BODY, // put background in the first image
        &RLE_IMG_FERRIS_EYE,
        &RLE_IMG_FERRIS_FOOT,
    ],
    Point::new(350, 10),
);

// RLE 压缩率: 16.40%, 数据长度: 1207B
const IMG_STACK_0XE43717: MonoRleImage<Rgb565> = MonoRleImage::new(
    include_bytes!("imgs/cuddlyferris_0xE43717.bin"),
    283,
    208,
    rgb565!(0xE43717),
)
.with_offset(17, 11);

// RLE 压缩率: 33.21%, 数据长度: 160B
const IMG_STACK_0X000000: MonoRleImage<Rgb565> = MonoRleImage::new(
    include_bytes!("imgs/cuddlyferris_0x000000.bin"),
    94,
    41,
    rgb565!(0x000000),
)
.with_offset(115, 105);

// RLE 压缩率: 19.79%, 数据长度: 433B
const IMG_STACK_0X8E1E19: MonoRleImage<Rgb565> = MonoRleImage::new(
    include_bytes!("imgs/cuddlyferris_0x8E1E19.bin"),
    224,
    78,
    rgb565!(0x8E1E19),
)
.with_offset(47, 105);

// 总数据长度: 1800B

const CUDDLYFERRIS_STACK: MonoImageStack<MonoRleImage<Rgb565>> =
    MonoImageStack::new_with_background_color(
        &[
            &IMG_STACK_0XE43717,
            &IMG_STACK_0X000000,
            &IMG_STACK_0X8E1E19,
        ],
        Rectangle::new(Point::new(10, 10), Size::new(320, 240)),
        rgb565!(0xffffff),
    );

fn main() -> Result<(), core::convert::Infallible> {
    simple_logger::init().ok();
    let mut disp = SimulatorDisplay::<Rgb565>::new(Size::new(340 * 2, 260));

    CUDDLYFERRIS_STACK.draw(&mut disp).unwrap();

    FERRIS_STACK.draw(&mut disp).unwrap();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
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
