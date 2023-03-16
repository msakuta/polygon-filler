struct Triangle {
    vertices: [[f64; 2]; 3],
}

struct Polygon {
    vertices: Vec<[f64; 2]>,
}

type Board = Vec<bool>;
type Shape = (usize, usize);

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

trait PolygonInterface {
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

fn scale(poly: &mut impl PolygonInterface, scale: f64) {
    for v in poly.vertices_mut() {
        v[0] *= scale;
        v[1] *= scale;
    }
}

fn fill_polygon(board: &mut Board, shape: Shape, poly: &impl PolygonInterface, outline: bool) {
    let vertices = poly.vertices();
    let bbox = [
        vertices
            .iter()
            .map(|pos| pos[0])
            .reduce(|acc, cur| acc.min(cur))
            .unwrap(),
        vertices
            .iter()
            .map(|pos| pos[1])
            .reduce(|acc, cur| acc.min(cur))
            .unwrap(),
        vertices
            .iter()
            .map(|pos| pos[0])
            .reduce(|acc, cur| acc.max(cur))
            .unwrap(),
        vertices
            .iter()
            .map(|pos| pos[1])
            .reduce(|acc, cur| acc.max(cur))
            .unwrap(),
    ];

    // println!("bbox: {bbox:?}");

    for y in usize(bbox[1])..=usize(bbox[3]) {
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
            if x < 0. || shape.0 as f64 <= x {
                continue;
            }
            let x = x as usize;
            if outline {
                board[x + y * shape.0] = true;
            } else if intersects.last() != Some(&x) {
                intersects.push(x);
            }
        }
        // println!("intersects: {intersects:?}");
        if !intersects.is_empty() {
            intersects.sort();
            for xs in intersects.chunks_exact(2) {
                for x in xs[0]..xs[1] {
                    board[x + y * shape.0] = true;
                }
            }
        }
    }
}
fn print_board(board: &Board, shape: Shape) {
    for y in 0..shape.1 {
        for x in 0..shape.0 {
            print!("{}", if board[x + y * shape.0] { '*' } else { '-' });
        }
        println!("");
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();

    let mut shape = (64, 35);
    let mut poly = false;
    let mut outline = false;
    let mut noprint = false;
    while let Some(arg) = args.next() {
        match &arg as &str {
            "help" => {
                eprintln!(
                    "Options:
    help      this help
    poly      render a polygon instead of a triangle
    outline   render outline of the shape instead of fill
    noprint   suppress print output
    <integer> set the canvas size in pixels"
                );
                return;
            }
            "poly" => poly = true,
            "outline" => outline = true,
            "noprint" => noprint = true,
            _ => {
                if let Ok(size) = arg.parse() {
                    shape = (size, size);
                } else {
                    println!("Unknown command line argument: {arg}");
                }
            }
        }
    }

    let mut board = vec![false; shape.0 * shape.1];

    if poly {
        let mut poly = Polygon {
            vertices: vec![[30., 5.], [10., 20.], [15., 30.], [50., 25.]],
        };
        scale(&mut poly, shape.0 as f64 / 64.);
        let (_, time) = measure_time(|| fill_polygon(&mut board, shape, &poly, outline));
        println!("Fill triangle time: {}ms", time * 1e3);
    } else {
        let mut tri = Triangle {
            vertices: [[30., 5.], [10., 20.], [50., 30.]],
        };
        scale(&mut tri, shape.0 as f64 / 64.);
        let (_, time) = measure_time(|| fill_polygon(&mut board, shape, &tri, outline));
        println!("Fill triangle time: {}ms", time * 1e3);
    }

    if !noprint {
        print_board(&board, shape);
    }
}

fn measure_time<T>(f: impl FnOnce() -> T) -> (T, f64) {
    let start = std::time::Instant::now();
    let ret = f();
    (ret, start.elapsed().as_secs_f64())
}
