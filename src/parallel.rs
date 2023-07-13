use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread::{available_parallelism, spawn};
use num::{BigUint, Zero};

use crate::generator::*;

pub fn m(n: usize) -> BigUint {
    let s = Arc::new(S::new().take(n).map(|val| val as u32).collect::<Vec<_>>());

    let last_j = Arc::new(AtomicUsize::new(0));

    let available_cores: usize = available_parallelism().unwrap().into();
    let mut handels = Vec::with_capacity(available_cores);

    for _ in 0..available_parallelism().unwrap().into() {
        let s = s.clone();
        let last_j = last_j.clone();

        handels.push(spawn(move || {
            let mut sum = BigUint::zero();

            loop {
                let j = last_j.fetch_add(1, Ordering::Relaxed);
                if j >= n {
                    break;
                }

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
