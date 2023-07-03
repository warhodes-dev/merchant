use std::fmt::Write;
use anyhow::Result;
use crossterm::style::{
    Stylize,
    Color,
};
use worldgen::{
    noise::perlin::PerlinNoise, 
    world::{Size, World, Tile, tile::{Constraint, ConstraintType}},
    noisemap::{NoiseMap, NoiseMapGenerator, NoiseMapGeneratorBase, Step, Seed}, constraint,
};

pub struct WorldApp {
    seed: u32,
    text: String,
}

impl WorldApp {
    pub fn new() -> Self {
        WorldApp {
            seed: 0,
            text: "Hello World".to_string()
        }
    }

    pub fn generate(&mut self) -> Result<()> {
        let noise = PerlinNoise::new();

        let noisemap = Box::new(
            NoiseMap::new(noise)
                .set(Seed::of(self.seed))
                .set(Step::of(0.05, 0.05))
        );

        let world = World::new()
            .set(Size::of(80, 50))
            .add(Tile::new('~').when(constraint!(noisemap.clone(), < 0.0)))
            .add(Tile::new(',').when(constraint!(noisemap.clone(), < 0.45)))
            .add(Tile::new('^').when(constraint!(noisemap.clone(), > 0.8)))
            .add(Tile::new('n'));

        let mut buf = String::new();

        for rows in world.generate(0, 0).iter() {
            for cols in rows.iter() {
                for char in cols.iter() {
                    write!(buf, "{char}")?;
                }
                writeln!(buf)?;
            }
        }

        self.text = buf;
        self.seed += 1;

        Ok(())
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}