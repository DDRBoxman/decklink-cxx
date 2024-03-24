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
            self.api_info
                .as_mut()
                .GetInt(0x76657273, &mut val as *mut i64);
        }
        return val;
    }
}

impl Drop for DecklinkAPIInformation<'_> {
    fn drop(&mut self) {
        self.api_info.as_mut().Release();
    }
}
