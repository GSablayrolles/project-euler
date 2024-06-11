pub fn main138() -> u64 {
    let mut i = 3;
    let mut n: u64 = 1;
    let mut premier = vec![2];

    while n < 10001 {
        let mut estpremier: bool = true;
        for &p in &premier {
            if i % p == 0 {
                estpremier = false;
                break;
            }
        }
        if estpremier {
            premier.push(i);
            n += 1;
        }
        i = i + 1;
    }
    return premier[10000];
}
