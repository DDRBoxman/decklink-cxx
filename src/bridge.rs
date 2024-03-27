use std::pin::Pin;

macro_rules! ctype_wrapper {
    ($r:ident, $c:expr) => {
        /// Newtype wrapper for a `$c`
        #[derive(Debug, Eq, Clone, PartialEq, Hash)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub ::std::os::raw::$r);

        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($c);
            type Kind = cxx::kind::Trivial;
        }
    };
}

macro_rules! bridge_type_wrapper {
    ($r:ident, $c:expr, $t:ident) => {
        /// Newtype wrapper for a `$c`
        #[derive(Debug, Eq, Clone, PartialEq, Hash)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $t);

        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($c);
            type Kind = cxx::kind::Trivial;
        }
    };
}

pub(crate) mod decklink_type_wrappers {
    use std::fmt;

    ctype_wrapper!(c_long, "c_long");
    ctype_wrapper!(c_ulong, "c_ulong");
    ctype_wrapper!(c_longlong, "c_longlong");

    impl fmt::Display for c_long {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    bridge_type_wrapper!(c_hresult, "c_hresult", i32);

    bridge_type_wrapper!(
        c_BMDDeckLinkAPIInformationID,
        "c_BMDDeckLinkAPIInformationID",
        u32
    );
    bridge_type_wrapper!(c_BMDDisplayMode, "c_BMDDisplayMode", u32);
    bridge_type_wrapper!(c_BMDPixelFormat, "c_BMDPixelFormat", u32);
    bridge_type_wrapper!(c_BMDVideoInputFlags, "c_BMDVideoInputFlags", u32);
    bridge_type_wrapper!(c_BMDVideoOutputFlags, "c_BMDVideoOutputFlags", u32);
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

    enum _BMDPixelFormat {
        bmdFormatUnspecified = 0,
        bmdFormat8BitYUV = 0x32767579,
        bmdFormat10BitYUV = 0x76323130,
        bmdFormat8BitARGB = 32,
        bmdFormat8BitBGRA = 0x42475241,
        bmdFormat10BitRGB = 0x72323130, // Big-endian RGB 10-bit per component with SMPTE video levels (64-940). Packed as 2:10:10:10
        bmdFormat12BitRGB = 0x52313242, // Big-endian RGB 12-bit per component with full range (0-4095). Packed as 12-bit per component
        bmdFormat12BitRGBLE = 0x5231324C, // Little-endian RGB 12-bit per component with full range (0-4095). Packed as 12-bit per component
        bmdFormat10BitRGBXLE = 0x5231306C, // Little-endian 10-bit RGB with SMPTE video levels (64-940)
        bmdFormat10BitRGBX = 0x52313062,   // Big-endian 10-bit RGB with SMPTE video levels (64-940)
        bmdFormatH265 = 0x68657631,        // High Efficiency Video Coding (HEVC/h.265)

        /* AVID DNxHR */
        bmdFormatDNxHR = 0x41566468,
    }

    enum _BMDDisplayMode {
        /* SD Modes */
        bmdModeNTSC = 0x6E747363,
        bmdModeNTSC2398 = 0x6E743233, // 3:2 pulldown
        bmdModePAL = 0x70616C20,
        bmdModeNTSCp = 0x6E747370,
        bmdModePALp = 0x70616C70,

        /* HD 1080 Modes */
        bmdModeHD1080p2398 = 0x32337073,
        bmdModeHD1080p24 = 0x32347073,
        bmdModeHD1080p25 = 0x48703235,
        bmdModeHD1080p2997 = 0x48703239,
        bmdModeHD1080p30 = 0x48703330,
        bmdModeHD1080p4795 = 0x48703437,
        bmdModeHD1080p48 = 0x48703438,
        bmdModeHD1080p50 = 0x48703530,
        bmdModeHD1080p5994 = 0x48703539,
        bmdModeHD1080p6000 = 0x48703630, // N.B. This _really_ is 60.00 Hz.
        bmdModeHD1080p9590 = 0x48703935,
        bmdModeHD1080p96 = 0x48703936,
        bmdModeHD1080p100 = 0x48703130,
        bmdModeHD1080p11988 = 0x48703131,
        bmdModeHD1080p120 = 0x48703132,
        bmdModeHD1080i50 = 0x48693530,
        bmdModeHD1080i5994 = 0x48693539,
        bmdModeHD1080i6000 = 0x48693630, // N.B. This _really_ is 60.00 Hz.

        /* HD 720 Modes */
        bmdModeHD720p50 = 0x68703530,
        bmdModeHD720p5994 = 0x68703539,
        bmdModeHD720p60 = 0x68703630,

        /* 2K Modes */
        bmdMode2k2398 = 0x326B3233,
        bmdMode2k24 = 0x326B3234,
        bmdMode2k25 = 0x326B3235,

