use decklink_cxx;
use image;
use std::{thread, time};

fn main() {
    let im = image::open("examples/assets/miku.png")
        .expect("File not found!")
        .to_rgba8();
    let mut image_vec: Vec<u8> = im.into_raw();

    for i in (0..image_vec.len()).step_by(4) {
        let b = image_vec[i + 2];
        image_vec[i + 2] = image_vec[i];
        image_vec[i] = b;
    }

    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator
        .next()
        .expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());

    let mut output = device.get_output();
    output.enable_video_output(decklink_cxx::BMDDisplayMode::bmdModeHD1080p6000, 0);

    let callback = |frame| {
        println!("FRAME");
    };
    output.set_scheduled_frame_completion_callback(callback);

    for i in 0..20 {
        let res = output.create_video_frame(
            1920,
            1080,
            1920 * 4,
            decklink_cxx::BMDPixelFormat::bmdFormat8BitBGRA,
        );

        match res {
            Ok(frame) => {
                frame.copy_from_slice(&image_vec);
                output.schedule_video_frame(frame, i * 1000, 1000, 25000);
            }
            Err(_) => todo!(),
        }
    }

    output.start_scheduled_playback(0, 25000, 1.0);

    let onesec = time::Duration::from_millis(1000);
    thread::sleep(onesec);

    output.stop_scheduled_playback(0, 25000);
}
