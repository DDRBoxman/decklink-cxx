/*mod bridge;

use bridge::decklink_ffi;

use crate::bridge::decklink_ffi::{
    new_output_callback, FillBlue, GetDisplayName, GetInput, GetOutput, IDeckLink,
    IDeckLinkIterator, IDeckLinkVideoFrame,
};
use std::{pin::Pin, ptr::null_mut, thread, time};

fn main() {
    unsafe {
        let mut input: *mut decklink_ffi::IDeckLinkInput = std::ptr::null_mut();
        let input_ptr: *mut *mut decklink_ffi::IDeckLinkInput = &mut input;
        let result = GetInput(device, input_ptr);
        println!("{}", result);
        println!("{:p}", input);

        let mut pin: Pin<&mut decklink_ffi::IDeckLinkInput> =
            Pin::new_unchecked(input.as_mut().unwrap());

        let mut display_mode_iterator: *mut decklink_ffi::IDeckLinkDisplayModeIterator =
            std::ptr::null_mut();
        let display_mode_iterator_ptr: *mut *mut decklink_ffi::IDeckLinkDisplayModeIterator =
            &mut display_mode_iterator;
        pin.as_mut()
            .GetDisplayModeIterator(display_mode_iterator_ptr);

        let mut display_mode_iterator_pin: Pin<&mut decklink_ffi::IDeckLinkDisplayModeIterator> =
            Pin::new_unchecked(display_mode_iterator.as_mut().unwrap());

        loop {
            let mut display_mode: *mut decklink_ffi::IDeckLinkDisplayMode = std::ptr::null_mut();
            let display_mode_ptr: *mut *mut decklink_ffi::IDeckLinkDisplayMode = &mut display_mode;
            display_mode_iterator_pin.as_mut().Next(display_mode_ptr);

            if display_mode == null_mut() {
                break;
            }

            let mut display_mode_pin: Pin<&mut decklink_ffi::IDeckLinkDisplayMode> =
                Pin::new_unchecked(display_mode.as_mut().unwrap());

            let name = decklink_ffi::GetDisplayModeName(display_mode);
            println!(
                "{} {} {}x{}",
                name,
                display_mode_pin.as_mut().GetDisplayMode(),
                display_mode_pin.as_mut().GetWidth(),
                display_mode_pin.GetHeight()
            );
        }

        let mut rust_callback = crate::bridge::RustInputCallback {};
        let input_callback = decklink_ffi::new_input_callback(
            &mut rust_callback as *mut crate::bridge::RustInputCallback,
        );
        pin.as_mut()
            .SetCallback(input_callback as *mut decklink_ffi::IDeckLinkInputCallback);

        // 1080p60  bmdFormat8BitBGRA
        pin.as_mut().EnableVideoInput(1215313456, 0x42475241, 0);

        pin.as_mut().StartStreams();

 


        let mut rust_callback = crate::bridge::RustOutputCallback {};
        let output_callback =
            new_output_callback(&mut rust_callback as *mut crate::bridge::RustOutputCallback);
        pin.as_mut().SetScheduledFrameCompletionCallback(
            output_callback as *mut decklink_ffi::IDeckLinkVideoOutputCallback,
        );


 

        let result = pin.as_mut().StartScheduledPlayback(0, 25000, 1.0);
        println!("{}", result);

        let onesec = time::Duration::from_millis(1000);

        thread::sleep(onesec);

        pin.as_mut()
            .StopScheduledPlayback(0, std::ptr::null_mut(), 25000);

        thread::sleep(onesec);
    }
}*/


/*
const uint32_t kFrameDuration = 1000;
const uint32_t kTimeScale = 25000;
const uint32_t kFrameWidth = 1920;
const uint32_t kFrameHeight = 1080;

// 10-bit YUV row bytes, ref. SDK Manual "2.7.4 Pixel Formats" / bmdFormat10BitYUV
const uint32_t kRowBytes = ((kFrameWidth + 47) / 48) * 128;

// 10-bit YUV colour pixels
const uint32_t kBlueData[] = { 0x40aa298, 0x2a8a62a8, 0x298aa040, 0x2a8102a8 };

    bmdFormat10BitYUV                                            = /* 'v210' */ 0x76323130,

*/
