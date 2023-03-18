pub struct Triangle {
    pub vertices: [[f64; 2]; 3],
}

pub struct Polygon {
    pub vertices: Vec<[f64; 2]>,
}

pub type Board = Vec<bool>;
pub type Shape = (usize, usize);

fn vecsub(lhs: [f64; 2], rhs: [f64; 2]) -> [f64; 2] {
    [lhs[0] - rhs[0], lhs[1] - rhs[1]]
}

fn length(v: [f64; 2]) -> f64 {
    (v[0].powf(2.) + v[1].powf(2.)).sqrt()
}

fn normalize(v: [f64; 2]) -> [f64; 2] {
    let len = length(v);
    [v[0] / len, v[1] / len]
}

fn usize(f: f64) -> usize {
    f.max(0.) as usize
}
fn get_t(y: f64, v: [f64; 2], d: [f64; 2]) -> f64 {
    (y - v[1]) / d[1]
}

fn get_s(y: f64, v: [f64; 2], d: [f64; 2]) -> f64 {
    (y - v[1]) * d[0] / d[1] + v[0]
}

pub trait PolygonInterface {
    fn vertices(&self) -> &[[f64; 2]];
    fn vertices_mut(&mut self) -> &mut [[f64; 2]];
}

impl PolygonInterface for Triangle {
    fn vertices(&self) -> &[[f64; 2]] {
        &self.vertices
    }
    fn vertices_mut(&mut self) -> &mut [[f64; 2]] {
        &mut self.vertices
    }
}

impl PolygonInterface for Polygon {
    fn vertices(&self) -> &[[f64; 2]] {
        &self.vertices
    }
    fn vertices_mut(&mut self) -> &mut [[f64; 2]] {
        &mut self.vertices
    }
}

pub fn scale(poly: &mut impl PolygonInterface, scale: f64) {
    for v in poly.vertices_mut() {
        v[0] *= scale;
        v[1] *= scale;
    }
}

/// Get the maximum of an iterator of floating point values, because we can't use `max` method
/// since floats don't implement `Eq`. It panics when the input iterator is empty.
fn max_f(it: impl Iterator<Item = f64>) -> f64 {
    it.reduce(|acc, cur| acc.max(cur)).unwrap()
}

/// See [`self::max_f`]
fn min_f(it: impl Iterator<Item = f64>) -> f64 {
    it.reduce(|acc, cur| acc.min(cur)).unwrap()
}

pub fn fill_polygon(board: &mut Board, shape: Shape, poly: &impl PolygonInterface, outline: bool) {
    let vertices = poly.vertices();
    let bbox = [
        min_f(vertices.iter().map(|pos| pos[0])),
        min_f(vertices.iter().map(|pos| pos[1])),
        max_f(vertices.iter().map(|pos| pos[0])),
        max_f(vertices.iter().map(|pos| pos[1])),
    ];

    // println!("bbox: {bbox:?}");

    for y in usize(bbox[1].max(0.))..=usize(bbox[3]).min(shape.1 - 1) {
        let mut intersects = vec![];
        for i in 0..vertices.len() {
            let i1 = (i + 1) % vertices.len();
            let d = vecsub(vertices[i1], vertices[i]);
            if d[1] == 0. {
                continue;
            }
            let t = get_t(y as f64, vertices[i], normalize(d));
            // println!("y: {y} t: {t} / {d}", d = length(d));
            if t < 0. || length(d) < t {
                continue;
            }
            let x = get_s(y as f64, vertices[i], normalize(d)).round();
            // println!("    x: {x}");
            if outline {
                let x = x as usize;
                board[x + y * shape.0] = true;
            } else if intersects.last() != Some(&x) {
                intersects.push(x);
            }
        }
        // println!("intersects: {intersects:?}");
        if !intersects.is_empty() {
            intersects.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for xs in intersects.chunks_exact(2) {
                for x in usize(xs[0].max(0.))..=usize(xs[1]).min(shape.0 - 1) {
                    board[x + y * shape.0] = true;
                }
            }
        }
    }
}

pub fn measure_time<T>(f: impl FnOnce() -> T) -> (T, f64) {
    let start = std::time::Instant::now();
    let ret = f();
    (ret, start.elapsed().as_secs_f64())
}

/// Fill the polygon with naive [point in polygon](https://en.wikipedia.org/wiki/Point_in_polygon) algorithm.
/// It is much slower than [`self::fill_polygon`], but provided as a reference.
pub fn fill_naive(board: &mut Board, shape: Shape, poly: &impl PolygonInterface) {
    let vertices = poly.vertices();
    let bbox = [
        min_f(vertices.iter().map(|pos| pos[0])),
        min_f(vertices.iter().map(|pos| pos[1])),
        max_f(vertices.iter().map(|pos| pos[0])),
        max_f(vertices.iter().map(|pos| pos[1])),
    ];

    // println!("bbox: {bbox:?}");

    for y in usize(bbox[1].max(0.))..=usize(bbox[3]).min(shape.1 - 1) {
        for x in usize(bbox[0].max(0.))..=usize(bbox[2]).min(shape.0 - 1) {
            let mut intersects = 0;
            for i in 0..vertices.len() {
                let i1 = (i + 1) % vertices.len();
                let d = vecsub(vertices[i1], vertices[i]);
                if d[1] == 0. {
                    continue;
                }
                let t = get_t(y as f64, vertices[i], normalize(d));
                // println!("y: {y} t: {t} / {d}", d = length(d));
                if t < 0. || length(d) < t {
                    continue;
                }
                let s = get_s(y as f64, vertices[i], normalize(d)).round();
                // println!("    x: {x}");
                if s < x as f64 {
                    continue;
                }
                intersects += 1;
            }
            // println!("intersects: {intersects:?}");
            if intersects % 2 == 1 {
                board[x + y * shape.0] = true;
            }
        }
    }
}
