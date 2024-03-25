use decklink_cxx;

fn main() {
    let mut api_info = decklink_cxx::DecklinkAPIInformation::new();

    println!("{}", api_info.get_version());
}
