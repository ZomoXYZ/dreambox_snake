use dbsdk_rs::{math::{Vector4, Vector3}, vdp};

fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

#[derive(PartialEq)]
pub enum CubeWeight {
    X1,
    X2,
    Y1,
    Y2,
    Z1,
    Z2,
    Center
}

pub struct Cube {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    z1: f32,
    z2: f32,
    scale: Vector3,
    color: Vector4,
    weight: CubeWeight
}

impl Cube {
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32, scale: Vector3, color: Vector4, weight: CubeWeight) -> Cube {
        Cube {
            x1: min(x1, x2),
            x2: max(x1, x2),
            y1: min(y1, y2),
            y2: max(y1, y2),
            z1: min(z1, z2),
            z2: max(z1, z2),
            scale,
            color,
            weight
        }
    }

    fn get_corners(&self) -> [Vector4; 8] {
        let pad_x = (1.0 - self.scale.x) * (self.x2 - self.x1) / 2.0;
        let pad_y = (1.0 - self.scale.y) * (self.y2 - self.y1) / 2.0;
        let pad_z = (1.0 - self.scale.z) * (self.z2 - self.z1) / 2.0;

        let nx1 = if self.weight == CubeWeight::X1 { self.x1 } else if self.weight == CubeWeight::X2 { self.x1 + pad_x * 2.0 } else { self.x1 + pad_x };
        let nx2 = if self.weight == CubeWeight::X2 { self.x2 } else if self.weight == CubeWeight::X1 { self.x2 - pad_x * 2.0 } else { self.x2 - pad_x };
        let ny1 = if self.weight == CubeWeight::Y1 { self.y1 } else if self.weight == CubeWeight::Y2 { self.y1 + pad_y * 2.0 } else { self.y1 + pad_y };
        let ny2 = if self.weight == CubeWeight::Y2 { self.y2 } else if self.weight == CubeWeight::Y1 { self.y2 - pad_y * 2.0 } else { self.y2 - pad_y };
        let nz1 = if self.weight == CubeWeight::Z1 { self.z1 } else if self.weight == CubeWeight::Z2 { self.z1 + pad_z * 2.0 } else { self.z1 + pad_z };
        let nz2 = if self.weight == CubeWeight::Z2 { self.z2 } else if self.weight == CubeWeight::Z1 { self.z2 - pad_z * 2.0 } else { self.z2 - pad_z };

        [
            Vector4::new(nx1, ny1, nz1, 1.0),
            Vector4::new(nx2, ny1, nz1, 1.0),
            Vector4::new(nx2, ny2, nz1, 1.0),
            Vector4::new(nx1, ny2, nz1, 1.0),
            Vector4::new(nx1, ny1, nz2, 1.0),
            Vector4::new(nx2, ny1, nz2, 1.0),
            Vector4::new(nx2, ny2, nz2, 1.0),
            Vector4::new(nx1, ny2, nz2, 1.0),
        ]
    }

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
