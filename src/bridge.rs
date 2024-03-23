use std::pin::Pin;


mod decklink_type_wrappers {
    use std::fmt;

    #[derive(Debug, Eq, Clone, PartialEq, Hash)]
    #[allow(non_camel_case_types)]
    #[repr(transparent)]
    pub struct c_long(pub ::std::os::raw::c_long);

    impl fmt::Display for c_long {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

unsafe impl cxx::ExternType for decklink_type_wrappers::c_long {
    type Id = cxx::type_id!("c_long");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
pub mod decklink_ffi {
    extern "Rust" {
        type RustInputCallback;
        fn video_input_format_changed(self: &RustInputCallback);
        unsafe fn video_input_frame_arrived(
            self: &RustInputCallback,
            videoFrame: *mut IDeckLinkVideoInputFrame,
        );

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
        type IDeckLinkVideoInputFrame;

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
        type IDeckLinkVideoFrameAncillaryPackets;
        type IDeckLinkAncillaryPacket;
        type IDeckLinkAncillaryPacketIterator;

        unsafe fn GetPacketIterator(self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>, iterator: *mut *mut IDeckLinkAncillaryPacketIterator) -> i32;
        unsafe fn AttachPacket(self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>, packet: *mut IDeckLinkAncillaryPacket) -> i32;
        unsafe fn DetachPacket(self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>, packet: *mut IDeckLinkAncillaryPacket) -> i32;
        fn DetachAllPackets(self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>) -> i32;


        fn GetWidth(self: Pin<&mut IDeckLinkVideoFrame>) -> c_long;
        fn GetHeight(self: Pin<&mut IDeckLinkVideoFrame>) -> c_long;
        fn GetRowBytes(self: Pin<&mut IDeckLinkVideoFrame>) -> c_long;

        //fn GetBytes (self: Pin<&mut IDeckLinkVideoFrame>, *mut *mut void) -> i32;

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

        unsafe fn GetAncillaryPackets(videoFrame: *mut IDeckLinkVideoFrame, videoFrameAncillaryPackets: *mut *mut IDeckLinkVideoFrameAncillaryPackets) -> i32;

        unsafe fn FillBlue(frame: *mut IDeckLinkMutableVideoFrame);

        unsafe fn Release(obj: *mut IUnknown);

        type c_long = crate::bridge::decklink_type_wrappers::c_long;
    }
}

pub struct RustInputCallback {}

impl RustInputCallback {
    fn video_input_format_changed(self: &RustInputCallback) {
        println!("FORMAT CHANGED");
    }

    fn video_input_frame_arrived(
        self: &RustInputCallback,
        video_frame: *mut decklink_ffi::IDeckLinkVideoInputFrame,
    ) {
        println!("NEW FRAME");

        unsafe {
            let mut pin: Pin<&mut decklink_ffi::IDeckLinkVideoFrame> = Pin::new_unchecked(
                (video_frame as *mut decklink_ffi::IDeckLinkVideoFrame)
                    .as_mut()
                    .unwrap(),
            );

            println!("{}", pin.as_mut().GetRowBytes());
        }
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
