use smithay::input::keyboard::Keysym;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ModifiersState {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub caps_lock: bool,
    pub win_key: bool,
    pub num_lock: bool,
}

// TOOD: add setter functions
impl ModifiersState {}

#[derive(Debug)]
enum KeyAction {
    Run(String),
    None,
}

fn process_keyboard_shortcut(modifiers: ModifiersState, keysym: Keysym) -> Option<KeyAction> {
    None
}