        /* 2K DCI Modes */
        bmdMode2kDCI2398 = 0x32643233,
        bmdMode2kDCI24 = 0x32643234,
        bmdMode2kDCI25 = 0x32643235,
        bmdMode2kDCI2997 = 0x32643239,
        bmdMode2kDCI30 = 0x32643330,
        bmdMode2kDCI4795 = 0x32643437,
        bmdMode2kDCI48 = 0x32643438,
        bmdMode2kDCI50 = 0x32643530,
        bmdMode2kDCI5994 = 0x32643539,
        bmdMode2kDCI60 = 0x32643630,
        bmdMode2kDCI9590 = 0x32643935,
        bmdMode2kDCI96 = 0x32643936,
        bmdMode2kDCI100 = 0x32643130,
        bmdMode2kDCI11988 = 0x32643131,
        bmdMode2kDCI120 = 0x32643132,

        /* 4K UHD Modes */
        bmdMode4K2160p2398 = 0x346B3233,
        bmdMode4K2160p24 = 0x346B3234,
        bmdMode4K2160p25 = 0x346B3235,
        bmdMode4K2160p2997 = 0x346B3239,
        bmdMode4K2160p30 = 0x346B3330,
        bmdMode4K2160p4795 = 0x346B3437,
        bmdMode4K2160p48 = 0x346B3438,
        bmdMode4K2160p50 = 0x346B3530,
        bmdMode4K2160p5994 = 0x346B3539,
        bmdMode4K2160p60 = 0x346B3630,
        bmdMode4K2160p9590 = 0x346B3935,
        bmdMode4K2160p96 = 0x346B3936,
        bmdMode4K2160p100 = 0x346B3130,
        bmdMode4K2160p11988 = 0x346B3131,
        bmdMode4K2160p120 = 0x346B3132,

        /* 4K DCI Modes */
        bmdMode4kDCI2398 = 0x34643233,
        bmdMode4kDCI24 = 0x34643234,
        bmdMode4kDCI25 = 0x34643235,
        bmdMode4kDCI2997 = 0x34643239,
        bmdMode4kDCI30 = 0x34643330,
        bmdMode4kDCI4795 = 0x34643437,
        bmdMode4kDCI48 = 0x34643438,
        bmdMode4kDCI50 = 0x34643530,
        bmdMode4kDCI5994 = 0x34643539,
        bmdMode4kDCI60 = 0x34643630,
        bmdMode4kDCI9590 = 0x34643935,
        bmdMode4kDCI96 = 0x34643936,
        bmdMode4kDCI100 = 0x34643130,
        bmdMode4kDCI11988 = 0x34643131,
        bmdMode4kDCI120 = 0x34643132,

        /* 8K UHD Modes */
        bmdMode8K4320p2398 = 0x386B3233,
        bmdMode8K4320p24 = 0x386B3234,
        bmdMode8K4320p25 = 0x386B3235,
        bmdMode8K4320p2997 = 0x386B3239,
        bmdMode8K4320p30 = 0x386B3330,
        bmdMode8K4320p4795 = 0x386B3437,
        bmdMode8K4320p48 = 0x386B3438,
        bmdMode8K4320p50 = 0x386B3530,
        bmdMode8K4320p5994 = 0x386B3539,
        bmdMode8K4320p60 = 0x386B3630,

        /* 8K DCI Modes */
        bmdMode8kDCI2398 = 0x38643233,
        bmdMode8kDCI24 = 0x38643234,
        bmdMode8kDCI25 = 0x38643235,
        bmdMode8kDCI2997 = 0x38643239,
        bmdMode8kDCI30 = 0x38643330,
        bmdMode8kDCI4795 = 0x38643437,
        bmdMode8kDCI48 = 0x38643438,
        bmdMode8kDCI50 = 0x38643530,
        bmdMode8kDCI5994 = 0x38643539,
        bmdMode8kDCI60 = 0x38643630,

        /* PC Modes */
        bmdMode640x480p60 = 0x76676136,
        bmdMode800x600p60 = 0x73766736,
        bmdMode1440x900p50 = 0x77786735,
        bmdMode1440x900p60 = 0x77786736,
        bmdMode1440x1080p50 = 0x73786735,
        bmdMode1440x1080p60 = 0x73786736,
        bmdMode1600x1200p50 = 0x75786735,
        bmdMode1600x1200p60 = 0x75786736,
        bmdMode1920x1200p50 = 0x77757835,
        bmdMode1920x1200p60 = 0x77757836,
        bmdMode1920x1440p50 = 0x31393435,
        bmdMode1920x1440p60 = 0x31393436,
        bmdMode2560x1440p50 = 0x77716835,
        bmdMode2560x1440p60 = 0x77716836,
        bmdMode2560x1600p50 = 0x77717835,
        bmdMode2560x1600p60 = 0x77717836,

