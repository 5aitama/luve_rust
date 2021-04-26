//! My crate opengl_rs is awesome
extern crate gl;
extern crate glfw;

use cgmath::Vector2;
use luve_rust::{ant::Ant, city::City, map::Map};

pub fn main() {
    
    // Create new map with cities...
    let mut map = Map::new(&[
        City::new(Vector2::new(0.0, 0.0)),
        City::new(Vector2::new(0.0, 1.0)),
        City::new(Vector2::new(1.0, 1.0)),
        City::new(Vector2::new(2.0, 0.0)),
    ]);

    // Generate ants (one per city)
    let mut ants = Vec::new();

    for i in 0..map.cities.len() {
        ants.push(Ant::new(i));
    }

    // Spawn 100 x 4 ants to explore the map
    for _ in 0..100 {
        for i in 0..ants.len() {
            ants[i].explore_map(&mut map)
        }
    }

    // Retrieve the best path from the first city (at index 0)
    let best_path = map.best_path(3);

    // Print the result !!
    print!("Path : ");
    for i in 0..best_path.len() {
        print!(
            "city {}{}",
            best_path[i],
            if i < best_path.len() - 1 {
                " to "
            } else {
                "\n"
            }
        )
    }
}
