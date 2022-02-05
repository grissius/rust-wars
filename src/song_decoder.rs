use regex::Regex;
use lazy_static::lazy_static;

fn song_decoder(song: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(WUB)+").unwrap();
    }
    RE.replace_all(&song, " ")
        .as_ref()
        .to_string()
        .trim()
        .to_string()
}

#[test]
fn returns_expected() {
    assert_eq!(song_decoder("WUBAWUBWUBC"), "A C");
    assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C");
    assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
    assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
}