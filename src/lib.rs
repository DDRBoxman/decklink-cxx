mod bridge;

use bridge::decklink_ffi;

use std::pin::Pin;

pub struct DecklinkAPIInformation<'a> {
    api_info: Pin<&'a mut decklink_ffi::IDeckLinkAPIInformation>,
}

impl DecklinkAPIInformation<'_> {
    pub fn new() -> Self {
        let api_info = decklink_ffi::CreateDeckLinkAPIInformationInstance();

        let pin: Pin<&mut decklink_ffi::IDeckLinkAPIInformation> =
            unsafe { Pin::new_unchecked(api_info.as_mut().unwrap()) };

        return DecklinkAPIInformation { api_info: pin };
    }

    pub fn get_version(&mut self) -> i64 {
        let mut val: i64 = 0;
        unsafe {
            self.api_info.as_mut().GetInt(
                decklink_ffi::_BMDDeckLinkAPIInformationID::BMDDeckLinkAPIVersion.repr,
                &mut val as *mut i64,
            );
        }
        return val;
    }
}

impl Drop for DecklinkAPIInformation<'_> {
    fn drop(&mut self) {
        self.api_info.as_mut().Release();
    }
}

pub struct DecklinkIterator<'a> {
    iterator: Pin<&'a mut decklink_ffi::IDeckLinkIterator>,
}

impl DecklinkIterator<'_> {
    pub fn new() -> Self {
        let iterator = decklink_ffi::CreateDeckLinkIteratorInstance();
        let pin: Pin<&mut decklink_ffi::IDeckLinkIterator> =
            unsafe { Pin::new_unchecked(iterator.as_mut().unwrap()) };

        return DecklinkIterator { iterator: pin };
    }

    pub fn next(&mut self) -> Option<DecklinkDevice> {
        let mut device: *mut decklink_ffi::IDeckLink = std::ptr::null_mut();
        let device_ptr: *mut *mut decklink_ffi::IDeckLink = &mut device;

        unsafe {
            self.iterator.as_mut().Next(device_ptr);
        }

        if device.is_null() {
            None
        } else {
            Some(DecklinkDevice {
                device: unsafe { Pin::new_unchecked(device.as_mut().unwrap()) },
                device_raw: device,
            })
        }
    }
}

impl Drop for DecklinkIterator<'_> {
    fn drop(&mut self) {
        self.iterator.as_mut().Release();
    }
}

pub struct DecklinkDevice<'a> {
    device: Pin<&'a mut decklink_ffi::IDeckLink>,
    device_raw: *mut decklink_ffi::IDeckLink,
}

impl DecklinkDevice<'_> {
    pub fn get_name(&self) -> String {
        unsafe { decklink_ffi::GetDisplayName(self.device_raw) }
    }

    pub fn get_output(&self) -> DecklinkOutput {
        let mut output: *mut decklink_ffi::IDeckLinkOutput = std::ptr::null_mut();
        let output_ptr: *mut *mut decklink_ffi::IDeckLinkOutput = &mut output;
        let result = unsafe { decklink_ffi::GetOutput(self.device_raw, output_ptr) };

        return DecklinkOutput {
            output: unsafe { Pin::new_unchecked(output.as_mut().unwrap()) },
        };
    }

    pub fn get_input(&self) -> DecklinkInput {
        let mut input: *mut decklink_ffi::IDeckLinkInput = std::ptr::null_mut();
        let input_ptr: *mut *mut decklink_ffi::IDeckLinkInput = &mut input;
        let result = unsafe { decklink_ffi::GetInput(self.device_raw, input_ptr) };

        return DecklinkInput {
            input: unsafe { Pin::new_unchecked(input.as_mut().unwrap()) },
        };
    }
}

impl Drop for DecklinkDevice<'_> {
    fn drop(&mut self) {
        self.device.as_mut().Release();
    }
}

pub struct DecklinkOutput<'a> {
    output: Pin<&'a mut decklink_ffi::IDeckLinkOutput>,
}

