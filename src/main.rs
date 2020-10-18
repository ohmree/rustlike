mod tcod_plugin;

use crate::tcod_plugin::{Tcod, TcodPlugin};
use bevy::app::AppExit;
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_core::CorePlugin;
use bevy_diagnostic::DiagnosticsPlugin;
use bevy_type_registry::TypeRegistryPlugin;
use tcod::{
    colors,
    console::*,
    input::{self, Key, KeyCode},
};

struct Controllable;

struct Position {
    x: i32,
    y: i32,
}

fn setup(mut commands: Commands, tcod: Res<Tcod>) {
    commands.spawn((
        '@',
        Controllable,
        Position {
            x: tcod.screen_width / 2,
            y: tcod.screen_height / 2,
        },
    ));
}

fn render(mut tcod: ResMut<Tcod>, mut query: Query<(&char, &Position)>) {
    for (symbol, position) in &mut query.iter() {
        tcod.root.set_default_foreground(colors::WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(position.x, position.y, *symbol, BackgroundFlag::None);
        tcod.root.flush();
    }
}

fn update_controllables(
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut tcod: ResMut<Tcod>,
    mut query: Query<(&mut Position, &Controllable)>,
) {
    if tcod.root.window_closed() {
        app_exit_events.send(AppExit);
    } else {
        let key = tcod.root.wait_for_keypress(true);
        let (x, y) = match key {
            Key {
                code: KeyCode::Char,
                printable: 'h',
                ..
            } => (-1, 0),
            Key {
                code: KeyCode::Char,
                printable: 'k',
                ..
            } => (0, -1),
            Key {
                code: KeyCode::Char,
                printable: 'j',
                ..
            } => (0, 1),
            Key {
                code: KeyCode::Char,
                printable: 'l',
                ..
            } => (1, 0),
            Key {
                code: KeyCode::Char,
                printable: 'y',
                ..
            } => (-1, -1),
            Key {
                code: KeyCode::Char,
                printable: 'b',
                ..
            } => (-1, 1),
            Key {
                code: KeyCode::Char,
                printable: 'u',
                ..
            } => (1, -1),
            Key {
                code: KeyCode::Char,
                printable: 'n',
                ..
            } => (1, 1),
            // Key {
            //     code: KeyCode::Escape,
            //     ..
            // }
            _ => {
                app_exit_events.send(AppExit);
                (0, 0)
            }
        };
        for (mut position, _) in &mut query.iter() {
            position.x += x;
            position.y += y;
        }
    }
}

fn main() {
    App::build()
        .add_plugin(TypeRegistryPlugin::default())
        .add_plugin(CorePlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(TcodPlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(setup.system())
        .add_system(render.system())
        .add_system(update_controllables.system())
        .run();
}
