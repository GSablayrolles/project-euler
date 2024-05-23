pub fn main4() -> u128{
    let c1 : u128 = 1000;
    let c2: u128 = 1000;
    let mut res: u128;
    let mut max: u128 = 0;

    
    for i in (1..c1).rev() {
        for j in (1..c2).rev() {
            let mut n = i * j;
            res = n;
            let mut reversed = 0;

            while n != 0 {
                reversed = reversed * 10 + n % 10;
                n /= 10;
            }

            if res == reversed && res > max {
                max = res;
            }
        }
    }

    return max;
}
