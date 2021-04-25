
use crate::City;

pub struct Map {
    pub cities: Vec<City>,
    pub pheromones: Vec<Vec<usize>>,
}

impl Map {
    pub fn new(cities: &[City]) -> Map {

        let mut pheromones = Vec::new();

        for _ in 0..cities.len() {
            let mut a = Vec::new();
            for _ in 0..cities.len() { a.push(0usize) }
            pheromones.push(a);
        }

        Map { 
            cities: cities.to_vec(),
            pheromones: pheromones,
        }
    }

    pub fn best_path(&self, from: usize) -> Vec<usize> {
        let mut already_visited = Vec::new();
        let mut path = Vec::new();

        already_visited.push(from);
        path.push(from);

        while already_visited.len() < self.cities.len() {
            let (mut best, mut index) = (self.pheromones[from][from], from);

            for i in 0..self.pheromones[from].len() {
                if self.pheromones[from][i] > best && !already_visited.contains(&i) {
                    best = self.pheromones[from][i];
                    index = i;
                }
            }

            path.push(index);
            already_visited.push(index);
        }

        path
    }
}