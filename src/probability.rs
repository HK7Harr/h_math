use crate::prelude::*;

/// Calculates the number of permutations of selecting r items from a total of n items.
/// The formula for permutations is P(n, r) = n! / (n - r)!.
/// Example usage:
/// let total = 5;
/// let select = 2;
/// let result = h_permutations(&total, &select);
/// The result will be 20, because there are 20 ways to arrange 2 items from a total of 5 items (AB, AC, AD, AE, BA, BC, BD, BE, CA, CB, CD, CE, DA, DB, DC, DE, EA, EB, EC, ED).
pub fn h_permutations<T, S>(total: &T, select: &S) -> u64 
where   
    T: Copy + Into<u32>,
    S: Copy + Into<u32>,
{
    let n: u32 = (*total).into();
    let r: u32 = (*select).into();
    
    return n.h_factorial() / (n - r).h_factorial();
}


/// Calculates the number of combinations of selecting r items from a total of n items.
/// The formula for combinations is C(n, r) = n! / (r! * (n - r)!).
/// Example usage:
/// let total = 5;
/// let select = 2;
/// let result = h_combinations(&total, &select);
/// The result will be 10, because there are 10 ways to choose 2 items from a total of 5 items (AB, AC, AD, AE, BC, BD, BE, CD, CE, DE).
/// Note that combinations do not consider the order of the items, so AB and BA are considered the same combination, while in permutations they are different arrangements.

pub fn h_combinations<T, S>(total: &T, select: &S) -> u64 
where   
    T: Copy + Into<u32>,
    S: Copy + Into<u32>,
{
    let n: u32 = (*total).into();
    let r: u32 = (*select).into();
    
    return n.h_factorial() / (r.h_factorial() * (n - r).h_factorial());
}

