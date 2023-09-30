use dbsdk_rs::math::Vector3;
use util::{min, max};

pub struct Container {
    pub from: Vector3,
    pub to: Vector3,
    pub diff: Vector3,
}

impl Container {
    pub fn new(from: Vector3, to: Vector3, diff: Vector3) -> Container {
        Container {
            from,
            to,
            diff,
        }
    }
    pub fn zero() -> Container {
        Container {
            from: Vector3::zero(),
            to: Vector3::zero(),
            diff: Vector3::zero(),
        }
    }
}

pub fn size(vertices: Vec<Vector3>) -> Container {
    if vertices.len() == 0 {
        return Container::zero();
    }

    let mut from = Vector3::from(vertices[0]);
    let mut to = Vector3::from(vertices[0]);

    for i in 1..vertices.len() {
        from.x = min(from.x, vertices[i].x);
        from.y = min(from.y, vertices[i].y);
        from.z = min(from.z, vertices[i].z);
        to.x = max(to.x, vertices[i].x);
        to.y = max(to.y, vertices[i].y);
        to.z = max(to.z, vertices[i].z);
    }

    Container::new(from, to, to - from)
}

pub static X1: Vector3 = Vector3::new(0.0, 0.5, 0.5);
pub static X2: Vector3 = Vector3::new(1.0, 0.5, 0.5);
pub static Y1: Vector3 = Vector3::new(0.5, 0.0, 0.5);
pub static Y2: Vector3 = Vector3::new(0.5, 1.0, 0.5);
pub static Z1: Vector3 = Vector3::new(0.5, 0.5, 0.0);
pub static Z2: Vector3 = Vector3::new(0.5, 0.5, 1.0);
pub static CENTER: Vector3 = Vector3::new(0.5, 0.5, 0.5);

pub fn scale(vertices: &mut Vec<Vector3>, scale: Vector3, weight: Vector3) {
    let s = size(vertices.clone());

    for i in 0..vertices.len() {
        let mut norm = (vertices[i] - s.from) / s.diff;
        norm = weight - (weight - norm) * scale;
        if s.diff.x == 0.0 {
            norm.x = 0.0;
        }
        if s.diff.y == 0.0 {
            norm.y = 0.0;
        }
        if s.diff.z == 0.0 {
            norm.z = 0.0;
        }
        vertices[i] = s.from + norm * s.diff;
    }
}
