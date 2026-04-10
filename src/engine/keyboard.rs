use glium::winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

/// long ass struct that stores a field corresponding to keys
/// that this sim uses as boolean values, where if the field is true
/// that means the key is currently being held down, and false if not.
#[derive(Default, Copy, Clone)]
pub struct Keyboard {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub esc: bool,
    pub space: bool,
    pub lctrl: bool,
    prev_state: KeyState,
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key_input(&mut self, key_event: KeyEvent) {
        let pressed = key_event.state == ElementState::Pressed;
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => self.w = pressed,
            PhysicalKey::Code(KeyCode::KeyA) => self.a = pressed,
            PhysicalKey::Code(KeyCode::KeyS) => self.s = pressed,
            PhysicalKey::Code(KeyCode::KeyD) => self.d = pressed,
            PhysicalKey::Code(KeyCode::KeyQ) => self.q = pressed,
            PhysicalKey::Code(KeyCode::KeyE) => self.e = pressed,
            PhysicalKey::Code(KeyCode::ControlLeft) => self.lctrl = pressed,
            PhysicalKey::Code(KeyCode::Escape) => self.esc = pressed,
            PhysicalKey::Code(KeyCode::Space) => self.space = pressed,
            _ => (),
        }
    }

    /// stores the current state as previous state so that
    /// you can detect when keys are just released.
    pub fn end_frame(&mut self) {
        self.prev_state = KeyState {
            w: self.w,
            a: self.a,
            s: self.s,
            d: self.d,
            q: self.q,
            e: self.e,
            esc: self.esc,
            space: self.space,
            lctrl: self.lctrl,
        }
    }

    pub fn just_released(&self, key: KeyCode) -> bool {
        let (curr, prev) = match key {
            KeyCode::KeyW => (self.w, self.prev_state.w),
            KeyCode::KeyA => (self.a, self.prev_state.a),
            KeyCode::KeyS => (self.s, self.prev_state.s),
            KeyCode::KeyD => (self.d, self.prev_state.d),
            KeyCode::KeyQ => (self.q, self.prev_state.q),
            KeyCode::KeyE => (self.e, self.prev_state.e),
            KeyCode::Escape => (self.esc, self.prev_state.esc),
            KeyCode::Space => (self.space, self.prev_state.space),
            KeyCode::ControlLeft => (self.lctrl, self.prev_state.lctrl),
            _ => return false,
        };
        !curr && prev
    }
}

#[derive(Default, Copy, Clone)]
struct KeyState {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    q: bool,
    e: bool,
    esc: bool,
    space: bool,
    lctrl: bool,
}