impl DecklinkOutput<'_> {
    pub fn enable_video_output(&mut self, display_mode: BMDDisplayMode, output_flags: u32) {
        self.output
            .as_mut()
            .EnableVideoOutput(display_mode.repr, output_flags);
    }

    pub fn create_video_frame(
        &mut self,
        width: i32,
        height: i32,
        pixel_format: BMDPixelFormat,
    ) -> Result<DecklinkVideoFrame, ()> {
        let mut frame: *mut decklink_ffi::IDeckLinkMutableVideoFrame = std::ptr::null_mut();
        let frame_ptr: *mut *mut decklink_ffi::IDeckLinkMutableVideoFrame = &mut frame;

        let result = unsafe {
            self.output.as_mut().CreateVideoFrame(
                width,
                height,
                ((width + 47) / 48) * 128, // todo: it's silly decklink passes this can we do it ourself?
                pixel_format.repr,
                0,
                frame_ptr,
            )
        };

        Ok(DecklinkVideoFrame { frame })
    }

    pub fn schedule_video_frame(
        &mut self,
        frame: DecklinkVideoFrame,
        display_time: i64,
        display_duration: i64,
        time_scale: i64,
    ) {
        let result = unsafe {
            self.output.as_mut().ScheduleVideoFrame(
                frame.frame as *mut decklink_ffi::IDeckLinkVideoFrame,
                display_time,
                display_duration,
                time_scale,
            )
        };
    }

    pub fn start_scheduled_playback(
        &mut self,
        playback_start_time: i64,
        time_scale: i64,
        playback_speed: f64,
    ) {
        self.output.as_mut().StartScheduledPlayback(
            playback_start_time,
            time_scale,
            playback_speed,
        );
    }

    pub fn stop_scheduled_playback(&mut self, stop_playback_at_time: i64, time_scale: i64) {
        unsafe {
            self.output.as_mut().StopScheduledPlayback(
                stop_playback_at_time,
                std::ptr::null_mut(),
                time_scale,
            )
        };
    }

    pub fn set_scheduled_frame_completion_callback(&mut self) {
        let mut rust_callback = crate::bridge::RustOutputCallback {};
        unsafe {
            let output_callback = decklink_ffi::new_output_callback(
                &mut rust_callback as *mut crate::bridge::RustOutputCallback,
            );
            self.output.as_mut().SetScheduledFrameCompletionCallback(
                output_callback as *mut decklink_ffi::IDeckLinkVideoOutputCallback,
            );
        }
    }
}

impl Drop for DecklinkOutput<'_> {
    fn drop(&mut self) {
        self.output.as_mut().Release();
    }
}

pub struct DecklinkVideoFrame {
    frame: *mut decklink_ffi::IDeckLinkMutableVideoFrame,
}

impl DecklinkVideoFrame {
    pub fn fill_blue(&self) {
        unsafe { decklink_ffi::FillBlue(self.frame) };
    }
}

impl Drop for DecklinkVideoFrame {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.frame as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkInput<'a> {
    input: Pin<&'a mut decklink_ffi::IDeckLinkInput>,
}

impl DecklinkInput<'_> {
    pub fn enable_video_input(
        &mut self,
        display_mode: BMDDisplayMode,
        pixel_format: BMDPixelFormat,
        output_flags: u32,
    ) {
        self.input
            .as_mut()
            .EnableVideoInput(display_mode.repr, pixel_format.repr, output_flags);
    }

    pub fn start_streams(&mut self) {
        self.input.as_mut().StartStreams();
    }

    pub fn stop_streams(&mut self) {
        self.input.as_mut().StopStreams();
    }

    pub fn set_callback(&mut self) {
        let mut rust_callback = crate::bridge::RustInputCallback {};
        let input_callback = unsafe {
            decklink_ffi::new_input_callback(
                &mut rust_callback as *mut crate::bridge::RustInputCallback,
            )
        };
        unsafe {
            self.input
                .as_mut()
                .SetCallback(input_callback as *mut decklink_ffi::IDeckLinkInputCallback)
        };
    }
}

impl Drop for DecklinkInput<'_> {
    fn drop(&mut self) {
        self.input.as_mut().Release();
    }
}

pub type BMDPixelFormat = decklink_ffi::_BMDPixelFormat;
pub type BMDDisplayMode = decklink_ffi::_BMDDisplayMode;
