mod bridge;

use bridge::{decklink_ffi, decklink_type_wrappers::c_BMDDeckLinkAPIInformationID};

use std::pin::Pin;

pub struct DecklinkAPIInformation {
    api_info: *mut decklink_ffi::IDeckLinkAPIInformation,
}

impl DecklinkAPIInformation {
    pub fn new() -> Self {
        let api_info = decklink_ffi::CreateDeckLinkAPIInformationInstance();
        return DecklinkAPIInformation { api_info };
    }

    pub fn get_version(&mut self) -> i64 {
        let mut val = crate::bridge::decklink_type_wrappers::c_longlong(0);
        unsafe {
            let pin: Pin<&mut decklink_ffi::IDeckLinkAPIInformation> =
                Pin::new_unchecked(self.api_info.as_mut().unwrap());
            pin.GetInt(
                c_BMDDeckLinkAPIInformationID(
                    decklink_ffi::_BMDDeckLinkAPIInformationID::BMDDeckLinkAPIVersion.repr,
                ),
                &mut val as *mut crate::bridge::decklink_type_wrappers::c_longlong,
            );
        }

        return val.0;
    }
}

impl Drop for DecklinkAPIInformation {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.api_info as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkIterator {
    iterator: *mut decklink_ffi::IDeckLinkIterator,
}

impl DecklinkIterator {
    pub fn new() -> Self {
        let iterator = decklink_ffi::CreateDeckLinkIteratorInstance();
        return DecklinkIterator { iterator };
    }

    pub fn next(&mut self) -> Option<DecklinkDevice> {
        let mut device: *mut decklink_ffi::IDeckLink = std::ptr::null_mut();
        let device_ptr: *mut *mut decklink_ffi::IDeckLink = &mut device;

        unsafe {
            let pin: Pin<&mut decklink_ffi::IDeckLinkIterator> =
                Pin::new_unchecked(self.iterator.as_mut().unwrap());
            pin.Next(device_ptr);
        }

        if device.is_null() {
            None
        } else {
            Some(DecklinkDevice { device })
        }
    }
}

impl Drop for DecklinkIterator {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.iterator as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkDevice {
    device: *mut decklink_ffi::IDeckLink,
}

impl DecklinkDevice {
    pub fn get_name(&self) -> String {
        unsafe { decklink_ffi::GetDisplayName(self.device) }
    }

    pub fn get_output(&self) -> DecklinkOutput {
        let mut output: *mut decklink_ffi::IDeckLinkOutput = std::ptr::null_mut();
        let output_ptr: *mut *mut decklink_ffi::IDeckLinkOutput = &mut output;
        let result = unsafe { decklink_ffi::GetOutput(self.device, output_ptr) };

        return DecklinkOutput { output };
    }

    pub fn get_input(&self) -> DecklinkInput {
        let mut input: *mut decklink_ffi::IDeckLinkInput = std::ptr::null_mut();
        let input_ptr: *mut *mut decklink_ffi::IDeckLinkInput = &mut input;
        let result = unsafe { decklink_ffi::GetInput(self.device, input_ptr) };

        return DecklinkInput { input };
    }
}

impl Drop for DecklinkDevice {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.device as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkOutput {
    output: *mut decklink_ffi::IDeckLinkOutput,
}

impl DecklinkOutput {
    pub fn get_display_mode_iterator(&mut self) -> DeckLinkDisplayModeIterator {
        let mut display_mode_iterator: *mut decklink_ffi::IDeckLinkDisplayModeIterator =
            std::ptr::null_mut();
        let display_mode_iterator_ptr: *mut *mut decklink_ffi::IDeckLinkDisplayModeIterator =
            &mut display_mode_iterator;
        unsafe {
            let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
                Pin::new_unchecked(self.output.as_mut().unwrap());
            pin.GetDisplayModeIterator(display_mode_iterator_ptr)
        };

        return DeckLinkDisplayModeIterator {
            iterator: display_mode_iterator,
        };
    }

