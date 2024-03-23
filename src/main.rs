use crate::decklink_ffi::{
    new_output_callback, FillBlue, GetDisplayName, GetInput, GetOutput, IDeckLink,
    IDeckLinkIterator,
};
use std::{pin::Pin, process::Output, ptr::null_mut, thread, time};

mod decklink_type_wrappers {
    #[derive(Debug, Eq, Clone, PartialEq, Hash)]
    #[allow(non_camel_case_types)]
    #[repr(transparent)]
    pub struct c_long(pub ::std::os::raw::c_long);
}

unsafe impl cxx::ExternType for decklink_type_wrappers::c_long {
    type Id = cxx::type_id!("c_long");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod decklink_ffi {

    extern "Rust" {
        type RustInputCallback;
        fn video_input_format_changed(self: &RustInputCallback);
        fn video_input_frame_arrived(self: &RustInputCallback);

        type RustOutputCallback;
        fn scheduled_frame_completed(self: &RustOutputCallback);
        fn scheduled_playback_has_stopped(self: &RustOutputCallback);
    }

    enum _BMDDeckLinkAPIInformationID {
        BMDDeckLinkAPIVersion = 0x76657273,
    }

    unsafe extern "C++" {
        include!("decklink-cxx/decklink/Mac/include/DeckLinkAPI.h");

        type IDeckLinkIterator;

        type IDeckLink;

        fn CreateDeckLinkIteratorInstance() -> *mut IDeckLinkIterator;

        unsafe fn Next(
            self: Pin<&mut IDeckLinkIterator>,
            deckLinkInstance: *mut *mut IDeckLink,
        ) -> i32;

        type IDeckLinkAPIInformation;

        type _BMDDeckLinkAPIInformationID;

        fn CreateDeckLinkAPIInformationInstance() -> *mut IDeckLinkAPIInformation;

        unsafe fn GetInt(self: Pin<&mut IDeckLinkAPIInformation>, id: u32, value: *mut i64) -> i32;

        type IDeckLinkInput;
        type IDeckLinkDisplayModeIterator;
        type IDeckLinkDisplayMode;
        type IDeckLinkInputCallback;

        unsafe fn GetDisplayModeIterator(
            self: Pin<&mut IDeckLinkInput>,
            iterator: *mut *mut IDeckLinkDisplayModeIterator,
        ) -> i32;

        unsafe fn Next(
            self: Pin<&mut IDeckLinkDisplayModeIterator>,
            deckLinkDisplayMode: *mut *mut IDeckLinkDisplayMode,
        ) -> i32;

        fn GetDisplayMode(self: Pin<&mut IDeckLinkDisplayMode>) -> u32;
        fn GetWidth(self: Pin<&mut IDeckLinkDisplayMode>) -> c_long;
        fn GetHeight(self: Pin<&mut IDeckLinkDisplayMode>) -> c_long;
        unsafe fn GetFrameRate(
            self: Pin<&mut IDeckLinkDisplayMode>,
            frameDuration: *mut i64,
            timeScale: *mut i64,
        ) -> i32;

        unsafe fn DoesSupportVideoMode(
            self: Pin<&mut IDeckLinkInput>,
            connection: u32,
            requestMode: u32,
            requestedPixelFormat: u32,
            conversionMode: u32,
            flags: u32,
            actualMode: *mut u32,
            supported: *mut bool,
        ) -> i32;
        fn EnableVideoInput(
            self: Pin<&mut IDeckLinkInput>,
            displayMode: u32,
            pixelFormat: u32,
            flags: u32,
        ) -> i32;
        fn StartStreams(self: Pin<&mut IDeckLinkInput>) -> i32;
        fn StopStreams(self: Pin<&mut IDeckLinkInput>) -> i32;

        unsafe fn SetCallback(
            self: Pin<&mut IDeckLinkInput>,
            callback: *mut IDeckLinkInputCallback,
        ) -> i32;

        type IDeckLinkOutput;
        type IDeckLinkMutableVideoFrame;
        type IDeckLinkVideoFrame;
        type IDeckLinkVideoOutputCallback;

        fn EnableVideoOutput(
            self: Pin<&mut IDeckLinkOutput>,
            displayMode: u32,
            outputFlags: u32,
        ) -> i32;
        fn StartScheduledPlayback(
            self: Pin<&mut IDeckLinkOutput>,
            playbackStartTime: i64,
            timeScale: i64,
            playbackSpeed: f64,
        ) -> i32;
        unsafe fn StopScheduledPlayback(
            self: Pin<&mut IDeckLinkOutput>,
            stopPlaybackAtTime: i64,
            actualStopTime: *mut i64,
            timeScale: i64,
        ) -> i32;

        unsafe fn ScheduleVideoFrame(
            self: Pin<&mut IDeckLinkOutput>,
            frame: *mut IDeckLinkVideoFrame,
            displayTime: i64,
            displayDuration: i64,
            timeScale: i64,
        ) -> i32;
        unsafe fn CreateVideoFrame(
            self: Pin<&mut IDeckLinkOutput>,
            width: i32,
            height: i32,
            row_bytes: i32,
            pixel_format: u32,
            flags: u32,
            frame: *mut *mut IDeckLinkMutableVideoFrame,
        ) -> i32;

        unsafe fn SetScheduledFrameCompletionCallback(
            self: Pin<&mut IDeckLinkOutput>,
            output: *mut IDeckLinkVideoOutputCallback,
        ) -> i32;

        type IUnknown;

        type CXXInputCallback;

        type CXXOutputCallback;

        include!("decklink-cxx/include/test.h");

        include!("decklink-cxx/include/callback.h");

        include!("decklink-cxx/include/bridge.h");

        unsafe fn new_input_callback(callback: *mut RustInputCallback) -> *mut CXXInputCallback;

        unsafe fn new_output_callback(callback: *mut RustOutputCallback) -> *mut CXXOutputCallback;

        unsafe fn GetDisplayName(decklink: *mut IDeckLink) -> String;

        unsafe fn GetDisplayModeName(displayMode: *mut IDeckLinkDisplayMode) -> String;

        unsafe fn GetInput(decklink: *mut IDeckLink, input: *mut *mut IDeckLinkInput) -> i32;

        unsafe fn GetOutput(decklink: *mut IDeckLink, output: *mut *mut IDeckLinkOutput) -> i32;

        unsafe fn FillBlue(frame: *mut IDeckLinkMutableVideoFrame);

        unsafe fn Release(obj: *mut IUnknown);

        type c_long = crate::decklink_type_wrappers::c_long;
    }
}

pub struct RustInputCallback {}

impl RustInputCallback {
    fn video_input_format_changed(self: &RustInputCallback) {
        println!("FORMAT CHANGED");
    }

