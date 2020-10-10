//              _          _  __ _
//        _ __ (_)_  _____| |/ _(_)_ __ ___
//       | '_ \| \ \/ / _ \ | |_| | '__/ _ \
//       | |_) | |>  <  __/ |  _| | | |  __/
//       | .__/|_/_/\_\___|_|_| |_|_|  \___|
//       |_|
//
// pixelfire - A pixel fire using Rust and winit + pixels
// Copyright (C) 2020, Mario Kilies <mario@kilies.de>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use rand::{prelude::ThreadRng, thread_rng, Rng};

const FIRE_WIDTH: usize = 80;
const FIRE_HEIGHT: usize = 80;
const REDUCTION: u8 = 6;

pub struct Fire {
    pixels: [[[u8; 4]; FIRE_WIDTH]; FIRE_HEIGHT],
    rng: ThreadRng,
    enabled: bool,
}

impl Default for Fire {
    fn default() -> Self {
        let mut f = Fire {
            pixels: [[[0; 4]; FIRE_WIDTH]; FIRE_HEIGHT],
            rng: thread_rng(),
            enabled: true,
        };
        f.pixels[FIRE_HEIGHT - 1] = [[0xFF, 0x00, 0x00, 0xFF]; FIRE_WIDTH];
        f
    }
}

impl Fire {
    pub fn as_slice(&self) -> &[u8] {
        let pixels_ptr = self.pixels.as_ptr() as *const u8;

        unsafe { std::slice::from_raw_parts(pixels_ptr, 4 * FIRE_WIDTH * FIRE_HEIGHT) }
    }

    pub fn width(&self) -> u32 {
        FIRE_WIDTH as u32
    }

    pub fn height(&self) -> u32 {
        FIRE_HEIGHT as u32
    }

    pub fn update(&mut self) {
        let p = &mut self.pixels;
        for h in 0..FIRE_HEIGHT - 1 {
            for w in 0..FIRE_WIDTH {
                p[h][w][0] = p[h + 1][w][0];
                p[h][w][1] = 0x00;
                p[h][w][2] = 0x00;
                let random_number = self.rng.gen_range(0, 3);
                let subtrahend = random_number * REDUCTION;
                p[h][w][0] = if p[h][w][0] >= subtrahend {
                    p[h][w][0] - subtrahend
                } else {
                    0x00
                };
            }
        }
    }

    pub fn toggle(&mut self) {
        if self.enabled {
            self.pixels[FIRE_HEIGHT - 1] = [[0x00, 0x00, 0x00, 0xFF]; FIRE_WIDTH];
            self.enabled = false;
        } else {
            self.pixels[FIRE_HEIGHT - 1] = [[0xFF, 0x00, 0x00, 0xFF]; FIRE_WIDTH];
            self.enabled = true;
        }
    }
}
