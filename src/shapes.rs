pub struct Vec3d{x: f64, y: f64, z: f64}

pub struct Triangle(pub [Vec3d; 3]);

pub struct Mesh(pub Vec<Triangle>);

impl Mesh {
    pub fn new_cube() -> Mesh {
        Mesh(vec![
            // SOUTH
            Triangle ([Vec3d{x: 0.0, y: 0.0, z: 0.0},    Vec3d{x: 0.0, y: 1.0, z: 0.0},    Vec3d{x: 1.0, y: 1.0, z: 0.0}]),
            Triangle ([Vec3d{x: 0.0, y: 0.0, z: 0.0},    Vec3d{x: 1.0, y: 1.0, z: 0.0},    Vec3d{x: 1.0, y: 0.0, z: 0.0}]),
        
            // EAST
            Triangle ([Vec3d{x: 1.0, y: 0.0, z: 0.0},    Vec3d{x: 1.0, y: 1.0, z: 0.0},    Vec3d{x: 1.0, y: 1.0, z: 1.0}]),
            Triangle ([Vec3d{x: 1.0, y: 0.0, z: 0.0},    Vec3d{x: 1.0, y: 1.0, z: 1.0},    Vec3d{x: 1.0, y: 0.0, z: 1.0}]),
        
            // NORTH
            Triangle ([Vec3d{x: 1.0, y: 0.0, z: 1.0},    Vec3d{x: 1.0, y: 1.0, z: 1.0},    Vec3d{x: 0.0, y: 1.0, z: 1.0}]),
            Triangle ([Vec3d{x: 1.0, y: 0.0, z: 1.0},    Vec3d{x: 0.0, y: 1.0, z: 1.0},    Vec3d{x: 0.0, y: 0.0, z: 1.0}]),
        
            // WEST
            Triangle ([Vec3d{x: 0.0, y: 0.0, z: 1.0},    Vec3d{x: 0.0, y: 1.0, z: 1.0},    Vec3d{x: 0.0, y: 1.0, z: 0.0}]),
            Triangle ([Vec3d{x: 0.0, y: 0.0, z: 1.0},    Vec3d{x: 0.0, y: 1.0, z: 0.0},    Vec3d{x: 0.0, y: 0.0, z: 0.0}]),
        
            // TOP
            Triangle ([Vec3d{x: 0.0, y: 1.0, z: 0.0},    Vec3d{x: 0.0, y: 1.0, z: 1.0},    Vec3d{x: 1.0, y: 1.0, z: 1.0}]),
            Triangle ([Vec3d{x: 0.0, y: 1.0, z: 0.0},    Vec3d{x: 1.0, y: 1.0, z: 1.0},    Vec3d{x: 1.0, y: 1.0, z: 0.0}]),
        
            // BOTTOM
            Triangle ([Vec3d{x: 1.0, y: 0.0, z: 1.0},    Vec3d{x: 0.0, y: 0.0, z: 1.0},    Vec3d{x: 0.0, y: 0.0, z: 0.0}]),
            Triangle ([Vec3d{x: 1.0, y: 0.0, z: 1.0},    Vec3d{x: 0.0, y: 0.0, z: 0.0},    Vec3d{x: 1.0, y: 0.0, z: 0.0}]),
        ])}
}


// Projection Matrix constants 
const NEAR: f64 = 0.1;
const FAR: f64 = 1000.0;
const FOV: f64 = 90.0;
const FOVRAD_CONST: f64 = 1.0 / (FOV * 0.5 / 180.0 * 3.14159);

/**
 * Can be logically thought of as a 4x4 matrix, but most values are 0. As such,
 * we define a single 1d vector to represent the structure, and explicitly define
 * how the matrix multiplicaiton would transpire if it were in correct matrix form
 */
pub struct DepthProj(Vec<f64>);

impl DepthProj {
    pub fn proj(AspectRatio: f64) -> DepthProj {
        let fov_rad = FOVRAD_CONST.tanh();
        DepthProj(vec![
            AspectRatio * fov_rad,
            fov_rad,
            FAR / (FAR - NEAR),
            (-FAR * NEAR) / (FAR - NEAR),
            1.0
        ])}

    pub fn mat_mul(i: &Vec3d, m: &DepthProj) -> Vec3d {

        let mut o = Vec3d{x: 0.0, y: 0.0, z: 0.0};

        // Matrix multiplication (relavent values only)
        o.x = i.x * m.0[0];
		o.y = i.y * m.0[1];
		o.z = i.z * m.0[2] + m.0[3];
		let w =  i.z * m.0[3];

		if w != 0.0 {o.x /= w; o.y /= w; o.z /= w} 
        
        o
    }
}