    pub fn enable_video_output(&mut self, display_mode: BMDDisplayMode, output_flags: u32) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
            unsafe { Pin::new_unchecked(self.output.as_mut().unwrap()) };
        pin.EnableVideoOutput(
            bridge::decklink_type_wrappers::c_BMDDisplayMode(display_mode.repr),
            bridge::decklink_type_wrappers::c_BMDVideoOutputFlags(output_flags),
        );
    }

    pub fn create_video_frame(
        &mut self,
        width: i32,
        height: i32,
        pixel_format: BMDPixelFormat,
    ) -> Result<DecklinkVideoFrame, ()> {
        let mut frame: *mut decklink_ffi::IDeckLinkMutableVideoFrame = std::ptr::null_mut();
        let frame_ptr: *mut *mut decklink_ffi::IDeckLinkMutableVideoFrame = &mut frame;

        let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
            unsafe { Pin::new_unchecked(self.output.as_mut().unwrap()) };
        let result = unsafe {
            pin.CreateVideoFrame(
                width,
                height,
                ((width + 47) / 48) * 128, // todo: it's silly decklink passes this can we do it ourself?
                bridge::decklink_type_wrappers::c_BMDPixelFormat(pixel_format.repr),
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
        let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
            unsafe { Pin::new_unchecked(self.output.as_mut().unwrap()) };
        let result = unsafe {
            pin.ScheduleVideoFrame(
                frame.frame as *mut decklink_ffi::IDeckLinkVideoFrame,
                crate::bridge::decklink_type_wrappers::c_longlong(display_time),
                crate::bridge::decklink_type_wrappers::c_longlong(display_duration),
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
        let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
            unsafe { Pin::new_unchecked(self.output.as_mut().unwrap()) };
        pin.StartScheduledPlayback(
            crate::bridge::decklink_type_wrappers::c_longlong(playback_start_time),
            crate::bridge::decklink_type_wrappers::c_longlong(time_scale),
            playback_speed,
        );
    }

    pub fn stop_scheduled_playback(&mut self, stop_playback_at_time: i64, time_scale: i64) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
            unsafe { Pin::new_unchecked(self.output.as_mut().unwrap()) };
        unsafe {
            pin.StopScheduledPlayback(
                crate::bridge::decklink_type_wrappers::c_longlong(stop_playback_at_time),
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
            let pin: Pin<&mut decklink_ffi::IDeckLinkOutput> =
                unsafe { Pin::new_unchecked(self.output.as_mut().unwrap()) };
            pin.SetScheduledFrameCompletionCallback(
                output_callback as *mut decklink_ffi::IDeckLinkVideoOutputCallback,
            );
        }
    }
}

impl Drop for DecklinkOutput {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.output as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkVideoFrame {
    frame: *mut decklink_ffi::IDeckLinkMutableVideoFrame,
}

impl DecklinkVideoFrame {
    pub fn fill_blue(&self) {
        unsafe { decklink_ffi::FillBlue(self.frame) };
    }

    pub fn get_ancillary_packets(&self) {
        let mut video_frame_ancillary_packets: *mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets =
        std::ptr::null_mut();
        let video_frame_ancillary_packets_ptr: *mut *mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets =
        &mut video_frame_ancillary_packets;

        unsafe {
            decklink_ffi::GetAncillaryPackets(
                self.frame as *mut decklink_ffi::IDeckLinkVideoFrame,
                video_frame_ancillary_packets_ptr,
            )
        };
    }
}

impl Drop for DecklinkVideoFrame {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.frame as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DeckLinkVideoFrameAncillaryPackets {
    packets: *mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets,
}

impl DeckLinkVideoFrameAncillaryPackets {
    pub fn get_packet_iterator(&self) {
        let mut ancillary_packet_iterator: *mut decklink_ffi::IDeckLinkAncillaryPacketIterator =
            std::ptr::null_mut();
        let ancillary_packet_iterator_ptr: *mut *mut decklink_ffi::IDeckLinkAncillaryPacketIterator =
        &mut ancillary_packet_iterator;

        let pin: Pin<&mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets> =
            unsafe { Pin::new_unchecked(self.packets.as_mut().unwrap()) };

        let result = unsafe { pin.GetPacketIterator(ancillary_packet_iterator_ptr) };
    }

    pub fn attach_packet(&self, packet: DeckLinkAncillaryPacket) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets> =
            unsafe { Pin::new_unchecked(self.packets.as_mut().unwrap()) };

        let result = unsafe { pin.AttachPacket(packet.packet) };
    }

    pub fn detach_packet(&self, packet: DeckLinkAncillaryPacket) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets> =
            unsafe { Pin::new_unchecked(self.packets.as_mut().unwrap()) };

        let result = unsafe { pin.DetachPacket(packet.packet) };
    }

    pub fn detach_all_packets(&self) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkVideoFrameAncillaryPackets> =
            unsafe { Pin::new_unchecked(self.packets.as_mut().unwrap()) };

        let result = pin.DetachAllPackets();
    }
}

impl Drop for DeckLinkVideoFrameAncillaryPackets {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.packets as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DeckLinkAncillaryPacket {
    packet: *mut decklink_ffi::IDeckLinkAncillaryPacket,
}

impl DeckLinkAncillaryPacket {
    pub fn get_did(&self) -> u8 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkAncillaryPacket> =
            unsafe { Pin::new_unchecked(self.packet.as_mut().unwrap()) };
        return pin.GetDID();
    }

    pub fn get_sdid(&self) -> u8 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkAncillaryPacket> =
            unsafe { Pin::new_unchecked(self.packet.as_mut().unwrap()) };
        return pin.GetSDID();
    }

    pub fn get_line_number(&self) -> u32 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkAncillaryPacket> =
            unsafe { Pin::new_unchecked(self.packet.as_mut().unwrap()) };
        return pin.GetLineNumber();
    }

    pub fn get_data_stream_index(&self) -> u8 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkAncillaryPacket> =
            unsafe { Pin::new_unchecked(self.packet.as_mut().unwrap()) };
        return pin.GetDataStreamIndex();
    }

