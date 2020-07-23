use std::thread;
use std::time;

fn main() {
    let (mut A, mut B): (f64, f64) = (0.0, 0.0);

    loop {
        A += 0.07;
        B += 0.03;
        let (sinA, cosA, sinB, cosB, mut b) = (
            A.sin(),
            A.cos(),
            B.sin(),
            B.cos(),
            [' '; 1760],);

        let (mut z, mut j): ([f64; 1760], f64) =(
            [0.0; 1760],
            0.0,
        );
        while j <= 6.28 {
            let (u, v) = (j.sin(),j.cos());
            let mut i: f64 = 0.0;
            while i <= 6.28 {
                let (w, c) = (i.sin(), i.cos());
                let h = v + 2.0;
                let (d, t) = (1.0 / (w * h * sinA + u * cosA + 5.0), w * h * cosA - u * sinA);
                let (x, y) = (
                    (40.0 + 30.0 * d * (c * h * cosB - t * sinB)) as usize,
                    (12.0 + 15.0 * d * (c * h * sinB + t * cosB)) as usize,
                );
                let (o, n) = (
                    x + 80 * y,
                    8.0 * ((u * sinA - w * v * cosA) * cosB - w * v * sinA - u * cosA - c * v * sinB),
                );
                if y < 22 && x < 79 && d > z[o] {
                    z[o] = d;
                    b[o] = (".,-~:;=".to_owned() + "!*#$@")
                        .chars()
                        .nth(n as usize)
                        .or(Some('.'))
                        .unwrap();
                }
                i += 0.02
            }
            j += 0.07
        }
        print!(
            "\x1B[H{}",
            b.chunks(80)
                .map(|l| l.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        );
        thread::sleep(time::Duration::from_millis(20));
    }
}
