pub fn main6() -> u128{
    let somme: u128;

    fn sum(n: u128) -> u128 {
        n*(n+1)/2
    }

    fn sum_square(n: u128) -> u128 {
        n*(n+1)*(2*n+1)/6
    }

    somme = sum(100);
    return somme * somme - sum_square(100);
}