    pub fn get_bytes(&self, format: BMDAncillaryPacketFormat) {
        let mut data: *const u8 = std::ptr::null_mut();
        let data_ptr: *mut *const u8 = &mut data;

        let mut size: *mut u32 = std::ptr::null_mut();

        unsafe {
            decklink_ffi::GetAncillaryPacketBytes(
                self.packet,
                bridge::decklink_type_wrappers::c_BMDAncillaryPacketFormat(format.repr),
                data_ptr,
                size,
            )
        };
    }
}

impl Drop for DeckLinkAncillaryPacket {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.packet as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkInput {
    input: *mut decklink_ffi::IDeckLinkInput,
}

impl DecklinkInput {
    pub fn get_display_mode_iterator(&mut self) -> DeckLinkDisplayModeIterator {
        let mut display_mode_iterator: *mut decklink_ffi::IDeckLinkDisplayModeIterator =
            std::ptr::null_mut();
        let display_mode_iterator_ptr: *mut *mut decklink_ffi::IDeckLinkDisplayModeIterator =
            &mut display_mode_iterator;

        unsafe {
            let pin: Pin<&mut decklink_ffi::IDeckLinkInput> =
                Pin::new_unchecked(self.input.as_mut().unwrap());
            pin.GetDisplayModeIterator(display_mode_iterator_ptr)
        };

        return DeckLinkDisplayModeIterator {
            iterator: display_mode_iterator,
        };
    }

