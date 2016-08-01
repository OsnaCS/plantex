use base::noise::{PermutationTable, open_simplex2};
use base::gen::seeded_rng;
use base::rand::Rand;


pub fn create_noise(seed: u64) -> Vec<Vec<(f32, f32, f32)>> {
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);

    let mut v = vec![Vec::new(); 256];
    for i in 0..256 {
        for j in 0..256 {
            let s = (open_simplex2::<f32>(&table, &[(i as f32) * 0.25, (j as f32) * 0.25]) + 1.0) /
                    2.0;
            v[i].push((s, s, s));
        }
    }
    v
}
