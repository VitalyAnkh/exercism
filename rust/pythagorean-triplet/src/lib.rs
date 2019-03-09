#![feature(euclidean_division)]
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    let bound = sum.div_euclid(2);
    for x in 1..bound {
        for y in x..sum {
            let z = sum - x - y;
            if z > y {
                if x * x + y * y == z * z {
                    result.insert([x, y, z]);
                }
            } else {
                break;
            };
        }
    }
    result
}
