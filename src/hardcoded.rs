use num::BigUint;

use crate::generator::*;
use crate::{linear, linear::LinearCalc};

const MIN: usize = 3;
const MIN_INDEX: usize = 2633996;
const MIN_CYCLE: usize = 8942944;
const CYCLE_WIDTH: usize = 6308948;
const FIRST_CYCLE_SUM: usize = 4638033462857199;
const MIN_FILL_STEP: usize = 119408474600112;

pub fn m(n: usize) -> BigUint {
    if n < MIN_INDEX {
        return linear::m(n);
    }

    let n = n - 1;
    let first_min_distance = n - MIN_INDEX;
    let cycle_index = first_min_distance / CYCLE_WIDTH;
    let cycle_local_index = first_min_distance % CYCLE_WIDTH;

    let mut cycle_local_sum = {
        let mut s = S {
            value: 3,
        };

        let mut linear_calc = LinearCalc {
            sum: 1723152794732772usize.into(),
            index: 2633996,
            min: 3,
            last_elem: 3,
            last_sum: 7901991usize.into(),
            local_buffer: Vec::with_capacity(8942944),
        };

        for _ in 0..cycle_local_index {
            linear_calc.next(s.next().unwrap());
        }

        if n < MIN_CYCLE {
            return linear_calc.sum.clone();
        }

        linear_calc.sum.clone()
    };

    cycle_local_sum += {
        let mut first_cycle_sum: BigUint = FIRST_CYCLE_SUM.into();
        first_cycle_sum *= cycle_index;

        first_cycle_sum
    };

    cycle_local_sum += {
        let mut min_fill_step: BigUint = MIN_FILL_STEP.into();

        let mut cycle_min_fill: BigUint = (cycle_index - 1).into();

        cycle_min_fill *= min_fill_step.clone();
        min_fill_step *= 2usize;
        cycle_min_fill += min_fill_step;
        cycle_min_fill *= cycle_index;
        cycle_min_fill /= 2usize;

        let mut min_fill_overflow: BigUint = MIN.into();
        min_fill_overflow *= cycle_index * CYCLE_WIDTH;
        min_fill_overflow *= CYCLE_WIDTH - 1 - cycle_local_index;

        cycle_min_fill -= min_fill_overflow;

        cycle_min_fill
    };

    cycle_local_sum
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

    #[cfg(not(debug_assertions))]
    #[test]
    fn m_test_2000000000() {
        assert_eq!(m(2000000000), 7435327983715286168usize.into());
    }
}
