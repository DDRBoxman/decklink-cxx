fn main() {
    /*
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
    */
}
