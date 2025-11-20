#![feature(let_chains)]

use ark_ff::Field;
use ecfft::ec::GoodCurve;
use ecfft::ec::Point;
use ecfft::find_curve::find_curve;

use ark_bn254::Fr as Fp;              // <--- BN254 scalar field
fn main() {
        let mut rng = rand::thread_rng();
        let (n, g) = find_curve::<Fp>(&mut rng, 28);
        let curve = g.curve.unwrap();
        match curve {
            GoodCurve::Odd { a, b } => {
                let bb = b.square();
                let Point { x, y, .. } = g;
                println!("n={n}, a = {a}, bb = {bb}, p=({x}, {y})",)
            }
            GoodCurve::Even { b: _ } => todo!(),
        }
}
