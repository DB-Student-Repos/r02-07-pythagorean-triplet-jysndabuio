use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // unimplemented!("Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c");
    let mut triples = HashSet::new();

    // Using Euclid Formula
    // a + b + c = N, where a = m²-n²; b = 2mn; c = m² + n²;
    // The conditions : m > n ; Max value of m is sqrt(N);

    let max_value_m = (sum as f32).sqrt() as u32;

    for m in 2..=max_value_m {
        for n in 1..m {
            let mut a:u32 =   m*m - n*n;
            let mut b:u32 = 2 * m * n;
            let mut c:u32 = m*m + n*n;
            let total:u32 = a + b + c;
            if total == sum {
                let triplet = [a,b,c];
                triples.insert(triplet);
            }
        }
    }

    triples
}
