use polygon_filler::{fill_polygon, scale, Board, Polygon, Shape, Triangle};

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
