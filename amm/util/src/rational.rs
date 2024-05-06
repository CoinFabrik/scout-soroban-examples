pub struct Ratio{
    pub num: i128,
    pub den: i128,
}

#[cfg(test)]
use bnum::types::I256;

fn gcd(a: i128, b: i128) -> i128{
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let c = b;
        b = a % b;
        a = c;
    }
    a
}

fn split(x: u128) -> (u128, u128){
    let lo = x & u128::from(u64::MAX);
    let hi = x >> 64;
    (hi, lo)
}

pub fn carrying_mul(l: u128, r: u128) -> (u128, u128){
    let (l_hi, l_lo) = split(l);
    let (r_hi, r_lo) = split(r);
    
    let (carry, x0) = split(r_lo * l_lo);
    let (x2, x1) = split(r_lo * l_hi + carry);

    let (carry, y1) = split(r_hi * l_lo);
    let (y3, y2) = split(r_hi * l_hi + carry);

    let z0 = x0;
    let (carry, z1) = split(x1 + y1);
    let (carry, z2) = split(x2 + y2 + carry);
    let (carry, z3) = split(y3 + carry);

    assert!(carry == 0);
    assert!(z0 <= u128::from(u64::MAX));
    assert!(z1 <= u128::from(u64::MAX));
    assert!(z2 <= u128::from(u64::MAX));
    assert!(z3 <= u128::from(u64::MAX));

    let lo = (z1 << 64) | z0;
    let hi = (z3 << 64) | z2;

    (hi, lo)
}

fn lt(l: (u128, u128), r: (u128, u128)) -> bool{
    if l.0 < r.0{
        return true;
    }
    if l.0 > r.0{
        return false;
    }
    l.1 < r.1
}

fn gt(l: (u128, u128), r: (u128, u128)) -> bool{
    lt(r, l)
}

//Calculates floor(sqrt((x.0 << 128) + x.1))
pub fn sqrt_u256(x: (u128, u128)) -> Result<u128, ()>{
    if x.0 == 0 && x.1 < 2{
        return Ok(x.1);
    }
    let mut lo: u128 = 1;
    let mut hi = lo;
    loop{
        let test = lo + (hi - lo) / 2;
        let sq = carrying_mul(test, test);
        if lt(sq, x){
            lo = test;
            hi = hi.checked_mul(2).unwrap_or(u128::MAX);
            continue;
        }
        if gt(sq, x){
            hi = test;
            break;
        }
        return Ok(test);
    }
    while hi > lo + 1{
        let test = lo + (hi - lo) / 2;
        let sq = carrying_mul(test, test);
        if lt(sq, x){
            lo = test;
            continue;
        }
        if gt(sq, x){
            hi = test;
            continue;
        }
        return Ok(test);
    }
    Ok(lo)
}

fn div(dividend: (u128, u128), divisor: u128) -> Result<u128, bool>{
    if divisor == 0{
        panic!("division by zero");
    }
    let (a, c_prime) = if dividend.0 > 0{
        let m = u128::MAX - divisor + 1;
        (
            (
                (m / divisor).checked_add(1).ok_or(false)?
            )
            .checked_mul(dividend.0).ok_or(true)? //unavoidable overflow in multiplication
        ,
            (m % divisor).checked_mul(dividend.0).ok_or(true)? //unavoidable overflow in multiplication
        )
    }else{
        (0, 0)
    };
    let b = dividend.1 / divisor;
    //unavoidable overflow in addition                                  vvvv
    let c = (c_prime.checked_add(dividend.1 % divisor).ok_or(true)?) / divisor;
    Ok(a
        .checked_add(b)
        .ok_or(false)?
        .checked_add(c)
        .ok_or(false)?)
}

fn safe_mul_internal(x: i128, num: i128, den: i128) -> Result<i128, bool>{
    let flip_sign = (x < 0) != (num < 0);
    let x: u128 = x.abs().try_into().or(Err(false))?;
    let num: u128 = num.abs().try_into().or(Err(false))?;
    let den: u128 = den.abs().try_into().or(Err(false))?;

    let (hi, lo) = carrying_mul(x, num);
    let lo = div((hi, lo), den)?;
    let mut ret: i128 = lo.try_into().or(Err(false))?;
    if flip_sign{
        ret = -ret;
    }
    Ok(ret)
}

pub fn safe_mul(x: i128, num: i128, den: i128) -> Result<i128, ()>{
    return safe_mul_internal(x, num, den).map_err(|_| ());
}

