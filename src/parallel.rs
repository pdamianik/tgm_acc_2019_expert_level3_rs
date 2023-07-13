use std::{sync::Arc, thread::{available_parallelism, spawn}};
use num::{BigUint, Zero};

use crate::generator::*;

pub fn m(n: usize) -> BigUint {
    let s = Arc::new(S::new().take(n).map(|val| val as u32).collect::<Vec<_>>());

    let available_cores: usize = available_parallelism().unwrap().into();
    let mut handels = Vec::with_capacity(available_cores);

    for i in 0..available_cores {
        let s = s.clone();

        handels.push(spawn(move || {
            let mut sum = BigUint::zero();

            for j in (i..n).step_by(available_cores) {
                let mut min = s[j] as usize;
                sum += min;

                for i in (0..j).rev() {
                    if (s[i] as usize) < min {
                        min = s[i] as usize;
                    }
                    sum += min;
                }
            }

            sum
        }));
    }

    let mut sum = BigUint::zero();
    for handel in handels {
        sum += handel.join().unwrap();
    }

    sum
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
