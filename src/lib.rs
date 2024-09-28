use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
    Button, Mouse,
    {Coordinate::Abs, Coordinate::Rel},
};

#[repr(u32)]
pub enum KeysE {
    Num0 = 0x30,
    Num1 = 0x31,
    Num2 = 0x32,
    Num3 = 0x33,
    Num4 = 0x34,
    Num5 = 0x35,
    Num6 = 0x36,
    Num7 = 0x37,
    Num8 = 0x38,
    Num9 = 0x39,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,
    Add = 0x6B,
    Decimal = 0x6E,
    Divide = 0x6F,
    Multiply = 0x6A,
    Subtract = 0x6D,
    Backspace = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Enter = 0x0D,
    Shift = 0x10,
    Control = 0x11,
    Alt = 0x12,
    Pause = 0x13,
    CapsLock = 0x14,
    Escape = 0x1B,
    Space = 0x20,
    PageUp = 0x21,
    PageDown = 0x22,
    End = 0x23,
    Home = 0x24,
    LeftArrow = 0x25,
    UpArrow = 0x26,
    RightArrow = 0x27,
    DownArrow = 0x28,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    LWin = 0x5B,
    RWin = 0x5C,
    Apps = 0x5D,
    Sleep = 0x5F,
    NumLock = 0x90,
    ScrollLock = 0x91,
    BrowserBack = 0xA6,
    BrowserForward = 0xA7,
    BrowserRefresh = 0xA8,
    BrowserStop = 0xA9,
    BrowserSearch = 0xAA,
    BrowserFavorites = 0xAB,
    BrowserHome = 0xAC,
    VolumeMute = 0xAD,
    VolumeDown = 0xAE,
    VolumeUp = 0xAF,
    MediaNextTrack = 0xB0,
    MediaPrevTrack = 0xB1,
    MediaStop = 0xB2,
    MediaPlayPause = 0xB3,
    LaunchMail = 0xB4,
    LaunchMediaSelect = 0xB5,
    LaunchApp1 = 0xB6,
    LaunchApp2 = 0xB7,
    OEM1 = 0xBA,
    OEMPlus = 0xBB,
    OEMComma = 0xBC,
    OEMMinus = 0xBD,
    OEMPeriod = 0xBE,
    OEM2 = 0xBF,
    OEM3 = 0xC0,
    OEM4 = 0xDB,
    OEM5 = 0xDC,
    OEM6 = 0xDD,
    OEM7 = 0xDE,
    OEM8 = 0xDF,
    OEM102 = 0xE2,
    ProcessKey = 0xE5,
    Packet = 0xE7,
    Attention = 0xF6,
    Crsel = 0xF7,
    Exsel = 0xF8,
    ErEOF = 0xF9,
    Play = 0xFA,
    Zoom = 0xFB,
    PA1 = 0xFD,
    OEMClear = 0xFE,
}

