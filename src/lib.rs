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

    pub fn next(&mut self) -> Option<DecklinkDevice>{
        let mut device: *mut decklink_ffi::IDeckLink = std::ptr::null_mut();
        let device_ptr: *mut *mut decklink_ffi::IDeckLink = &mut device;

        unsafe { self.iterator.as_mut().Next(device_ptr); }

        if device.is_null() {
            None
        } else {
            Some(DecklinkDevice{
                device: unsafe { Pin::new_unchecked(device.as_mut().unwrap()) },
                device_raw: device
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
    device_raw: *mut decklink_ffi::IDeckLink
}

impl DecklinkDevice<'_> {
    pub fn get_name(&self) -> String{
        unsafe { decklink_ffi::GetDisplayName(self.device_raw) }
    }
}

impl Drop for DecklinkDevice<'_> {
    fn drop(&mut self) {
        self.device.as_mut().Release();
    }
}

type BMDPixelFormat = decklink_ffi::_BMDPixelFormat;
