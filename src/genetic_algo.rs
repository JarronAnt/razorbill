use rand::seq::SliceRandom;
use rand::{Rng, RngCore};
use std::ops::Index;

//genes
#[derive(Clone,Debug)]
pub struct Genome {
    genes: Vec<f32>,
}

impl Genome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &f32>{
        self.genes.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Genome {
    type Output = f32;

    fn index(&self, index:usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl std::iter::FromIterator<f32> for Genome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Genome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

// individual bird
pub trait Individual {
    fn create(genome: Genome) -> Self;
    fn genome(&self) -> &Genome;
    fn fitness(&self) -> f32;
}

// selection
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteSelection;

impl SelectionMethod for RouletteSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where 
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty Pop!")
    }
}

//recombination
pub trait RecomboMethod {
    fn recombo(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Genome,
        parent_b: &Genome,
    ) -> Genome;
}

#[derive(Clone, Debug)]
pub struct UniformRecombo;

impl RecomboMethod for UniformRecombo {
    fn recombo(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Genome,
        parent_b: &Genome,
    )->Genome {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) {a} else {b})
            .collect()
    }
}

//Mutation
pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Genome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >=0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Genome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as f64) {
            *gene += sign * self.coeff * rng.r#gen::<f32>();            
            }
        }
    }
}

// genetic algo
pub struct GeneticAlgo<S> {
    selection_method: S,
    recombo_method: Box<dyn RecomboMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgo<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        recombo_method: impl RecomboMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            recombo_method: Box::new(recombo_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).genome();
                let parent_b = self.selection_method.select(rng, population).genome();
                let mut child = self.recombo_method.recombo(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
        .collect()

    } 
}

