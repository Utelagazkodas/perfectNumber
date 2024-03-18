use primal::Sieve;
use is_prime::is_prime;
use num_traits::One;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use std::convert::TryInto;


fn main() {

    let ONE : BigUint = (1 as u32).to_biguint().unwrap();
    let TWO : BigUint = (2 as u32).to_biguint().unwrap();

    let valid : Vec<u32> = vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279, 2203, 2281, 3217];
    let highest = valid[valid.len()-1];

    let mut p : u32 = 1;
    let mut number : BigUint;
    while p < p + 1 {


        number = pow(TWO.clone(), p.to_biguint().unwrap()) - ONE.clone();

        if(p < highest +1 ){
            if(valid.contains(&p)){
                println!("! WE GO(A)T A VALID P: {}", p);
                println!("{}", (number * pow(TWO.clone(), (p - 1).to_biguint().unwrap())).to_string().as_str());

            }
            p += 1;

            continue;
        }


        if is_prime(&(number.to_string().as_str())){
            println!("! WE GO(A)T A VALID P: {}", p);
            println!("{}", (number * pow(TWO.clone(), (p - 1).to_biguint().unwrap())).to_string().as_str());
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