use crate::*;

use noise::{Perlin, NoiseFn};
use pathfinding::num_traits::NumCast;
use rand::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Reflect)]
pub struct Perlin2dParams {
    pub scale: f32,
    pub octaves: u32,
    pub persistance: f32,
    pub lacunarity: f32,
    pub offset: Vec2,
}

impl Default for Perlin2dParams {
    fn default() -> Self {
        Self {
            scale: 200.0,
            octaves: 4,
            persistance: 0.5,
            lacunarity: 2.0,
            offset: Vec2::ZERO,
        }
    }
}

impl Validate for Perlin2dParams {
    fn validate(&mut self) {
        if self.scale <= 0.0 { self.scale = 0.0001; }
        if self.octaves < 1 { self.octaves = 1; } else if self.octaves > 8 { self.octaves = 8; }
        if self.persistance < 0.0 { self.persistance = 0.0; } else if self.persistance > 1.0 { self.persistance = 1.0; }
    }
}

struct InternalData {
    max: f32,
    data: Vec<f32>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct NoiseGen;
impl NoiseGen {
    fn perlin_2d_map_internal(
        seed: u32,
        dim: u32,
        scale: f32,
        octaves: u32,
        persistance: f32,
        lacunarity: f32,
        offset: Vec2,
    ) -> InternalData {
        let mut data = vec![];
        
        let mut rng: Pcg64 = Seeder::from(&seed).make_rng();
        let mut octave_offsets = vec![];

        let mut max = 0.0;
        let mut amplitude = 1.0;
        let mut frequency;

        for _ in 0..octaves {
            octave_offsets.push(Vec2::new(
                rng.gen_range(-100_000.0..100_000.0) + offset.x,
                rng.gen_range(-100_000.0..100_000.0) + offset.y
            ));

            max += amplitude;
            amplitude *= persistance;
        }

        let mut scale = scale;
        if scale <= 0.0 { scale = 0.0001 }

        let half_dim = dim as f32 * 0.5;
        
        let perlin = Perlin::new(seed);
        for y in 0..dim { for x in 0..dim {
            amplitude = 1.0;
            frequency = 1.0;
            let mut noise_height = 0.0;

            for i in 0..octaves {
                let sample_x = (x as f32 - half_dim + octave_offsets[i as usize].x) / scale * frequency;
                let sample_y = (y as f32 - half_dim + octave_offsets[i as usize].y) / scale * frequency;

                let value = perlin.get([sample_x as f64, sample_y as f64]) as f32;
                noise_height += value * amplitude;

                amplitude *= persistance;
                frequency *= lacunarity;
            }

            data.push(noise_height);
        }}

        InternalData { max, data }
    }

    pub fn perlin_2d_map<T: Default + NumCast + Clone + Copy + PartialEq + Sync + Send + 'static>(
        seed: u32,
        dim: u32,
        scale: f32,
        octaves: u32,
        persistance: f32,
        lacunarity: f32,
        offset: Vec2,
        multiplier: f32,
    ) -> Vec<T> {
        let internal = Self::perlin_2d_map_internal(seed, dim, scale, octaves, persistance, lacunarity, offset);

        let mut data = vec![];
        for y in 0..dim { for x in 0..dim {
            let normalized_value = Math::inverse_lerp(-internal.max, internal.max, internal.data[(y * dim + x) as usize]) * multiplier;
            data.push(pathfinding::num_traits::cast(normalized_value).unwrap());
            // print!("{} -> {normalized_value} -> {}", internal.data[(y * dim + x) as usize], pathfinding::num_traits::cast::<f32, u32>(normalized_value).unwrap());
        }}

        data
    }
    
    pub fn perlin_2d_map_from_params<T: Default + NumCast + Clone + Copy + PartialEq + Sync + Send + 'static>(
        seed: u32,
        dim: u32,
        params: Perlin2dParams,
        multiplier: f32,
    ) -> Vec<T> {
        Self::perlin_2d_map(seed, dim, params.scale, params.octaves, params.persistance, params.lacunarity, params.offset, multiplier)
    }
}