fn map_key(key: KeysE) -> Key {
    match key {
        KeysE::Num0 => Key::Num0,
        KeysE::Num1 => Key::Num1,
        KeysE::Num2 => Key::Num2,
        KeysE::Num3 => Key::Num3,
        KeysE::Num4 => Key::Num4,
        KeysE::Num5 => Key::Num5,
        KeysE::Num6 => Key::Num6,
        KeysE::Num7 => Key::Num7,
        KeysE::Num8 => Key::Num8,
        KeysE::Num9 => Key::Num9,
        KeysE::A => Key::A,
        KeysE::B => Key::B,
        KeysE::C => Key::C,
        KeysE::D => Key::D,
        KeysE::E => Key::E,
        KeysE::F => Key::F,
        KeysE::G => Key::G,
        KeysE::H => Key::H,
        KeysE::I => Key::I,
        KeysE::J => Key::J,
        KeysE::K => Key::K,
        KeysE::L => Key::L,
        KeysE::M => Key::M,
        KeysE::N => Key::N,
        KeysE::O => Key::O,
        KeysE::P => Key::P,
        KeysE::Q => Key::Q,
        KeysE::R => Key::R,
        KeysE::S => Key::S,
        KeysE::T => Key::T,
        KeysE::U => Key::U,
        KeysE::V => Key::V,
        KeysE::W => Key::W,
        KeysE::X => Key::X,
        KeysE::Y => Key::Y,
        KeysE::Z => Key::Z,
        KeysE::F1 => Key::F1,
        KeysE::F2 => Key::F2,
        KeysE::F3 => Key::F3,
        KeysE::F4 => Key::F4,
        KeysE::F5 => Key::F5,
        KeysE::F6 => Key::F6,
        KeysE::F7 => Key::F7,
        KeysE::F8 => Key::F8,
        KeysE::F9 => Key::F9,
        KeysE::F10 => Key::F10,
        KeysE::F11 => Key::F11,
        KeysE::F12 => Key::F12,
        KeysE::Numpad0 => Key::Numpad0,
        KeysE::Numpad1 => Key::Numpad1,
        KeysE::Numpad2 => Key::Numpad2,
        KeysE::Numpad3 => Key::Numpad3,
        KeysE::Numpad4 => Key::Numpad4,
        KeysE::Numpad5 => Key::Numpad5,
        KeysE::Numpad6 => Key::Numpad6,
        KeysE::Numpad7 => Key::Numpad7,
        KeysE::Numpad8 => Key::Numpad8,
        KeysE::Numpad9 => Key::Numpad9,
        KeysE::Add => Key::Add,
        KeysE::Decimal => Key::Decimal,
        KeysE::Divide => Key::Divide,
        KeysE::Multiply => Key::Multiply,
        KeysE::Subtract => Key::Subtract,
        KeysE::Backspace => Key::Backspace,
        KeysE::Tab => Key::Tab,
        KeysE::Clear => Key::Clear,
        KeysE::Enter => Key::Return,
        KeysE::Shift => Key::Shift,
        KeysE::Control => Key::Control,
        KeysE::Alt => Key::Alt,
        KeysE::Pause => Key::Pause,
        KeysE::CapsLock => Key::CapsLock,
        KeysE::Escape => Key::Escape,
        KeysE::Space => Key::Space,
        KeysE::PageUp => Key::PageUp,
        KeysE::PageDown => Key::PageDown,
        KeysE::End => Key::End,
        KeysE::Home => Key::Home,
        KeysE::LeftArrow => Key::LeftArrow,
        KeysE::UpArrow => Key::UpArrow,
        KeysE::RightArrow => Key::RightArrow,
        KeysE::DownArrow => Key::DownArrow,
        KeysE::Insert => Key::Insert,
        KeysE::Delete => Key::Delete,
        KeysE::Help => Key::Help,
        KeysE::LWin => Key::Meta,
        KeysE::RWin => Key::Meta,
        KeysE::Apps => Key::Apps,
        KeysE::Sleep => Key::Sleep,
        KeysE::NumLock => Key::Numlock,
        KeysE::ScrollLock => Key::Scroll,
        KeysE::BrowserBack => Key::BrowserBack,
        KeysE::BrowserForward => Key::BrowserForward,
        KeysE::BrowserRefresh => Key::BrowserRefresh,
        KeysE::BrowserStop => Key::BrowserStop,
        KeysE::BrowserSearch => Key::BrowserSearch,
        KeysE::BrowserFavorites => Key::BrowserFavorites,
        KeysE::BrowserHome => Key::BrowserHome,
        KeysE::VolumeMute => Key::VolumeMute,
        KeysE::VolumeDown => Key::VolumeDown,
        KeysE::VolumeUp => Key::VolumeUp,
        KeysE::MediaNextTrack => Key::MediaNextTrack,
        KeysE::MediaPrevTrack => Key::MediaPrevTrack,
        KeysE::MediaStop => Key::MediaStop,
        KeysE::MediaPlayPause => Key::MediaPlayPause,
        KeysE::LaunchMail => Key::LaunchMail,
        KeysE::LaunchMediaSelect => Key::LaunchMediaSelect,
        KeysE::LaunchApp1 => Key::LaunchApp1,
        KeysE::LaunchApp2 => Key::LaunchApp2,
        KeysE::OEM1 => Key::OEM1,
        KeysE::OEMPlus => Key::OEMPlus,
        KeysE::OEMComma => Key::OEMComma,
        KeysE::OEMMinus => Key::OEMMinus,
        KeysE::OEMPeriod => Key::OEMPeriod,
        KeysE::OEM2 => Key::OEM2,
        KeysE::OEM3 => Key::OEM3,
        KeysE::OEM4 => Key::OEM4,
        KeysE::OEM5 => Key::OEM5,
        KeysE::OEM6 => Key::OEM6,
        KeysE::OEM7 => Key::OEM7,
        KeysE::OEM8 => Key::OEM8,
        KeysE::OEM102 => Key::OEM102,
        KeysE::ProcessKey => Key::Processkey,
        KeysE::Packet => Key::Packet,
        KeysE::Attention => Key::Attn,
        KeysE::Crsel => Key::Crsel,
        KeysE::Exsel => Key::Exsel,
        KeysE::ErEOF => Key::Ereof,
        KeysE::Play => Key::Play,
        KeysE::Zoom => Key::Zoom,
        KeysE::PA1 => Key::PA1,
        KeysE::OEMClear => Key::OEMClear,
    }
}

