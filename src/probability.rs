use crate::prelude::*;

pub fn h_permutations<T, S>(total: &T, select: &S) -> u64 
where   
    T: Copy + Into<u32>,
    S: Copy + Into<u32>,
{
    let n: u32 = (*total).into();
    let r: u32 = (*select).into();
    
    return n.h_factorial() / (n - r).h_factorial();
}

pub fn h_combinations<T, S>(total: &T, select: &S) -> u64 
where   
    T: Copy + Into<u32>,
    S: Copy + Into<u32>,
{
    let n: u32 = (*total).into();
    let r: u32 = (*select).into();
    
    return n.h_factorial() / (r.h_factorial() * (n - r).h_factorial());
}

