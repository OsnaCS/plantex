use base::noise::{PermutationTable, open_simplex2};
use base::gen::seeded_rng;
use base::rand::Rand;
use base::world::ground::GroundMaterial;

/// Create texture and height map for given ´GroundMaterial´
pub fn create_texture_maps(ground: GroundMaterial) -> (Vec<Vec<f32>>, Vec<Vec<(f32, f32, f32)>>) {
    let mut tex_map = vec![Vec::new(); 256];
    let mut texture_rng = seeded_rng(2, 13, ());
    let table = PermutationTable::rand(&mut texture_rng);
    let mut height_map = vec![Vec::new(); 256];
    // matches groundmaterial and sets values, so unnecessary calls for simplex are
    // ignored
    // value 0-6 are simplex params
    // value 7 and 8:
    //                  0.0 -> not call of open_simplex2
    //                  0.0 -> call open_simplex2
    // value 9 is the ´height_map´ exponent
    let long = match ground {
        GroundMaterial::Grass => (7.0, 7.0, 9.0, 9.0, 1.0, 1.0, 0.5, 0.0, 3.0),
        GroundMaterial::Sand => (0.05, 0.05, 0.015, 0.015, 1.0, 1.0, 0.5, 0.0, 3.3),
        GroundMaterial::Snow => (0.5, 0.5, 1.0, 1.0, 2.0, 4.0, 1.0, 0.25, 0.35),
        GroundMaterial::Dirt => (0.02, 0.05, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.5),
        GroundMaterial::Stone => (0.05, 0.05, 0.1, 0.1, 1.0, 1.0, 0.5, 0.0, 2.3),
        GroundMaterial::JungleGrass => (7.0, 7.0, 9.0, 9.0, 1.0, 1.0, 0.5, 0.0, 3.0),
        GroundMaterial::Mulch => (0.25, 0.25, 2.5, 2.5, 1.0, 1.0, 0.5, 0.0, 2.3),
        GroundMaterial::Debug => (0.25, 0.25, 2.5, 2.5, 1.0, 1.0, 0.5, 0.0, 2.3),
    };

    // if long.7 or long.8 equals 0.0 then is not open_simplex2 called
    for i in 0..256 {
        for j in 0..256 {
            let e = ((open_simplex2::<f32>(&table, &[(i as f32) * long.0, (j as f32) * long.1]) +
                      1.0) / 2.0) +
                    if long.6 > 0.2 {
                long.6 *
                ((open_simplex2::<f32>(&table, &[(i as f32) * long.2, (j as f32) * long.3]) +
                  1.0) / 2.0)
            } else {
                long.6
            } +
                    if long.7 > 0.2 {
                long.7 *
                ((open_simplex2::<f32>(&table, &[(i as f32) * long.4, (j as f32) * long.5]) +
                  1.0) / 2.0)
            } else {
                long.7
            };
            height_map[i].push(e.powf(long.8));
            tex_map[i].push((e, e, e));
        }
    }

    (height_map, tex_map)
}
