use std::fmt::Write;
use anyhow::Result;
use cursive::theme::Color;
use worldgen::{
    noise::perlin::PerlinNoise, 
    world::{Size, World, Tile, tile::{Constraint, ConstraintType}},
    noisemap::{NoiseMap, NoiseMapGenerator, NoiseMapGeneratorBase, Step, Seed}, constraint,
};

#[derive(Copy, Clone)]
pub struct TileStyle {
    glyph: char,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
}

impl TileStyle {
    pub fn fg_rgb(&self) -> Color {
        Color::Rgb(self.fg.0, self.fg.1, self.fg.2)
    }
    pub fn bg_rgb(&self) -> Color {
        Color::Rgb(self.bg.0, self.bg.1, self.bg.2)
    }
    pub fn glyph(&self) -> char { self.glyph }
}

#[derive(Clone)]
pub struct WorldApp {
    seed: u32,
    tiles: Vec<TileStyle>,
    pub width: usize,
    pub height: usize,
}

impl WorldApp {
    pub fn new(size: (usize, usize), seed: u32) -> Self {
        let mut world = WorldApp {
            seed,
            tiles: Vec::new(),
            width: size.0,
            height: size.1,
        };
        world.generate();
        world
    }

    pub fn generate(&mut self) -> Result<()> {
        let noise = PerlinNoise::new();

        let noisemap = Box::new(
            NoiseMap::new(noise)
                .set(Seed::of(self.seed))
                .set(Step::of(0.05, 0.05))
        );

        let world = World::new()
            .set(Size::of(self.width as i64, self.height as i64))
            .add(Tile::new('~').when(constraint!(noisemap.clone(), < 0.0)))
            .add(Tile::new(',').when(constraint!(noisemap.clone(), < 0.45)))
            .add(Tile::new('^').when(constraint!(noisemap.clone(), > 0.8)))
            .add(Tile::new('n'));

        let mut tiles = Vec::new();

        for rows in world.generate(0, 0).iter() {
            for cols in rows.iter() {
                for char in cols.iter() {
                    let tile = style_map(*char);
                    tiles.push(tile);
                }
            }
        }

        self.tiles = tiles;
        self.seed += 1;

        Ok(())
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileStyle {
        *self.tiles.get(y * self.width + x).expect(&format!("X:{x}, Y:{y}"))
    }
}

fn style_map(c: char) -> TileStyle {
    match c {
        '~' => TileStyle{glyph: '~', fg: (1, 1, 1), bg: (0, 0, 255)},
        ',' => TileStyle{glyph: ',', fg: (0, 0, 0), bg: (0, 255, 0)},
        '^' => TileStyle{glyph: '^', fg: (0, 0, 0), bg: (100, 100, 100)},
        'n' => TileStyle{glyph: 'n', fg: (0, 0, 0), bg: (99, 66, 33)},
        _ => panic!("Unsupported tile character")
    }
}