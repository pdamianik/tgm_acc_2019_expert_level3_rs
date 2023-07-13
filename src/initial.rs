use num::{BigUint, Zero};

fn s(o: usize) -> usize {
    let mut result = 290797;

    for _ in 1..=o {
        result = result * result % 50515093;
    }

    result
}

fn a(i: usize, j: usize) -> BigUint {
    let mut min = s(i);

    for idx in (i + 1)..=j {
        let elem = s(idx);

        if elem < min {
            min = elem;
        }
    }

    min.into()
}

pub fn m(n: usize) -> BigUint {
    let mut sum = BigUint::zero();

    for j in 1..=n {
        for i in 1..=j {
            sum += a(i, j);
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_m_10() {
        assert_eq!(m(10), 432256955usize.into());
    }
}

