use base::noise::{PermutationTable, open_simplex2};
use base::gen::seeded_rng;
use base::rand::Rand;

pub fn create_sand(seed: u64) -> (Vec<Vec<f32>>, Vec<Vec<(f32, f32, f32)>>) {
    let mut tex_map = vec![Vec::new(); 256];
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut sand_height_map = vec![Vec::new(); 256];

    for i in 0..256 {
        for j in 0..256 {
            let e = (open_simplex2::<f32>(&table, &[(i as f32) * 0.07, (j as f32) * 0.015]) +
                     1.0) / 2.0;
            sand_height_map[i].push(e);
            tex_map[i].push((e, e, e));
        }
    }
    (sand_height_map, tex_map)
}

pub fn create_snow(seed: u64) -> (Vec<Vec<f32>>, Vec<Vec<(f32, f32, f32)>>) {
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut snow_height_map = vec![Vec::new(); 256];
    let mut tex_map = vec![Vec::new(); 256];

    for i in 0..256 {
        for j in 0..256 {
            let e = ((open_simplex2::<f32>(&table, &[(i as f32) * 0.5, (j as f32) * 0.5]) +
                      1.0) / 2.0) +
                    ((open_simplex2::<f32>(&table, &[(i as f32), (j as f32)]) + 1.0) / 2.0) +
                    0.25 *
                    ((open_simplex2::<f32>(&table, &[(i as f32) * 2.0, (j as f32) * 4.0]) + 1.0) /
                     2.0);
            snow_height_map[i].push(e.powf(0.35));
            tex_map[i].push((e, e, e));
        }
    }
    (snow_height_map, tex_map)
}

pub fn create_grass(seed: u64) -> (Vec<Vec<f32>>, Vec<Vec<(f32, f32, f32)>>) {
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut grass_height_map = vec![Vec::new(); 256];
    let mut tex_map = vec![Vec::new(); 256];

    for i in 0..256 {
        for j in 0..256 {
            let e = ((open_simplex2::<f32>(&table, &[(i as f32) * 7.0, (j as f32) * 7.0]) +
                      1.0) / 2.0) +
                    0.5 *
                    ((open_simplex2::<f32>(&table, &[(i as f32) * 9.0, (j as f32) * 9.0]) + 1.0) /
                     2.0);
            grass_height_map[i].push(e.powf(0.37));
            tex_map[i].push((e, e, e));
        }
    }


    (grass_height_map, tex_map)
}

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

pub fn create_height_map_for_snow(seed: u64) -> Vec<Vec<f32>> {
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut snow_height_map = vec![Vec::new(); 256];

    for i in 0..256 {
        for j in 0..256 {
            let e = ((open_simplex2::<f32>(&table, &[(i as f32), (j as f32)]) + 1.0) / 2.0) +
                    0.5 *
                    ((open_simplex2::<f32>(&table, &[(i as f32) * 2.0, (j as f32) * 2.0]) +
                      1.0) / 2.0) +
                    0.25 *
                    ((open_simplex2::<f32>(&table, &[(i as f32) * 4.0, (j as f32) * 4.0]) + 1.0) /
                     2.0);
            snow_height_map[i].push(e.powf(0.35));
        }
    }
    snow_height_map
}


pub fn create_height_map_for_sand(seed: u64) -> Vec<Vec<f32>> {
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut sand_height_map = vec![Vec::new(); 256];

    for i in 0..256 {
        for j in 0..256 {
            let e = (open_simplex2::<f32>(&table, &[(i as f32) * 1.14, (j as f32) * 1.14]) + 1.0) /
                    2.0;
            sand_height_map[i].push(e);
        }
    }
    sand_height_map
}

pub fn create_height_map_for_grass(seed: u64) -> Vec<Vec<f32>> {
    let mut texture_rng = seeded_rng(seed, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut grass_height_map = vec![Vec::new(); 256];

    for i in 0..256 {
        for j in 0..256 {
            let e = ((open_simplex2::<f32>(&table, &[(i as f32) * 7.0, (j as f32)]) + 7.0) / 2.0) +
                    0.5 *
                    ((open_simplex2::<f32>(&table, &[(i as f32) * 9.0, (j as f32) * 9.0]) + 1.0) /
                     2.0);

            grass_height_map[i].push(e.powf(0.35));
        }
    }
    grass_height_map
}
