#[allow(unused)]
pub mod banana; //bringing module in (banana)

use bevy::prelude::*;
use banana::BananaPlugin;
use bevy_framepace::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, BananaPlugin, FramepacePlugin));
    app.run();
}
