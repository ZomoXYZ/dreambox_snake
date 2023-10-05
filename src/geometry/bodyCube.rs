use dbsdk_rs::{math::{Vector3, Vector4}, vdp};

use crate::util::{vec4, vec3};

use super::cube::{Cube, square_tris};

pub struct BodyCube {
    pub cube: Cube,
    pub cut_height: f32, // [0,1], from z- to z+
    pub cut_spread: f32, // [0,1], from center to edge along z
}

impl BodyCube {
    pub fn new(from: Vector3, to: Vector3, scale: Vector3, color: Vector4, weight: Vector3, cut_height: f32, cut_spread: f32) -> BodyCube {
        BodyCube {
            cube: Cube::new(from, to, scale, color, weight),
            cut_height,
            cut_spread,
        }
    }

    pub fn get_corners(&self) -> [Vector4; 12] {
        let s = self.cube.get_raw_corners();
        let inner = vec3(
            (s[1].x - s[0].x) / 2.0 * (1.0 - self.cut_spread),
            (s[1].y - s[0].y) / 2.0 * (1.0 - self.cut_spread),
            (s[1].z - s[0].z) * self.cut_height
        );

        [
            // z-
            vec4(s[0].x, s[0].y, s[0].z, 1.0),
            vec4(s[1].x, s[0].y, s[0].z, 1.0),
            vec4(s[1].x, s[1].y, s[0].z, 1.0),
            vec4(s[0].x, s[1].y, s[0].z, 1.0),

            // z+
            vec4(s[0].x+inner.x, s[0].y+inner.y, s[1].z, 1.0),
            vec4(s[1].x-inner.x, s[0].y+inner.y, s[1].z, 1.0),
            vec4(s[1].x-inner.x, s[1].y-inner.y, s[1].z, 1.0),
            vec4(s[0].x+inner.x, s[1].y-inner.y, s[1].z, 1.0),

            // z mid
            vec4(s[0].x, s[0].y, inner.z, 1.0),
            vec4(s[1].x, s[0].y, inner.z, 1.0),
            vec4(s[1].x, s[1].y, inner.z, 1.0),
            vec4(s[0].x, s[1].y, inner.z, 1.0),
        ]
    }

    pub fn tris(&self) -> Vec<vdp::Vertex> {
        let corners = self.get_corners();
        let mut tris = Vec::new();

        // z-
        tris.append(&mut square_tris([corners[0], corners[1], corners[2], corners[3]], self.cube.color));

        // z+
        tris.append(&mut square_tris([corners[4], corners[5], corners[6], corners[7]], self.cube.color));

        // z+ edges
        tris.append(&mut square_tris([corners[8], corners[9], corners[5], corners[4]], self.cube.color));
        tris.append(&mut square_tris([corners[9], corners[10], corners[6], corners[5]], self.cube.color));
        tris.append(&mut square_tris([corners[10], corners[11], corners[7], corners[6]], self.cube.color));
        tris.append(&mut square_tris([corners[11], corners[8], corners[4], corners[7]], self.cube.color));

        // x-
        tris.append(&mut square_tris([corners[0], corners[3], corners[11], corners[8]], self.cube.color));

        // x+
        tris.append(&mut square_tris([corners[1], corners[2], corners[10], corners[9]], self.cube.color));

        // y-
        tris.append(&mut square_tris([corners[0], corners[1], corners[9], corners[8]], self.cube.color));

        // y+
        tris.append(&mut square_tris([corners[2], corners[3], corners[11], corners[10]], self.cube.color));

        tris
    }
}
