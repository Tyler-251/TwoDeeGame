#[allow(unused)]
pub mod banana; //bringing module in (banana)

use bevy::prelude::*;
use banana::BananaPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, BananaPlugin));
    app.run();
}