    fn video_input_frame_arrived(self: &RustInputCallback) {
        println!("NEW FRAME");
    }
}

pub struct RustOutputCallback {}

impl RustOutputCallback {
    fn scheduled_frame_completed(self: &RustOutputCallback) {
        println!("COMPLETED");
    }

    fn scheduled_playback_has_stopped(self: &RustOutputCallback) {
        println!("STOPPED");
    }
}

fn main() {
    let api_info = decklink_ffi::CreateDeckLinkAPIInformationInstance();
    unsafe {
        let pin: Pin<&mut decklink_ffi::IDeckLinkAPIInformation> =
            Pin::new_unchecked(api_info.as_mut().unwrap());
        let mut val: i64 = 0;

        pin.GetInt(0x76657273, &mut val as *mut i64);
        println!("{}", val);
    }

    let iterator: *mut IDeckLinkIterator = decklink_ffi::CreateDeckLinkIteratorInstance();

    unsafe {
        let pin: Pin<&mut IDeckLinkIterator> = Pin::new_unchecked(iterator.as_mut().unwrap());

        let mut device: *mut IDeckLink = std::ptr::null_mut();
        let device_ptr: *mut *mut IDeckLink = &mut device;

        pin.Next(device_ptr);

        if device.is_null() {
            println!("No device found. Please install device or drivers");
            return;
        }

        let name = GetDisplayName(device);
        println!("{}", name);

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

            //println!("{:p}", display_mode);
            if display_mode == null_mut() {
                break;
            }

            let mut display_mode_pin: Pin<&mut decklink_ffi::IDeckLinkDisplayMode> =
                Pin::new_unchecked(display_mode.as_mut().unwrap());

            let name = decklink_ffi::GetDisplayModeName(display_mode);
            println!("{}", name);
            println!("{}", display_mode_pin.as_mut().GetDisplayMode());
            println!(
                "{:?}x{:?}",
                display_mode_pin.as_mut().GetWidth(),
                display_mode_pin.GetHeight()
            );
        }

        let mut rust_callback = RustInputCallback {};
        let input_callback =
            decklink_ffi::new_input_callback(&mut rust_callback as *mut RustInputCallback);
        pin.as_mut()
            .SetCallback(input_callback as *mut decklink_ffi::IDeckLinkInputCallback);

        // 1080p60  bmdFormat8BitBGRA
        pin.as_mut().EnableVideoInput(1215313456, 0x42475241, 0);

        pin.as_mut().StartStreams();

        let mut output: *mut decklink_ffi::IDeckLinkOutput = std::ptr::null_mut();
        let output_ptr: *mut *mut decklink_ffi::IDeckLinkOutput = &mut output;
        let result = GetOutput(device, output_ptr);
        println!("{}", result);
        println!("{:p}", output);

        let mut pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
            Pin::new_unchecked(output.as_mut().unwrap());

        let mut rust_callback = RustOutputCallback {};
        let output_callback = new_output_callback(&mut rust_callback as *mut RustOutputCallback);
        pin.as_mut().SetScheduledFrameCompletionCallback(
            output_callback as *mut decklink_ffi::IDeckLinkVideoOutputCallback,
        );

        pin.as_mut().EnableVideoOutput(0x48703630, 0);
        //println!("{}", output.);

        for i in 0..20 {
            let mut frame: *mut decklink_ffi::IDeckLinkMutableVideoFrame = std::ptr::null_mut();
            let frame_ptr: *mut *mut decklink_ffi::IDeckLinkMutableVideoFrame = &mut frame;
            let result = pin.as_mut().CreateVideoFrame(
                1920,
                1080,
                ((1920 + 47) / 48) * 128,
                0x76323130,
                0,
                frame_ptr,
            );
            //println!("{}", result);
            //println!("{:p}", frame);

            FillBlue(frame);

            let result = pin.as_mut().ScheduleVideoFrame(
                frame as *mut decklink_ffi::IDeckLinkVideoFrame,
                i * 1000,
                1000,
                25000,
            );
            println!("schedule {}", result);

            decklink_ffi::Release(frame as *mut decklink_ffi::IUnknown);
        }

        let result = pin.as_mut().StartScheduledPlayback(0, 25000, 1.0);
        println!("{}", result);

        let onesec = time::Duration::from_millis(1000);

        thread::sleep(onesec);

        pin.as_mut()
            .StopScheduledPlayback(0, std::ptr::null_mut(), 25000);

        thread::sleep(onesec);
    }
}

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
