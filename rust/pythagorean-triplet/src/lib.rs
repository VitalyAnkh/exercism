use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    for x in 1..sum {
        for y in x..sum {
            for z in y..sum {
                if x + y + z == sum && x * x + y * y == z * z {
                    result.insert([x, y, z]);
                }
            }
        }
    }
    result
}