impl Ratio{
    pub fn new(n: i128, d: i128) -> Self{
        if d == 0{
            panic!("division by zero");
        }

        let mut ret = Ratio{
            num: n,
            den: d,
        };
        ret.reduce();
        ret
    }
    fn reduce(self: &mut Self){
        if self.den < 0{
            self.num = -self.num;
            self.den = -self.den;
        }

        let gcd = gcd(self.num, self.den);
        self.num /= gcd;
        self.den /= gcd;
    }
    pub fn compare(self: &Self, other: &Ratio) -> i32{
        let a = self.num * other.den;
        let b = other.num * self.den;
        if a < b{
            -1
        }else if a > b{
            1
        }else{
            0
        }
    }
    pub fn truncating_mul(self: &Self, x: i128) -> i128{
        let result = safe_mul(x, self.num, self.den);
        if result.is_err(){
            panic!("integer overflow")
        }
        result.unwrap()
    }
    pub fn reciprocal(self: &Self) -> Self{
        if self.num == 0{
            panic!("division by zero")
        }
        Self::new(self.den, self.num)
    }
    pub fn mul(self: &Self, other: &Self) -> Self{
        let n = self.num.checked_mul(other.num);
        let d = self.den.checked_mul(other.den);
        if n.is_none() || d.is_none(){
            panic!("integer overflow")
        }
        Self::new(n.unwrap(), d.unwrap())
    }
    pub fn to_i128(self: &Self) -> i128{
        self.num / self.den
    }
}

#[test]
fn test_safe_mul_small(){
    for a in -255..256{
        for b in -255..256{
            for c in 1..256{
                let expected = a * b / c;
                let actual = safe_mul(a, b, c);
                if actual.is_err(){
                    panic!("failed test (error): {}, {}, {}", a, b, c);
                }
                if i128::from(expected) != actual.unwrap(){
                    panic!("failed test ({} != {}): {}, {}, {}", expected, actual.unwrap(), a, b, c);
                }
            }
        }
    }
}

#[cfg(test)]
fn expand_value(n: i16) -> i128{
    let sign = n < 0;
    let n: u16 = n.abs().try_into().unwrap();
    let ret: u128 = n.into();
    let ret = (ret << 8) | ret;
    let ret = (ret << 16) | ret;
    let ret = (ret << 32) | ret;
    let ret = (ret << 64) | ret;
    let ret: i128 = ret.try_into().unwrap();
    if sign{
        -ret
    }else{
        ret
    }
}

#[test]
fn test_safe_mul_big_small(){
    for a in -127_i16..127_i16{
        let expanded_a = expand_value(a);
        for b in -127_i16..127_i16{
            let expanded_b: i128 = b.into();
            for c in 1_i16..127_i16{
                let expanded_c = expand_value(c);
                let expected = I256::from(expanded_a) * I256::from(expanded_b) / I256::from(expanded_c);
                let overflow_expected = expected > I256::from(i128::MAX) || expected < I256::from(i128::MIN);
                let actual = safe_mul_internal(expanded_a, expanded_b, expanded_c);
                if actual.is_err() != ((expected >> 128) > 0.into()){
                    if !actual.is_err() || overflow_expected{
                        panic!("failed test (error): {}, {}, {}", a, b, c);
                    }
                }
                if !actual.is_err() && expected != actual.unwrap().into(){
                    panic!("failed test ({} != {}): {}, {}, {}", expected, actual.unwrap(), a, b, c);
                }
            }
        }
    }
}

#[test]
fn test_safe_mul_small_big(){
    for a in -127_i16..127_i16{
        let expanded_a = a.into();
        for b in -127_i16..127_i16{
            let expanded_b = expand_value(b);
            for c in 1_i16..127_i16{
                let expanded_c = expand_value(c);
                let expected = I256::from(expanded_a) * I256::from(expanded_b) / I256::from(expanded_c);
                let overflow_expected = expected > I256::from(i128::MAX) || expected < I256::from(i128::MIN);
                let actual = safe_mul_internal(expanded_a, expanded_b, expanded_c);
                if actual.is_err() != overflow_expected{
                    if !actual.is_err() || !actual.err().unwrap(){
                        panic!("failed test (error): {}, {}, {}", a, b, c);
                    }
                }
                if !actual.is_err() && expected != actual.unwrap().into(){
                    panic!("failed test ({} != {}): {}, {}, {}", expected, actual.unwrap(), a, b, c);
                }
            }
        }
    }
}

#[test]
fn test_safe_mul_big(){
    for a in -127_i16..127_i16{
        let expanded_a = expand_value(a);
        for b in -127_i16..127_i16{
            let expanded_b = expand_value(b);
            for c in 1_i16..127_i16{
                let expanded_c = expand_value(c);
                let expected = I256::from(expanded_a) * I256::from(expanded_b) / I256::from(expanded_c);
                let overflow_expected = expected > I256::from(i128::MAX) || expected < I256::from(i128::MIN);
                let actual = safe_mul_internal(expanded_a, expanded_b, expanded_c);
                if actual.is_err() != overflow_expected{
                    if !actual.is_err() || !actual.err().unwrap(){
                        panic!("failed test (error {} {}): {}, {}, {}", expected, overflow_expected, a, b, c);
                    }
                }
                if !actual.is_err() && expected != actual.unwrap().into(){
                    panic!("failed test ({} != {}): {}, {}, {}", expected, actual.unwrap(), a, b, c);
                }
            }
        }
    }
}
