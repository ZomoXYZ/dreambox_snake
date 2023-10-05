use std::f32::consts::PI;

use dbsdk_rs::{math::Vector3, vdp};

use crate::rng::Rng;

#[derive(Clone, Copy)]
pub struct StateFloaty {
    pub tick: u32,
    pub ticks: Vector3,
    pub tick_offset: Vector3,
    pub magnitude: Vector3
}

impl StateFloaty {
    pub fn new(ticks: Vector3, tick_offset: Vector3, magnitude: Vector3) -> StateFloaty {
        StateFloaty {
            tick: 0,
            ticks,
            tick_offset,
            magnitude
        }
    }

    fn translation(&self) -> Vector3 {
        let mut t = Vector3::zero();
        let tick = self.tick as f32;

        t.x = (((tick + self.tick_offset.x) % self.ticks.x) / self.ticks.x * 2.0 * PI).sin() * self.magnitude.x / 2.0;
        t.y = (((tick + self.tick_offset.y) % self.ticks.y) / self.ticks.y * 2.0 * PI).sin() * self.magnitude.y / 2.0;
        t.z = (((tick + self.tick_offset.z) % self.ticks.z) / self.ticks.z * 2.0 * PI).sin() * self.magnitude.z / 2.0;
        
        t
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn float(&self, verts: Vec<vdp::Vertex>, size: f32) -> Vec<vdp::Vertex> {
        let t = self.translation();
        let mut new_verts = Vec::with_capacity(verts.len());

        for vert in verts {
            let mut new_vert = vert;
            new_vert.position.x += t.x * size;
            new_vert.position.y += t.y * size;
            new_vert.position.z += t.z * size;
            new_verts.push(new_vert);
        }

        new_verts
    }
}




#[derive(Clone, Copy)]
pub struct FloatyCamera {
    pub ticks: [u32; 6],
    pub tick_offset: [u32; 6],
    pub magnitude: [f32; 6]
}

pub struct FloatyCameraOffsets {
    pub translation: Vector3,
    pub rotation: Vector3
}

impl FloatyCamera {
    pub fn new(rng: &mut Rng, magnitude_translation: Vector3, magnitude_rotation: Vector3, ticks_min: u32, ticks_max: u32, tick_offset_min: u32, tick_offset_max: u32) -> FloatyCamera {
        FloatyCamera {
            ticks: FloatyCamera::ticks(rng, ticks_min, ticks_max),
            tick_offset: FloatyCamera::tick_offset(rng, tick_offset_min, tick_offset_max),
            magnitude: [
                magnitude_translation.x,
                magnitude_translation.y,
                magnitude_translation.z,
                magnitude_rotation.x,
                magnitude_rotation.y,
                magnitude_rotation.z
            ]
        }
    }

    fn ticks(rng: &mut Rng, min: u32, max: u32) -> [u32; 6] {
        [
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max)
        ]
    }

    fn tick_offset(rng: &mut Rng, min: u32, max: u32) -> [u32; 6] {
        [
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max),
            min + rng.random_single_u32(max)
        ]
    }

    pub fn offsets(&self, tick: f32) -> FloatyCameraOffsets {
        let mut t = Vector3::zero();
        let mut r = Vector3::zero();

        t.x = tick_progress_sin(tick, self.tick_offset[0], self.ticks[0]) * self.magnitude[0] / 2.0;
        t.y = tick_progress_sin(tick, self.tick_offset[1], self.ticks[1]) * self.magnitude[1] / 2.0;
        t.z = tick_progress_sin(tick, self.tick_offset[2], self.ticks[2]) * self.magnitude[2] / 2.0;

        r.x = tick_progress_sin(tick, self.tick_offset[3], self.ticks[3]) * self.magnitude[3] / 2.0;
        r.y = tick_progress_sin(tick, self.tick_offset[4], self.ticks[4]) * self.magnitude[4] / 2.0;
        r.z = tick_progress_sin(tick, self.tick_offset[5], self.ticks[5]) * self.magnitude[5] / 2.0;
        
        FloatyCameraOffsets {
            translation: t,
            rotation: r
        }
    }
}

fn tick_progress(tick: f32, offset: f32, ticks: f32) -> f32 {
    ((tick + offset) % ticks) as f32 / ticks as f32
}

fn tick_progress_sin(tick: f32, offset: u32, ticks: u32) -> f32 {
    (tick_progress(tick, offset as f32, ticks as f32) * 2.0 * PI).sin()
}
