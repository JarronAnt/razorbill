pub use crate::eye::Eye;
pub use crate::network::Network;
pub use crate::network::LayerTopology;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::TAU;

// ------------------------------------------------
// World
// ------------------------------------------------

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

// ------------------------------------------------
// Animal
// ------------------------------------------------

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Network,
    pub(crate) satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = Network::random(
            rng,
            &[
                LayerTopology { neurons: eye.cells() },
                LayerTopology { neurons: 2 * eye.cells() },
                LayerTopology { neurons: 2 },
            ],
        );

        Self {
            position: na::Point2::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            rotation: na::Rotation2::new(rng.gen_range(0.0..TAU)),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

// ------------------------------------------------
// Food
// ------------------------------------------------

#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

// ------------------------------------------------
// Simulation
// ------------------------------------------------

pub struct Simulation {
    pub(crate) world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = na::Point2::new(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    );
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.propagate(vision);

            let speed = response[0].clamp(-0.001, 0.001);
            let rotation = response[1].clamp(-0.001, 0.001);

            animal.speed = (animal.speed + speed).clamp(0.001, 0.005);
            animal.rotation = na::Rotation2::new(
                animal.rotation.angle() + rotation,
            );
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position +=
                animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}
