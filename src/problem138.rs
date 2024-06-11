pub fn main138() -> u64 {
    let mut sum_l: u64 = 0;
    let mut tot: u64 = 0;
    let mut i: u64 = 6;

    let mut l1: f64;
    let mut l2: f64;

    while tot != 12 {
        let h: u64 = (i + 1) * 2;

        let b1 = h - 1;
        let b2 = h + 1;
        l1 = ((h.pow(2) - (b1 / 2) ^ 2) as f64).sqrt();
        l2 = ((h.pow(2) - (b2 / 2) ^ 2) as f64).sqrt();
        println!("l1 = {}", l1);

        let check_l1 = l1.fract() == 0.0;
        let check_l2 = l2.fract() == 0.0;

        if check_l1 || check_l2 {
            if check_l1 {
                sum_l += (l1 as f64).sqrt() as u64;
                println!("sum = {}", sum_l);
                tot += 1;
            } else {
                sum_l += (l2 as f64).sqrt() as u64;
                println!("sum = {}", sum_l);
                tot += 1;
            }
        } else {
        }
        i += 1;
    }
    return sum_l;
}
