
use crate::map::Map;

use cgmath::prelude::*;
use rand::Rng;

/// Represent a simple Ant!
pub struct Ant {
    /// The index of the city where the current
    /// ant was.
    pub index: usize,
}

impl Ant {
    /// Create new `Ant`
    /// 
    /// # Arguments
    /// * `index` - The index of the city where the ant must be started.
    pub fn new(index: usize) -> Ant {
        Ant { index: index }
    }

    pub fn explore_map(&mut self, map: &mut Map) {
        let mut explored_city_indices = Vec::<usize>::new();

        explored_city_indices.push(self.index);
        
        while explored_city_indices.len() < map.cities.len() {
            let random_city_index = self.find_best_city_index(&explored_city_indices, &map);

            explored_city_indices.push(random_city_index);
            map.pheromones[self.index][random_city_index] += 1;
            self.index = random_city_index;
        }
    }

    /// Search a city to go based on pheromones and some probabilites.
    /// It return the city index.
    /// 
    /// # Arguments
    /// * `explored_cities_index` - An array that contains all cities index that the ant was already explored.
    /// * `map` - A map.
    fn find_best_city_index(&self, explored_cities_index: &[usize], map: &Map) -> usize {

        /* 
        This array store the index of some cities.

        The index of a city can be duplicate many time in
        this array and the reason for that is that more a 
        city index is duplicated more the index have a chance 
        to be selected.
        */
        let mut cities_index = Vec::<usize>::new();

        for i in 0..map.cities.len() {
            // Don't include the city where the current Ant are and
            // the already visited cities...
            if i == self.index || explored_cities_index.contains(&i) {
                continue
            }

            // Get the distance from the current city (where the current ant are)
            // to another city.
            let distance = (map.cities[i].position - map.cities[self.index].position).magnitude();

            // Calculate the probability that the ant is attracted to this city.
            let prob = (1.0 / distance).powf(2.3) * (map.pheromones[self.index][i] as f32).powf(1.1);

            // More prob value is higher more the current city index was duplicated.
            for _ in 0..=(prob as usize) {
                cities_index.push(i)
            }
        }

        println!("{}", cities_index.len());
        cities_index[rand::thread_rng().gen_range(0..cities_index.len())]
    }
}