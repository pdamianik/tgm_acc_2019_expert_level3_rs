use num::BigUint;
use std::cmp::min;

use crate::generator::*;

#[derive(Debug)]
pub struct LinearCalc {
    pub sum: BigUint,
    pub index: usize,
    pub min: usize,
    pub last_elem: usize,
    pub last_sum: BigUint,
    pub local_buffer: Vec<usize>,
}

impl LinearCalc {
    pub fn new(first_elem: usize, n: Option<usize>) -> Self {
        let mut sum = BigUint::new(vec![]);
        sum += first_elem;

        Self {
            last_sum: sum.clone(),
            sum,
            index: 0,
            min: first_elem,
            last_elem: first_elem,
            local_buffer: Vec::with_capacity(n.map_or(8942944, |n| min(n, 8942944))),
        }
    }

    pub fn next(&mut self, elem: usize) -> BigUint {
        self.index += 1;
        self.sum += elem;

        if elem > self.min {
            if elem < self.last_elem {
                let mut local_min = usize::MAX;
                
                for &elem2 in self.local_buffer.iter().rev() {
                    if elem2 < elem {
                        break;
                    }
                    if elem2 < local_min {
                        local_min = elem2;
                    }
                    self.last_sum -= local_min;
                    self.last_sum += elem;
                }
            }
            self.sum += self.last_sum.clone();
            self.last_sum += elem;
            self.local_buffer.push(elem);
        } else {
            let mut addition = BigUint::new(vec![]);
            addition += elem;
            addition *= self.index;
            self.sum += addition.clone();
            addition += elem;
            self.last_sum = addition.clone();
            self.min = elem;
            self.local_buffer.clear();
        }

        self.last_elem = elem;
        self.sum.clone()
    }
}

pub fn m(n: usize) -> BigUint {
    let mut s = S::new();

    let mut linear_calc = LinearCalc::new(s.next().unwrap(), Some(n));

    for _i in 1..n {
        if _i & u16::MAX as usize == 0 {
            print!("\r{}/{} {}%\x1B[0K", _i, n, (_i as f32)/(n as f32)*(100.0));
        }

        linear_calc.next(s.next().unwrap());
    }

    print!("\r\x1b[02K");

    linear_calc.sum.clone()
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
