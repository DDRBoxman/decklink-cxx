use show_image::{create_window, event, ImageInfo, ImageView};

#[show_image::main]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = create_window("image", Default::default())?;

    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator
        .next()
        .expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());

    let mut input = device.get_input();
    input.enable_video_input(
        decklink_cxx::BMDDisplayMode::bmdModeHD1080p6000,
        decklink_cxx::BMDPixelFormat::bmdFormat8BitYUV,
        0,
    );

    let callback = |frame: decklink_cxx::DecklinkInputVideoFrame| {
        let pixels = decklink_cxx::DecklinkVideoFrameShared::get_bytes(&frame);
        let image = ImageView::new(ImageInfo::rgba8(1920, 1080), pixels);
        window.set_image("image-001", image);
    };
    input.set_callback(callback);

    input.start_streams();

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    input.stop_streams();

    Ok(())
}
