use dbsdk_rs::{math::{Vector3, Vector4}, vdp};
use crate::util::{vec3_to4, min, vec3, max};

use super::weight::scale;

pub struct Square {
    from: Vector3,
    to: Vector3,
    scale: Vector3,
    color: Vector4,
    weight: Vector3
}

impl Square {
    pub fn new(from: Vector3, to: Vector3, scale: Vector3, color: Vector4, weight: Vector3) -> Square {
        Square {
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

    fn get_corners(&self) -> [Vector4; 4] {
        // let mid_z = (self.from.z + self.to.z) / 2.0;
        let mid_z = self.from.z;

        let mut s = vec![
            self.from,
            Vector3::new(self.to.x, self.from.y, mid_z),
            self.to,
            Vector3::new(self.from.x, self.to.y, mid_z),
        ];
        scale(&mut s, self.scale, self.weight);

        [
            vec3_to4(s[0]),
            vec3_to4(s[1]),
            vec3_to4(s[2]),
            vec3_to4(s[3]),
        ]
    }

    pub fn tris(&self) -> Vec<vdp::Vertex> {
        let corners = self.get_corners();

        vec![
            vdp::Vertex::new(
                corners[0],
                self.color,
                Vector4::zero(),
                Vector4::zero()),
            vdp::Vertex::new(
                corners[1],
                self.color,
                Vector4::zero(),
                Vector4::zero()),
            vdp::Vertex::new(
                corners[2],
                self.color,
                Vector4::zero(),
                Vector4::zero()),
            
            vdp::Vertex::new(
                corners[0],
                self.color,
                Vector4::zero(),
                Vector4::zero()),
            vdp::Vertex::new(
                corners[2],
                self.color,
                Vector4::zero(),
                Vector4::zero()),
            vdp::Vertex::new(
                corners[3],
                self.color,
                Vector4::zero(),
                Vector4::zero()),
        ]
    }
}