fn get_enigo(release_keys: bool) -> Enigo {
    let mut settings = Settings::default();
    settings.release_keys_when_dropped = release_keys;
    Enigo::new(&settings).unwrap()
}

#[no_mangle]
pub extern "C" fn KeyPress(key: KeysE) {
    let mut enigo = get_enigo(true);
    enigo.key(map_key(key), Click).unwrap();
}

#[no_mangle]
pub extern "C" fn KeyDown(key: KeysE) {
    let mut enigo = get_enigo(false);
    enigo.key(map_key(key), Press).unwrap();
}

#[no_mangle]
pub extern "C" fn KeyUp(key: KeysE) {
    let mut enigo = get_enigo(true);
    enigo.key(map_key(key), Release).unwrap();
}

#[no_mangle]
pub extern "C" fn KeyPressWhile(key1: KeysE, key2: KeysE) {
    let mut enigo = get_enigo(true);
    let ke2 = map_key(key2);
    enigo.key(ke2, Press).unwrap();
    enigo.key(map_key(key1), Click).unwrap();
    enigo.key(ke2, Release).unwrap();
}

#[no_mangle]
pub extern "C" fn KeyPressWhileTwo(key1: KeysE, key2: KeysE, key3: KeysE) {
    let mut enigo = get_enigo(true);
    let ke2 = map_key(key2);
    let ke3 = map_key(key3);
    enigo.key(ke3, Press).unwrap();
    enigo.key(ke2, Press).unwrap();
    enigo.key(map_key(key1), Click).unwrap();
    enigo.key(ke3, Release).unwrap();
    enigo.key(ke2, Release).unwrap();
}

#[no_mangle]
pub extern "C" fn KeyPressAlt(key1: KeysE) {
    let mut enigo = get_enigo(true);
    enigo.key(Key::Alt, Press).unwrap();
    enigo.key(map_key(key1), Click).unwrap();
    enigo.key(Key::Alt, Release).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseMove(x: i32, y: i32, coord_type: i32) {
    let coordinate = match coord_type {
        0 => Abs,
        1 => Rel,
        _ => panic!("Invalid coordinate type. Use 0 for Abs and 1 for Rel."),
    };
    let mut enigo = get_enigo(true);
    enigo.move_mouse(x, y, coordinate).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseLeftClick() {
    let mut enigo = get_enigo(true);
    enigo.button(Button::Left, Click).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseLeftDown() {
    let mut enigo = get_enigo(true);
    enigo.button(Button::Left, Press).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseLeftUp() {
    let mut enigo = get_enigo(true);
    enigo.button(Button::Left, Release).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseRightClick() {
    let mut enigo = get_enigo(true);
    enigo.button(Button::Right, Click).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseRightDown() {
    let mut enigo = get_enigo(true);
    enigo.button(Button::Right, Press).unwrap();
}

#[no_mangle]
pub extern "C" fn MouseRightUp() {
    let mut enigo = get_enigo(true);
    enigo.button(Button::Right, Release).unwrap();
}