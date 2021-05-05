use rand::prelude::*;

pub fn estimate_pi() {
    let total = 1_000_000;
    let mut count = 0;
    let mut rng = thread_rng();
    for _ in 1..total {
        let x = (2.0 * rng.gen::<f32>()) - 1.0;
        let y = (2.0 * rng.gen::<f32>()) - 1.0;
        if (x * x + y * y).sqrt() < 1.0 {
            count += 1;
        }
    }
    println!("Estimated Pi: {}", 4.0 * count as f32 / total as f32);
}