        /* Special Modes */
        bmdModeUnknown = 0x69756E6B,
    }

    unsafe extern "C++" {
        include!("decklink-cxx/include/platform.h");

        type IDeckLinkAPIInformation;

        type _BMDDeckLinkAPIInformationID;
        type BMDDeckLinkAPIInformationID;
        type _BMDPixelFormat;
        type BMDPixelFormat;
        type _BMDDisplayMode;
        type BMDDisplayMode;

        fn CreateDeckLinkAPIInformationInstance() -> *mut IDeckLinkAPIInformation;

        unsafe fn GetInt(
            self: Pin<&mut IDeckLinkAPIInformation>,
            id: c_BMDDeckLinkAPIInformationID,
            value: *mut c_longlong,
        ) -> c_hresult;
        fn Release(self: Pin<&mut IDeckLinkAPIInformation>) -> c_ulong;

        type IDeckLinkIterator;

        type IDeckLink;

        fn Release(self: Pin<&mut IDeckLink>) -> c_ulong;

        fn CreateDeckLinkIteratorInstance() -> *mut IDeckLinkIterator;

        unsafe fn Next(
            self: Pin<&mut IDeckLinkIterator>,
            deckLinkInstance: *mut *mut IDeckLink,
        ) -> c_hresult;

        fn Release(self: Pin<&mut IDeckLinkIterator>) -> c_ulong;

        type IDeckLinkInput;
        type IDeckLinkDisplayModeIterator;
        type IDeckLinkDisplayMode;
        type IDeckLinkInputCallback;
        type IDeckLinkVideoInputFrame;

        unsafe fn GetDisplayModeIterator(
            self: Pin<&mut IDeckLinkInput>,
            iterator: *mut *mut IDeckLinkDisplayModeIterator,
        ) -> c_hresult;

        unsafe fn Next(
            self: Pin<&mut IDeckLinkDisplayModeIterator>,
            deckLinkDisplayMode: *mut *mut IDeckLinkDisplayMode,
        ) -> c_hresult;

        fn Release(self: Pin<&mut IDeckLinkDisplayModeIterator>) -> c_ulong;

        fn GetDisplayMode(self: Pin<&mut IDeckLinkDisplayMode>) -> c_BMDDisplayMode;
        fn GetWidth(self: Pin<&mut IDeckLinkDisplayMode>) -> c_long;
        fn GetHeight(self: Pin<&mut IDeckLinkDisplayMode>) -> c_long;
        unsafe fn GetFrameRate(
            self: Pin<&mut IDeckLinkDisplayMode>,
            frameDuration: *mut c_longlong,
            timeScale: *mut c_longlong,
        ) -> c_hresult;
        fn Release(self: Pin<&mut IDeckLinkDisplayMode>) -> c_ulong;

        /*  unsafe fn DoesSupportVideoMode(
            self: Pin<&mut IDeckLinkInput>,
            connection: u32,
            requestMode: u32,
            requestedPixelFormat: c_BMDPixelFormat,
            conversionMode: u32,
            flags: u32,
            actualMode: *mut u32,
            supported: *mut bool,
        ) -> c_hresult;*/
        fn EnableVideoInput(
            self: Pin<&mut IDeckLinkInput>,
            displayMode: c_BMDDisplayMode,
            pixelFormat: c_BMDPixelFormat,
            flags: c_BMDVideoInputFlags,
        ) -> c_hresult;
        fn StartStreams(self: Pin<&mut IDeckLinkInput>) -> c_hresult;
        fn StopStreams(self: Pin<&mut IDeckLinkInput>) -> c_hresult;
        fn Release(self: Pin<&mut IDeckLinkInput>) -> c_ulong;

        unsafe fn SetCallback(
            self: Pin<&mut IDeckLinkInput>,
            callback: *mut IDeckLinkInputCallback,
        ) -> c_hresult;

        type IDeckLinkOutput;
        type IDeckLinkMutableVideoFrame;
        type IDeckLinkVideoFrame;
        type IDeckLinkVideoOutputCallback;
        type IDeckLinkVideoFrameAncillaryPackets;
        type IDeckLinkAncillaryPacket;
        type IDeckLinkAncillaryPacketIterator;

        unsafe fn GetPacketIterator(
            self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>,
            iterator: *mut *mut IDeckLinkAncillaryPacketIterator,
        ) -> c_hresult;
        unsafe fn AttachPacket(
            self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>,
            packet: *mut IDeckLinkAncillaryPacket,
        ) -> c_hresult;
        unsafe fn DetachPacket(
            self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>,
            packet: *mut IDeckLinkAncillaryPacket,
        ) -> c_hresult;
        fn DetachAllPackets(self: Pin<&mut IDeckLinkVideoFrameAncillaryPackets>) -> c_hresult;

        fn GetWidth(self: Pin<&mut IDeckLinkVideoFrame>) -> c_long;
        fn GetHeight(self: Pin<&mut IDeckLinkVideoFrame>) -> c_long;
        fn GetRowBytes(self: Pin<&mut IDeckLinkVideoFrame>) -> c_long;

        //fn GetBytes (self: Pin<&mut IDeckLinkVideoFrame>, *mut *mut void) -> c_hresult;

        unsafe fn GetDisplayModeIterator(
            self: Pin<&mut IDeckLinkOutput>,
            iterator: *mut *mut IDeckLinkDisplayModeIterator,
        ) -> c_hresult;

        fn EnableVideoOutput(
            self: Pin<&mut IDeckLinkOutput>,
            displayMode: c_BMDDisplayMode,
            outputFlags: c_BMDVideoOutputFlags,
        ) -> c_hresult;
        fn StartScheduledPlayback(
            self: Pin<&mut IDeckLinkOutput>,
            playbackStartTime: c_longlong,
            timeScale: c_longlong,
            playbackSpeed: f64,
        ) -> c_hresult;
        unsafe fn StopScheduledPlayback(
            self: Pin<&mut IDeckLinkOutput>,
            stopPlaybackAtTime: c_longlong,
            actualStopTime: *mut c_longlong,
            timeScale: i64,
        ) -> c_hresult;

        unsafe fn ScheduleVideoFrame(
            self: Pin<&mut IDeckLinkOutput>,
            frame: *mut IDeckLinkVideoFrame,
            displayTime: c_longlong,
            displayDuration: c_longlong,
            timeScale: i64,
        ) -> c_hresult;
        unsafe fn CreateVideoFrame(
            self: Pin<&mut IDeckLinkOutput>,
            width: i32,
            height: i32,
            row_bytes: i32,
            pixel_format: c_BMDPixelFormat,
            flags: u32,
            frame: *mut *mut IDeckLinkMutableVideoFrame,
        ) -> c_hresult;

        unsafe fn SetScheduledFrameCompletionCallback(
            self: Pin<&mut IDeckLinkOutput>,
            output: *mut IDeckLinkVideoOutputCallback,
        ) -> c_hresult;

        fn Release(self: Pin<&mut IDeckLinkOutput>) -> c_ulong;

        type IUnknown;

        fn Release(self: Pin<&mut IUnknown>) -> c_ulong;

        type CXXInputCallback;

        type CXXOutputCallback;

        include!("decklink-cxx/include/test.h");

        include!("decklink-cxx/include/callback.h");

        include!("decklink-cxx/include/bridge.h");

        unsafe fn new_input_callback(callback: *mut RustInputCallback) -> *mut CXXInputCallback;

        unsafe fn new_output_callback(callback: *mut RustOutputCallback) -> *mut CXXOutputCallback;

        unsafe fn GetDisplayName(decklink: *mut IDeckLink) -> String;

        unsafe fn GetDisplayModeName(displayMode: *mut IDeckLinkDisplayMode) -> String;

        unsafe fn GetInput(decklink: *mut IDeckLink, input: *mut *mut IDeckLinkInput) -> c_hresult;

        unsafe fn GetOutput(
            decklink: *mut IDeckLink,
            output: *mut *mut IDeckLinkOutput,
        ) -> c_hresult;

        unsafe fn GetAncillaryPackets(
            videoFrame: *mut IDeckLinkVideoFrame,
            videoFrameAncillaryPackets: *mut *mut IDeckLinkVideoFrameAncillaryPackets,
        ) -> c_hresult;

        unsafe fn FillBlue(frame: *mut IDeckLinkMutableVideoFrame);

        unsafe fn Release(obj: *mut IUnknown);

        type c_long = crate::bridge::decklink_type_wrappers::c_long;
        type c_ulong = crate::bridge::decklink_type_wrappers::c_ulong;
        type c_longlong = crate::bridge::decklink_type_wrappers::c_longlong;
        type c_hresult = crate::bridge::decklink_type_wrappers::c_hresult;
        type c_BMDDeckLinkAPIInformationID =
            crate::bridge::decklink_type_wrappers::c_BMDDeckLinkAPIInformationID;
        type c_BMDDisplayMode = crate::bridge::decklink_type_wrappers::c_BMDDisplayMode;
        type c_BMDPixelFormat = crate::bridge::decklink_type_wrappers::c_BMDPixelFormat;
        type c_BMDVideoInputFlags = crate::bridge::decklink_type_wrappers::c_BMDVideoInputFlags;
        type c_BMDVideoOutputFlags = crate::bridge::decklink_type_wrappers::c_BMDVideoOutputFlags;
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
