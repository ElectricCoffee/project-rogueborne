use tcod::console::*;

pub struct RootConsole {
    pub root: Root,
}

impl Default for RootConsole {
    fn default() -> Self {
        RootConsole { root: Root::initializer().init() }
    }
}