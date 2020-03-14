use base::math::*;

/// Converts a height map into a normal map
pub fn convert(map: Vec<Vec<f32>>, scale: f32) -> Vec<Vec<(f32, f32, f32)>> {
    let mut normals: Vec<Vec<(f32, f32, f32)>> =
        vec![vec![(0.0, 0.0, 0.0); map[0].len() as usize]; map.len() as usize];
    let mut n: Vector3f = Vector3f::new(0.0, 0.0, 0.0);
    let mut arr;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            arr = neighbours(&map, (i as i32, j as i32));
            n.x = scale
                * -(arr[8 as usize] - arr[6 as usize]
                    + 2.0 * (arr[5 as usize] - arr[3 as usize])
                    + arr[2 as usize]
                    - arr[0 as usize]);
            n.y = scale
                * -(arr[0 as usize] - arr[6 as usize]
                    + 2.0 * (arr[1 as usize] - arr[7 as usize])
                    + arr[2 as usize]
                    - arr[8 as usize]);
            n.z = 1.0;
            n = n.normalize();
            normals[i as usize][j as usize] = (n.x, n.y, n.z);
        }
    }
    normals
}

/// Finds Moore (8-Way) Neighbours for a position in a map
fn neighbours(map: &Vec<Vec<f32>>, pos: (i32, i32)) -> [f32; 9] {
    let ref tmap = *map;
    let mut iter = 0;
    let mut arr: [f32; 9] = [0.0; 9];

    for p in 0..3 {
        for q in 0..3 {
            // println!("{}{}", p, q);
            if tmap.get((pos.0 + p - 1) as usize).is_none()
                || tmap[(pos.0 + p - 1) as usize]
                    .get((pos.1 + q - 1) as usize)
                    .is_none()
            {
                arr[iter] = tmap[pos.0 as usize][pos.1 as usize];
            } else {
                arr[iter] = tmap[(pos.0 + p - 1) as usize][(pos.1 + q - 1) as usize];
            }
            iter += 1;
        }
    }
    // info!("{:?}", arr);
    arr
}
