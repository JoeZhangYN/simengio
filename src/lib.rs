use enigo::*;

#[no_mangle]
pub extern "C" fn KeyDown(s: u16) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Raw(s));
}

#[no_mangle]
pub extern "C" fn KeyUp(s: u16) {
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Raw(s));
}

#[no_mangle]
pub extern "C" fn KeyPress(s: u16) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Raw(s));
    enigo.key_up(Key::Raw(s));
}

#[no_mangle]
pub extern "C" fn KeyPressWhile(s: u16, d: u16) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Raw(d));
    enigo.key_down(Key::Raw(s));
    enigo.key_up(Key::Raw(s));
    enigo.key_up(Key::Raw(d));
}

#[no_mangle]
pub extern "C" fn KeyPressAlt(s: u16) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Alt);
    enigo.key_down(Key::Raw(s));
    enigo.key_up(Key::Raw(s));
    enigo.key_up(Key::Alt);
}

#[no_mangle]
pub extern "C" fn MouseMove(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
}

#[no_mangle]
pub extern "C" fn MouseMoveRelative(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(x, y);
}

#[no_mangle]
pub extern "C" fn LeftClick() {
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Left);
}

#[no_mangle]
pub extern "C" fn Rightlick() {
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Right);
}