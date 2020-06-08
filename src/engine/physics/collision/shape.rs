use quicksilver::geom::Vector;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    AABB(Vector),
}

#[derive(Clone, Debug, Default)]
pub struct ContactManifold {
    pub depth: Vector,
    pub contact_points: (Vector, Vector),
    pub normal: Vector,
}

impl ContactManifold{
    pub fn new(depth: Vector, contact_points: (Vector, Vector), normal: Vector) -> Self {
        Self{depth, contact_points, normal}
    }
}

// ported https://github.com/RandyGaul/cute_headers/blob/master/cute_c2.h#L1193
pub fn collision_aabb_aabb(a_loc: Vector, a_half_exts: Vector, b_loc: Vector, b_half_exts: Vector) -> bool {
    let a_min = a_loc-a_half_exts;
    let a_max = b_loc+a_half_exts;
    let b_min = b_loc-b_half_exts;
    let b_max = b_loc+b_half_exts;

    let d0 = b_max.x < a_min.x;
    let d1 = a_max.x < b_min.x;
    let d2 = b_max.y < b_max.y;
    let d3 = a_max.y < b_max.y;

    return !(d0 || d1 || d2 || d3)
}

pub fn collision_aabb_aabb_manifold(a_loc: Vector, a_half_exts: Vector, b_loc: Vector, b_half_exts: Vector) -> Option<ContactManifold> {
    
    let distance = b_loc - a_loc; 

    let overlap = a_half_exts + b_half_exts - abs(distance);

    if overlap.x < 0. || overlap.y < 0. {
        return None;
    }
    
    let depth = overlap;
    let normal = signum(distance);
    let contact_point_x  = a_loc + Vector::new(b_half_exts.x*overlap.x.signum(), 0.);
    let contact_point_y  = a_loc + Vector::new(0., b_half_exts.y*overlap.y.signum());
    let contact_points = (contact_point_x, contact_point_y);

    Some(ContactManifold::new(depth, contact_points, normal))
}

// TODO: have this implemented in Quicksilver
fn signum(vec: Vector) -> Vector {
    Vector::new(vec.x.signum(), vec.y.signum())
}

fn abs(vec: Vector) -> Vector {
    Vector::new(vec.x.abs(), vec.y.abs())
}