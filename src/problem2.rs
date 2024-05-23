fn main() {
    let mut f: u32 = 1;
    let mut f_2: u32 = 1;
    let mut sum: u32 = 0;

    while f < 4e6 as u32 {
        let old: u32 = f;

        if f % 2 == 0 {
            sum += f;
        }

        f += f_2;
        f_2 = old;
    }

    println!("Somme : {sum}");
}
