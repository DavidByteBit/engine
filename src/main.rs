mod UI;
mod shapes;

const h: f64 = 256.0;
const w: f64 = 240.0;
const fontw: usize = 4;
const fonth: usize = 4;

pub fn OnUserCreate() -> bool{
    return true
}

pub fn OnUserUpdate(fElapsedTime: f64) -> bool {

    let cube = shapes::Mesh::new_cube();
    let projection = shapes::DepthProj::proj(h/w);

    for tri in cube.0 {
        let triProjected0 = shapes::DepthProj::mat_mul(&tri.0[0], &projection);
        let triProjected1 = shapes::DepthProj::mat_mul(&tri.0[1], &projection);
        let triProjected2 = shapes::DepthProj::mat_mul(&tri.0[2], &projection);

        // TODO: draw tringle...
    }

    return true;
}

pub fn main() {
    let cube = shapes::Mesh::new_cube();

    UI::run(h, w, fontw, fonth);
}