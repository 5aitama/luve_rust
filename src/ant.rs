
use crate::map::Map as AntMap;
use crate::city::City;
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

    pub fn explore_map(&mut self, map: &mut AntMap) {
        let mut rng = rand::thread_rng();
        let mut explored_city_indices = Vec::<usize>::new();

        explored_city_indices.push(self.index);
        while explored_city_indices.len() < map.cities.len() {
            let probs = self.calculate_distances_probs(&explored_city_indices, &map.cities);
            let random_city_index = probs[rng.gen_range(0..probs.len())];

            explored_city_indices.push(random_city_index);
            map.pheromones[self.index][random_city_index] += 1;
            self.index = random_city_index;
        }
    }

    fn calculate_distances_probs(&self, explored_city_indices: &Vec::<usize>, cities: &[City]) -> Vec<usize> {

        let mut distances = Vec::<usize>::new();
    
        for i in 0..cities.len() {
            if i == self.index || explored_city_indices.contains(&i) { continue }
            let distance = (cities[i].position - cities[self.index].position).magnitude();
            let probability = ((1.0 / distance).powf(2.3) * 5.) as usize;
    
            for _ in 0..=probability { distances.push(i) }
        }
    
        distances
    }
}