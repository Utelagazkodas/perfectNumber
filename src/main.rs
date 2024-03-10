use primal::Sieve;
use is_prime::is_prime;
use num_traits::One;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use std::convert::TryInto;


fn main() {

    let ONE : BigUint = (1 as u32).to_biguint().unwrap();
    let TWO : BigUint = (2 as u32).to_biguint().unwrap();

    

    let mut p : u32 = 1;
    let mut number : BigUint;
    while p < p + 1 {
        number = pow(TWO.clone(), p.to_biguint().unwrap()) - ONE.clone();

        if is_prime(&(number.to_string().as_str())){
            println!("! WE GO(A)T A VALID P: {}", p);
            //println!("{}", ((2 as u128).pow(p) - 1) * (2 as u128).pow(p -1))
        }
        else {
         //   println!("{} as p is invalid", p);
        }
        p+= 1;
    }
}

//static mut primes : Vec<u128> = primal::;

fn isPrime(number : u128) -> bool {
    let mut i : u128 = 1;
    let mut squareRoot : u128 = (number as f64).sqrt() as u128;
    

    while i <= squareRoot{
        i += 1;
        if(number % i == 0){
            return false;
        }
    }
    return true;
}



fn pow(n: BigUint, exp: BigUint) -> BigUint {
    n.pow(exp.try_into().expect("exponent too large for pow()"))
}