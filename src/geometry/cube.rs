use dbsdk_rs::{math::{Vector4, Vector3}, vdp};
use util::{min, max};

use crate::util::{vec3, vec4};

use super::weight::scale;

pub struct Cube {
    pub from: Vector3,
    pub to: Vector3,
    pub scale: Vector3,
    pub color: Vector4,
    pub weight: Vector3
}

impl Cube {
    pub fn new(from: Vector3, to: Vector3, scale: Vector3, color: Vector4, weight: Vector3) -> Cube {
        Cube {
            from: vec3(
                min(from.x, to.x),
                min(from.y, to.y),
                min(from.z, to.z)
            ),
            to: vec3(
                max(from.x, to.x),
                max(from.y, to.y),
                max(from.z, to.z)
            ),
            scale,
            color,
            weight
        }
    }

    pub fn get_raw_corners(&self) -> [Vector3; 2] {
        let mut s = vec![self.from, self.to];
        scale(&mut s, self.scale, self.weight);
        [
            s[0],
            s[1]
        ]
    }
    
    pub fn get_corners(&self) -> [Vector4; 8] {
        let s = self.get_raw_corners();

        [
            // z-
            vec4(s[0].x, s[0].y, s[0].z, 1.0),
            vec4(s[1].x, s[0].y, s[0].z, 1.0),
            vec4(s[1].x, s[1].y, s[0].z, 1.0),
            vec4(s[0].x, s[1].y, s[0].z, 1.0),

            // z+
            vec4(s[0].x, s[0].y, s[1].z, 1.0),
            vec4(s[1].x, s[0].y, s[1].z, 1.0),
            vec4(s[1].x, s[1].y, s[1].z, 1.0),
            vec4(s[0].x, s[1].y, s[1].z, 1.0),
        ]
    }

    // should be a trait
    pub fn tris(&self) -> Vec<vdp::Vertex> {
        let corners = self.get_corners();
        let mut tris = Vec::new();

        // z-
        tris.append(&mut square_tris([corners[0], corners[1], corners[2], corners[3]], self.color));

        // z+
        tris.append(&mut square_tris([corners[4], corners[5], corners[6], corners[7]], self.color));

        // x-
        tris.append(&mut square_tris([corners[0], corners[3], corners[7], corners[4]], self.color));

        // x+
        tris.append(&mut square_tris([corners[1], corners[2], corners[6], corners[5]], self.color));

        // y-
        tris.append(&mut square_tris([corners[0], corners[1], corners[5], corners[4]], self.color));

        // y+
        tris.append(&mut square_tris([corners[2], corners[3], corners[7], corners[6]], self.color));

        tris
    }
}

pub fn square_tris(corners: [Vector4; 4], color: Vector4) -> Vec<vdp::Vertex> {
    vec![
        vdp::Vertex::new(
            corners[0],
            color,
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            corners[1],
            color,
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            corners[2],
            color,
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            corners[0],
            color,
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            corners[2],
            color,
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            corners[3],
            color,
            Vector4::zero(),
            Vector4::zero()),
    ]
}
