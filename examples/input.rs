use std::{thread, time};

fn main() {
    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator
        .next()
        .expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());

    let mut input = device.get_input();
    input.enable_video_input(
        decklink_cxx::BMDDisplayMode::bmdModeHD1080p6000,
        decklink_cxx::BMDPixelFormat::bmdFormat10BitYUV,
        0,
    );

    input.set_callback();

    input.start_streams();

    let onesec = time::Duration::from_millis(1000);
    thread::sleep(onesec);

    input.stop_streams();
}
