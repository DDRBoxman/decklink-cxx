use decklink_cxx;


fn main() {
    let mut iterator = decklink_cxx::DecklinkIterator::new();

    let device = iterator.next().expect("No device found. Please install device or drivers");

    println!("{}", device.get_name());
}