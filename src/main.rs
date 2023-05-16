use bevy::prelude::*;
mod chunking;


fn main() {
    let app = &mut App::new();
    let app = chunking::add_chunking(app);
    app.run();
}