// I just Math'd this one.

fn quadratic_equation(a: f64, b: f64, c: f64) -> [f64; 2] {
    [
        (-b + (b.powi(2) - (4.0*a*c)).sqrt()) / (2.0*a),
        (-b - (b.powi(2) - (4.0*a*c)).sqrt()) / (2.0*a)
    ]
}

static TIMES : [f64; 4] = [44.0, 70.0, 70.0, 80.0];
static DISTS : [f64; 4] = [283.0, 1134.0, 1134.0, 1491.0];

fn main() {
    // total_time = time_press + time_move
    // distance < time_move * time_press
    // -t_p^2 + T*t_p - D > 0
    // Solve quadratic

    let part1 : u64 = TIMES.iter()
        .zip(DISTS.iter())
        .map(|(t,d)| quadratic_equation(-1.0, *t, -d))
        .map(|[t1, t2]| (t1.max(t2).floor() - t1.min(t2).max(0.0).ceil() + 1.0) as u64)
        .product();

    println!("Part 1: {:?}", part1);

    let [t1, t2] = quadratic_equation(-1.0, 44707080.0, -283113411341491.0);
    let part2 = (t1.max(t2).floor() - t1.min(t2).max(0.0).ceil() + 1.0) as u64;
    println!("Part 2: {:?}", part2);
}
