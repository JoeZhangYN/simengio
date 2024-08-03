use std::time::Duration;
use enigo::{
    Direction::{Press,Release},
    Enigo, Settings,
    //{Axis::Horizontal, Axis::Vertical},
    //{Coordinate::Abs, Coordinate::Rel},
    Key, Keyboard,
};
fn main() {
    let mut settings: Settings = Settings::default();
    settings.release_keys_when_dropped = false;
    let mut enigo = Enigo::new(&settings).unwrap();
    std::thread::sleep(Duration::from_millis(3000));
    enigo.key(Key::D, Press).unwrap();
    std::thread::sleep(Duration::from_millis(3000));
    enigo.key(Key::D, Release).unwrap();
}
