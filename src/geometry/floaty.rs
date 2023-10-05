use std::f32::consts::PI;

use dbsdk_rs::{math::Vector3, vdp};

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

        t.x = (((tick + self.tick_offset.x) % self.ticks.x) / self.ticks.x * 2.0 * PI).sin() * self.magnitude.x;
        t.y = (((tick + self.tick_offset.y) % self.ticks.y) / self.ticks.y * 2.0 * PI).sin() * self.magnitude.y;
        t.z = (((tick + self.tick_offset.z) % self.ticks.z) / self.ticks.z * 2.0 * PI).sin() * self.magnitude.z;
        
        t
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }
}

pub fn floaty(verts: Vec<vdp::Vertex>, state: StateFloaty, size: f32) -> Vec<vdp::Vertex> {
    let t = state.translation();
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
