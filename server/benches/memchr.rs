use itertools::Itertools;
use rand::{random_iter, random_range};

const LENS: [usize; 6] = [500,1000,2000,3000,4000,5000];
const SAMPLE_FUNC: [u8; 3] = [128, 34, 190];

fn main() {
    divan::main();
}

#[divan::bench(args = LENS,sample_count = 10000)]
pub fn memchr(len: usize) -> bool {
    let req = gen_param(len);
    memchr::memmem::find(&req,&SAMPLE_FUNC).is_some_and(|e|e == 0)
}

#[divan::bench(args = LENS,sample_count = 10000)]
pub fn normal_search(len: usize) -> bool {
    let req = gen_param(len);
    req.get(0..3).is_some_and(|e|e.eq(&SAMPLE_FUNC))
}

fn gen_param(len: usize) -> Vec<u8> {
    let mut r =vec![0u8;len];
    rand::fill(r.as_mut_slice());
    let mut result = SAMPLE_FUNC.to_vec();
    result.append(&mut r);
    result
}
