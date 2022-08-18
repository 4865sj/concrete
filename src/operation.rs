use concrete::FheUint16;
use concrete::prelude::*;


pub fn root(ct:FheUint16, one:FheUint16, three:FheUint16) -> FheUint16 {

    let mut a:FheUint16 = ct.clone();
    let mut b:FheUint16 = &ct - &one;
    for i in 0..6 {
        a = mult(a.clone(),&(-b.clone() >> 1 as u16) + &one);
        b = mult(mult(b.clone(),b.clone()),-((&(-b.clone()) + &three) >> 2 as u16));
    }
    a = mult(a.clone(),&(-b.clone() >> 1 as u16) + &one);

    a
}

pub fn divide(ct:FheUint16,n: usize) -> FheUint16 {

    let p:FheUint16 = ct >> n as u16;
    let res:FheUint16 = &p.clone() - &((p.clone() >> 15-n as u16) << 16-n as u16);

    res
}

pub fn add_many(ct: Vec<FheUint16>) -> FheUint16 {


    let mut res: FheUint16 = &ct[0].clone() + &ct[1].clone();
    for i in 2..ct.len() {
        res = &res.clone() + &ct[i].clone();
    }

    res
}

pub fn mult(ct1:FheUint16, ct2:FheUint16) -> FheUint16 {

    let p: FheUint16 = (ct1.clone() * &ct2.clone()) >> 7 as u16;
    let res: FheUint16 = &p.clone() - &((p.clone() >> 8 as u16) << 9 as u16);

    res
}

pub fn encode_cleartext(clear_a: f64) -> u16 {
    // 1. encoding param
    let base: f64 = 2.;
    let param_mod: f64 = base.powf(16.);
    let param_scale: f64 = base.powf(7.);
    let param_mid: f64 = base.powf(15.);
  
    // 2. scale
    let mut plain_a: f64 = clear_a * param_scale; // 0.5342 * 1000 = 534.2 -> round
    plain_a = plain_a.round(); // 534

    if clear_a < 0. {
        plain_a += param_mod;
    }

    // 3. convert into u16
    let plain_a = plain_a as u16;

    plain_a
}

pub fn decode_plaintext(plain_res: u16) -> f64 {

    // param
    let base: f64 = 2.;
    let param_mod: f64 = base.powf(16.);
    let param_scale: f64 = base.powf(7.);
    let param_mid: f64 = base.powf(15.);

    // 1. convert into f32
    let mut plain_res = plain_res as f64;

    // 2. scale
    if param_mid <= plain_res && plain_res < param_mod {
        plain_res -= param_mod;
    }
    let res:f64 = plain_res * base.powf(-7.);

    res
}

