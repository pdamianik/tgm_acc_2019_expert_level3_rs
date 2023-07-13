use num::BigUint;
use rayon::prelude::*;

use crate::generator::*;

pub fn m(n: usize) -> BigUint {
    let s = S::new().take(n).map(|val| val as u32).collect::<Vec<_>>();

    (0..n).into_par_iter().map(|j| {
        (0..=j).rev().scan(s[j], |min, i| -> Option<BigUint> {
            let elem = s[i];

            if elem < *min {
                *min = elem;
            }

            Some((*min).into())
        }).sum::<BigUint>()
    }).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn m_test_10() {
        assert_eq!(m(10), 432256955usize.into());
    }

    #[test]
    fn m_test_10000() {
        assert_eq!(m(10000), 3264567774119usize.into());
    }
}
