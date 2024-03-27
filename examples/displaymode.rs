fn main() {
    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator
        .next()
        .expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());

    let mut output = device.get_output();
    let mut iterator = output.get_display_mode_iterator();

    loop {
        let res = iterator.next();

        match res {
            Some(mut display_mode) => {
                println!(
                    "{} {} {}x{}",
                    display_mode.name(),
                    display_mode.display_mode_id(),
                    display_mode.width(),
                    display_mode.height(),
                );
            }
            None => break,
        }
    }
}
