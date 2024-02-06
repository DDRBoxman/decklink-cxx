use core_foundation::string::CFStringRef;
use cxx::UniquePtr;
use std::pin::Pin;
use std::ptr::null;
use std::{pin, ptr::null_mut};

use crate::decklink_ffi::{IDeckLink, IDeckLinkIterator};

#[cxx::bridge]
mod decklink_ffi {

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

        //unsafe fn GetModelName(self: Pin<&mut IDeckLink>, modelName: *const *const c_char) -> i32;
    }
}

fn main() {
    let api_info = decklink_ffi::CreateDeckLinkAPIInformationInstance();
    println!("{:?}", api_info);
    unsafe {
        let pin: Pin<&mut decklink_ffi::IDeckLinkAPIInformation> =
            Pin::new_unchecked(api_info.as_mut().unwrap());
        let mut val: i64 = 0;

        pin.GetInt(0x76657273, &mut val as *mut i64);
        println!("{}", val);
    }

    let iterator: *mut IDeckLinkIterator = decklink_ffi::CreateDeckLinkIteratorInstance();

    println!("{:?}", iterator);

    unsafe {
        let pin: Pin<&mut IDeckLinkIterator> = Pin::new_unchecked(iterator.as_mut().unwrap());

        let mut device: *mut IDeckLink = std::ptr::null_mut();
        let device_ptr: *mut *mut IDeckLink = &mut device;

        pin.Next(device_ptr);

        let pin: Pin<&mut IDeckLink> = Pin::new_unchecked(device.as_mut().unwrap());

        /* pin.GetModelName();

        result = deckLink->GetModelName(&deviceNameString);
        if (result == S_OK)
        {
            std::string deviceName = DlToStdString(deviceNameString);
            printf("=============== %s ===============\n\n", deviceName.c_str());
            DeleteString(deviceNameString);
        }*/
    }
}
