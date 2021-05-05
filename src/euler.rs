use rand::prelude::*;

pub fn estimate_e() {
    let mut rng = thread_rng();
    let total = 1_000_000;
    let mut total_selections = 0;

    for _ in 1..total {
        let mut sum = 0f32;
        let mut i = 0;
        total_selections += loop {
            sum += rng.gen::<f32>();
            i += 1;
            if sum > 1f32 {
                break i;
            }
        };
    }
    println!("Estimated e: {}", total_selections as f32 / total as f32);
}
