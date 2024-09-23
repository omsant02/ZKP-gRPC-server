use num_bigint::BigUint;

//  a^x mod P  ------------------general
//  output = n^exp mod P
pub fn exponentiate(n: &BigUint, exponent: &BigUint, module: &BigUint) -> BigUint{
    n.modpow(exponent, modulus)
}

// -------------------------------bob
//  output = s = k-c*x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q:&BigUint) -> BigUint{
    if *k >= c*x{
        return (k-c*x).modpow(exponent: &BigUint::from(1u32), modulus:q);
    }
    return (q-(k-c*x)).modpow(exponent: &BigUint::from(1u32), modulus:q);
}

//(verifier)  --------------------Alice
// r1 = a^s * y1^c   --cond1
// r2 = b^s * y2^c   --cond2
pub fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, p: &BigUint) -> bool{
    let cond1: bool = *r1 == alpha.modpow(exponent: s, modulus: p) * y1.modpow(exponent: c, modulus: p);
    let cond1: bool = *r2 == beta.modpow(exponent: s, modulus: p) * y2.modpow(exponent: c, modulus: p);
    cond1 && cond2
}
