use rand::prelude::*;
use wasm_bindgen::prelude::*;
use razorbill::sim::Simulation as CoreSimulation;
use razorbill::sim::{World as CoreWorld, Animal as CoreAnimal, Food as CoreFood};

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: CoreSimulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = CoreSimulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
}

// ------------------------------------------------
// World
// ------------------------------------------------

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,

    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

impl From<&CoreWorld> for World {
    fn from(world: &CoreWorld) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self { animals, foods }
    }
}

// ------------------------------------------------
// Animal
// ------------------------------------------------

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&CoreAnimal> for Animal {
    fn from(animal: &CoreAnimal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

// Food

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&CoreFood> for Food {
    fn from(food: &CoreFood) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
