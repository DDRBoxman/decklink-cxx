use decklink_cxx::{self, DecklinkMutableVideoFrame};
use image;
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

fn load_image(path: &str) -> Vec<u8> {
    let im = image::open(path).expect("File not found!").to_rgba8();
    let mut image_vec: Vec<u8> = im.into_raw();

    for i in (0..image_vec.len()).step_by(4) {
        let b = image_vec[i + 2];
        image_vec[i + 2] = image_vec[i];
        image_vec[i] = b;
    }

    return image_vec;
}

fn main() {
    let mut displayed_frames = 0;
    let num_frames = 10;
    let mut output_frames: Vec<DecklinkMutableVideoFrame> = Vec::new();

    let image_vec = load_image("examples/assets/miku.png");
    let test_card = load_image("examples/assets/test_card.png");

    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator
        .next()
        .expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());

    let mut output = device.get_output();
    output.enable_video_output(decklink_cxx::BMDDisplayMode::bmdModeHD1080p6000, 0);

    for i in 0..num_frames {
        let res = output.create_video_frame(
            1920,
            1080,
            1920 * 4,
            decklink_cxx::BMDPixelFormat::bmdFormat8BitBGRA,
        );

        match res {
            Ok(frame) => {
                frame.copy_from_slice(&test_card);
                output_frames.push(frame);
            }
            Err(_) => todo!(),
        }
    }

    for i in 0..3 {
        let next_frame = displayed_frames % num_frames;

        println!("Scheduling frame {}", next_frame);

        let out_frame = &output_frames[next_frame];

        output.schedule_video_frame(&out_frame, displayed_frames as i64 * 1000, 1000, 25000);
        displayed_frames += 1
    }

    let output = Arc::new(Mutex::new(output));

    let o1 = output.clone();

    let callback = move || {
        let next_frame = displayed_frames % num_frames;

        println!("Scheduling frame {}", next_frame);

        let out_frame = &output_frames[next_frame];

        out_frame.copy_from_slice(&image_vec);
        o1.lock().unwrap().schedule_video_frame(
            &out_frame,
            displayed_frames as i64 * 1000,
            1000,
            25000,
        );
        displayed_frames += 1;
    };

    output
        .lock()
        .unwrap()
        .set_scheduled_frame_completion_callback(callback);

    output
        .lock()
        .unwrap()
        .start_scheduled_playback(0, 25000, 1.0);

    println!("Started!");

    let onesec = time::Duration::from_millis(1000);
    thread::sleep(onesec);

    println!("Stopping");
    output.lock().unwrap().stop_scheduled_playback(0, 25000);
}
