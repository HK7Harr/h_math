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
    T: Copy + Into<u64>,
    S: Copy + Into<u64>,
{
    let n: u64 = (*total).into();
    let r: u64 = (*select).into();
    
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
    T: Copy + Into<u64>,
    S: Copy + Into<u64>,
{
    let n: u64 = (*total).into();
    let r: u64 = (*select).into();
    
    return n.h_factorial() / (r.h_factorial() * (n - r).h_factorial());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_permutations() {
        assert_eq!(h_permutations(&5u32, &2u64), 20);
        assert_eq!(h_permutations(&4u64, &2u32), 12);
    }

    #[test]
    fn test_h_combinations() {
        assert_eq!(h_combinations(&5u32, &2u32), 10);
        assert_eq!(h_combinations(&4u64, &2u64), 6);
    }
}




