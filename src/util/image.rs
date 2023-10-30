use crate::*;

use rand::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ImageGen;
impl ImageGen {
    pub fn test_terrain_from_height_data_f32(height_data: &[f32], dim: u32, stride: u32, max_height: f32) -> Vec<u8>{
        let mut data = vec![];
        
        let mut y = 0;
        while y < dim {
            let mut x = 0;
            while x < dim {
                let height = height_data[(y * dim + x) as usize];
                let value = Math::inverse_lerp(0.0, max_height, height);

                let mut rng = thread_rng();
                let color_mod: [i16; 3] = [rng.gen_range(-5..=5); 3];

                let mut color: [i16; 3] = [245; 3];
                if value < 0.85 { color = [235, 235, 245]; } // snow
                if value < 0.70 { color = [139, 155, 180]; } // mountain
                if value < 0.60 { color = [90 , 105, 136]; } // dark mountain
                if value < 0.50 { color = [58 , 68 , 102]; } // darker mountain
                if value < 0.45 { color = [62 , 137, 72 ]; } // grass
                if value < 0.40 { color = [99 , 199, 77 ]; } // dark grass
                if value < 0.30 { color = [116, 62 , 57 ]; } // dirt
                if value < 0.25 { color = [62 , 39 , 49 ]; } // dark dirt

                let final_color: [u8; 3] = [
                    (color[0] + color_mod[0]) as u8,
                    (color[1] + color_mod[1]) as u8,
                    (color[2] + color_mod[2]) as u8,
                ];

                data.extend(final_color);
                data.push(255);

                x += stride;
            }

            y += stride;
        }

        data
    }
}