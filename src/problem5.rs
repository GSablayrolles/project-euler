pub fn main5() -> u64{
    let mut j: u64 = 1;
    let mut res: u64 = 0;
    let mut c1: u64;

    loop {
        c1 = j*20;
        for i in (1..21).rev() {
            if c1 % i != 0 {
                res = 0;
                break 
            }
            res = c1;
        }
        if res != 0 {
            break;
        }
        j += 1;
    };

    return res;
}
