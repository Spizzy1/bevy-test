use bevy::prelude::*;



fn add_people(mut commands: Commands) {
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, add_people)
        .run();
}
