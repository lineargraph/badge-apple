use anyhow::Result;
use badgemagic::{
    embedded_graphics::{self,
        geometry::Point, mono_font::MonoTextStyle, pixelcolor::BinaryColor, text::Text, image::{Image, ImageRaw},
    },
    protocol::{Mode, PayloadBuffer, Style, Speed},
    usb_hid::Device,
    util::DrawableLayoutExt,
};
use image::{ImageReader, ImageBuffer, DynamicImage, GrayImage};
fn load_image(path: &std::path::Path) -> anyhow::Result<GrayImage>{
    use image::open;
    let rgba = open(path).unwrap().into_rgba8();
    let gray = DynamicImage::ImageRgba8(rgba).resize_to_fill(44, 11, image::imageops::FilterType::CatmullRom).into_luma8();
    Ok(gray)
}

fn render_frames<T, F>(
    frames: &[T],
    mut should_render: F,
) -> Vec<u8>
where F : FnMut(&T, u32, u32) -> bool {
    // Fast mode for animations
    //
    // Will leave a 4 pixel gap between screens:
    // Place a 44x11 pixel screen every 48 pixels
    let frame_size = 48 / 8usize;
    let stride = frames.len() * frame_size;
    let mut data = vec![0u8; stride * 11 ];
    for (idx, frame) in frames.iter().enumerate() {
        for i in 0..11 {
            for j in 0..44 {
                let should_be_on = should_render(frame, j, i);
                if should_be_on {
                    data[i as usize * stride + j as usize / 8 + idx * frame_size] |= 1u8 << (7 - (j % 8));
                }
            }
        }
    }
    return data;
}

fn main() -> Result<()> {
    let mut payload = PayloadBuffer::new();
    let speed = Speed::Fps15;
    let framespeed = (60.0/15.0) as usize;
    let MAX_PAYLOAD_SIZE = 8192;
    let HEADER_SIZE = 64;
    let DRAWABLE_HEADER = 64;
    let FRAME_SIZE = 48 / 8 * 11;
    let MAX_FRAME_COUNT = (MAX_PAYLOAD_SIZE - HEADER_SIZE - DRAWABLE_HEADER - 3 * 64) / FRAME_SIZE;
    let frames = std::fs::read_dir(std::env::var("BAD_APPLE")?)?
        .skip(275)
        .step_by(framespeed)
        .map(|e| load_image(&e.unwrap().path()))
        .take(MAX_FRAME_COUNT).collect::<Result<Vec<_>>>()?;
    let data = render_frames(&frames, |frame, x,y|frame.get_pixel(x,y)[0]>0x70);
    // let frames = vec![1,0];
    // let data = render_frames(&frames, |frame, x,y|((frame+x+y)%2 == 0));
    let imgraw = ImageRaw::<BinaryColor>::new(&data[..], 48 * frames.len() as u32);
    let drawable = embedded_graphics::image::Image::new(
        &imgraw, Point::new(0,0));


    payload.add_message_drawable(Style::default().mode(Mode::Fast).speed(speed), &drawable);
    // payload.add_message_drawable(
    //     Style::default().mode(Mode::Left),
    //     &Text::new(
    //         "paid hugs - discounts available",
    //         Point::new(0, 8),
    //         MonoTextStyle::new(
    //             &embedded_graphics::mono_font::iso_8859_1::FONT_6X10,
    //             BinaryColor::On,
    //         ),
    //     ),
    // );
    Device::single()?.write(payload)?;

    Ok(())
}