    pub fn enable_video_input(
        &mut self,
        display_mode: BMDDisplayMode,
        pixel_format: BMDPixelFormat,
        input_flags: u32,
    ) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkInput> =
            unsafe { Pin::new_unchecked(self.input.as_mut().unwrap()) };
        pin.EnableVideoInput(
            bridge::decklink_type_wrappers::c_BMDDisplayMode(display_mode.repr),
            bridge::decklink_type_wrappers::c_BMDPixelFormat(pixel_format.repr),
            bridge::decklink_type_wrappers::c_BMDVideoInputFlags(input_flags),
        );
    }

    pub fn start_streams(&mut self) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkInput> =
            unsafe { Pin::new_unchecked(self.input.as_mut().unwrap()) };
        pin.StartStreams();
    }

    pub fn stop_streams(&mut self) {
        let pin: Pin<&mut decklink_ffi::IDeckLinkInput> =
            unsafe { Pin::new_unchecked(self.input.as_mut().unwrap()) };
        pin.StopStreams();
    }

    pub fn set_callback(&mut self) {
        let mut rust_callback = crate::bridge::RustInputCallback {};
        let input_callback = unsafe {
            decklink_ffi::new_input_callback(
                &mut rust_callback as *mut crate::bridge::RustInputCallback,
            )
        };
        let pin: Pin<&mut decklink_ffi::IDeckLinkInput> =
            unsafe { Pin::new_unchecked(self.input.as_mut().unwrap()) };
        unsafe { pin.SetCallback(input_callback as *mut decklink_ffi::IDeckLinkInputCallback) };
    }
}

impl Drop for DecklinkInput {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.input as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DeckLinkDisplayModeIterator {
    iterator: *mut decklink_ffi::IDeckLinkDisplayModeIterator,
}

impl DeckLinkDisplayModeIterator {
    pub fn next(&mut self) -> Option<DecklinkDisplayMode> {
        let mut display_mode: *mut decklink_ffi::IDeckLinkDisplayMode = std::ptr::null_mut();
        let display_mode_ptr: *mut *mut decklink_ffi::IDeckLinkDisplayMode = &mut display_mode;

        let pin: Pin<&mut decklink_ffi::IDeckLinkDisplayModeIterator> =
            unsafe { Pin::new_unchecked(self.iterator.as_mut().unwrap()) };
        unsafe {
            pin.Next(display_mode_ptr);
        }

        if display_mode.is_null() {
            None
        } else {
            Some(DecklinkDisplayMode { display_mode })
        }
    }
}

impl Drop for DeckLinkDisplayModeIterator {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.iterator as *mut decklink_ffi::IUnknown) }
    }
}

pub struct DecklinkDisplayMode {
    display_mode: *mut decklink_ffi::IDeckLinkDisplayMode,
}

impl DecklinkDisplayMode {
    pub fn name(&self) -> String {
        return unsafe { decklink_ffi::GetDisplayModeName(self.display_mode) };
    }

    pub fn width(&mut self) -> i64 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkDisplayMode> =
            unsafe { Pin::new_unchecked(self.display_mode.as_mut().unwrap()) };
        return pin.GetWidth().0 as i64;
    }

    pub fn height(&mut self) -> i64 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkDisplayMode> =
            unsafe { Pin::new_unchecked(self.display_mode.as_mut().unwrap()) };
        return pin.GetHeight().0 as i64;
    }

    pub fn display_mode_id(&mut self) -> u32 {
        let pin: Pin<&mut decklink_ffi::IDeckLinkDisplayMode> =
            unsafe { Pin::new_unchecked(self.display_mode.as_mut().unwrap()) };
        return pin.GetDisplayMode().0;
    }
}

impl Drop for DecklinkDisplayMode {
    fn drop(&mut self) {
        unsafe { decklink_ffi::Release(self.display_mode as *mut decklink_ffi::IUnknown) }
    }
}

pub type BMDPixelFormat = decklink_ffi::_BMDPixelFormat;
pub type BMDDisplayMode = decklink_ffi::_BMDDisplayMode;
pub type BMDAncillaryPacketFormat = decklink_ffi::_BMDAncillaryPacketFormat;
