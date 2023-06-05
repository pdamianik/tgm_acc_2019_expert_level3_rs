use num::BigUint;

use crate::generator::*;
use crate::linear::LinearCalc;

pub fn m(n: usize) -> BigUint {
    let mut s = S::new();
    let n = n - 1;

    // Search for minimum and minimum cycle
    let (min, min_index, min_cycle, linear_sums) = {
        let mut min = s.next().unwrap();
        let mut min_index = 0;
        let min_cycle: usize;
        let mut i = 0;

        let mut linear_calc = LinearCalc::new(min, Some(n + 1));
        let mut linear_sums = vec![linear_calc.sum.clone()];

        loop {
            i += 1;
            let elem = s.next().unwrap();
            linear_sums.push(linear_calc.next(elem));

            if i == n {
                // Return early if n is smaller than min_cycle
                return linear_sums.last().unwrap().clone();
            } else if elem < min {
                // Found new minimum
                min = elem;
                min_index = i;
            } else if elem == min {
                // Found minimum cycle
                min_cycle = i;
                break;
            }
        }

        // Verify the cycle
        #[cfg(debug_assertions)]
        {
            let target = min_cycle * 2 - min_index;

            loop {
                i += 1;
                let elem = s.next().unwrap();

                if elem == min {
                    assert_eq!(i, target, "Cycle is invalid (expected cycle at {}, found cycle at {}, distance = {})", target, i, i - target);
                    break;
                }
            }
        }

        (min, min_index, min_cycle, linear_sums)
    };

    let cycle_width = min_cycle - min_index;
    let first_min_distance = n - min_index;
    let cycle_index = first_min_distance / cycle_width;
    let cycle_local_index = first_min_distance % cycle_width;
    let mut cycle_local_sum = linear_sums[min_index + cycle_local_index].clone();

    // Add sums consistent for every cycle + global minima for the prefix
    cycle_local_sum += {
        let mut first_cycle_sum = linear_sums[min_cycle - 1].clone();
        let prefix_sum = linear_sums[min_index - 1].clone();

        first_cycle_sum -= prefix_sum;
        first_cycle_sum *= cycle_index;

        first_cycle_sum
    };

    // Add missing global minima for cycles in between
    cycle_local_sum += {
        let mut min_fill_step = BigUint::new(vec![]);

        min_fill_step += min;
        min_fill_step *= cycle_width;
        min_fill_step *= cycle_width;

        let mut cycle_min_fill = BigUint::new(vec![]);

        cycle_min_fill += cycle_index - 1;
        cycle_min_fill *= min_fill_step.clone();
        min_fill_step *= 2usize;
        cycle_min_fill += min_fill_step;
        cycle_min_fill *= cycle_index;
        cycle_min_fill /= 2usize;

        let mut min_fill_overflow = BigUint::new(vec![]);
        min_fill_overflow += min;
        min_fill_overflow *= cycle_index * cycle_width;
        min_fill_overflow *= cycle_width - 1 - cycle_local_index;

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
