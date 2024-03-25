use std::{thread, time};

use decklink_cxx;

fn main() {
    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator
        .next()
        .expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());

    let mut output = device.get_output();
    output.enable_video_output(decklink_cxx::BMDDisplayMode::bmdModeHD1080p6000, 0);

    for i in 0..20 {
        let res =
            output.create_video_frame(1920, 1080, decklink_cxx::BMDPixelFormat::bmdFormat10BitYUV);

        match res {
            Ok(frame) => {
                frame.fill_blue();
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
