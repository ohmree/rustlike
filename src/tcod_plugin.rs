use bevy::prelude::*;
use tcod::{colors, console::*};

pub struct Tcod {
    pub root: Root,
    pub screen_width: i32,
    pub screen_height: i32,
    pub limit_fps: i32,
}

impl Tcod {
    pub fn new(name: &str, (screen_width, screen_height): (i32, i32)) -> Self {
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(screen_width, screen_height)
            .title(name)
            .init();
        Self {
            root,
            screen_width,
            screen_height,
            limit_fps: 20,
        }
    }
}

fn setup_tcod(mut tcod: ResMut<Tcod>, commands: &Commands) {
    tcod.root.set_default_foreground(colors::WHITE);
    tcod.root.clear();
}

#[derive(Default)]
pub struct TcodPlugin;
impl Plugin for TcodPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Tcod::new("Rustlike", (80, 50)))
            .add_startup_system(setup_tcod.system());
    }
}
