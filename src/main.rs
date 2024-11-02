use bevy::{ecs::query, prelude::*};

#[derive(Component)]
struct Crab {
    speed: f32,
    legs: u8,
}

#[derive(Component)]
struct CrabName(String);

#[derive(Component)]
struct CrabColor(String);

#[derive(Component)]
struct CrabSize(f32);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, crab_init);
    app.add_systems(Update, (change_all_crab_names, gimme_crab, gimme_named_crabs, gimme_nameless_crab, gimme_special_crabs).chain());
    app.run();
}

fn crab_init(mut commands: Commands) {
    commands.spawn((
        Crab {
            speed: 10.0,
            legs: 8,
        },
        CrabName("Max".to_string()),
    ));

    commands.spawn((
        Crab {
            speed: 150.0,
            legs: 7,
        },
        CrabName("Josh".to_string()),
    ));

    commands.spawn(
        Crab {
            speed: 100.0,
            legs: 6,
        }
    );

    commands.spawn((
        Crab {
            speed: 200.0,
            legs: 5,
        },
        CrabName("Bob".to_string()),
        CrabSize(10.0),
        CrabColor("Red".to_string()),
    ));
}


//single component query
fn gimme_crab(query: Query<&Crab>) {
    println!("Crabs:");
    for crab in query.iter() {
        println!("Crab speed: {}", crab.speed);
        println!("Crab legs: {}", crab.legs);
        println!("");
    }
}

//multiple component query
fn gimme_named_crabs(query: Query<(&Crab, &CrabName)>) {
    println!("Crabs with names:");
    for (crab, name) in query.iter() {
        println!("Crab name: {}", name.0);
        println!("Crab speed: {}", crab.speed);
        println!("Crab legs: {}", crab.legs);
        println!("");
    }
}

//without component query
fn gimme_nameless_crab(query: Query<&Crab, Without<CrabName>>) {
    println!("Nameless crabs:");
    for crab in query.iter() {
        println!("Crab speed: {}", crab.speed);
        println!("Crab legs: {}", crab.legs);
        println!("");
    }
}

//multiple component query with multiple component filters
fn gimme_special_crabs(query: Query<(&Crab, &CrabName), (With<CrabSize>, With<CrabColor>)>) {
    println!("Special crabs:");
    for (crab, name) in query.iter() {
        println!("Crab name: {}", name.0);
        println!("Crab speed: {}", crab.speed);
        println!("Crab legs: {}", crab.legs);
        println!("");
    }
}

//mutable query
fn change_all_crab_names(mut query: Query<&mut CrabName>) {
    for mut name in query.iter_mut() {
        name.0 = "Michael".to_string();
    }
}