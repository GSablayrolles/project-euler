pub fn main5() -> u64{
    let mut c1 : u64 = 2520;
    let mut res: u64 = 0;

    loop {
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
        c1 += 1;
    };

    return res;
}
