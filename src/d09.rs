use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(9)
        .run_part(1, run_part::<false>)
        .run_part(2, run_part::<true>);
}

fn run_part<const PART2: bool>(text: &str) -> i64 {
    // In the naive implementation, we effectively compute the values of the derivative recursively
    // until we calculated the derivative of a constant (differences are all 0). For the purpose of extrapolation
    // we conjure a polynomial that all the initial numbers lay on. It then becomes clear that at each iteration
    // we extract the lowest order coefficient of the current derivative polynomial. So really we are extracting
    // the coefficients of the final polynomial from low to high degree. The last initial number is the lowest
    // order coefficient and thus the value of this polynomial at x = 0. We are interested in the value at x = 1
    // which is equivalent to the sum of the coefficients.
    //
    // So, how do we find the coefficients efficiently? We can use Lagrange interpolation with some precomputing.

    let mut sums = [0; 21];
    let mut num = 0;
    let mut num_idx = if PART2 { 0 } else { 20 };
    let mut sign = false;
    // Everything is squishy and variable in today's input so SIMD is impractical
    for &b in text.as_bytes() {
        if b == b'-' {
            sign = true;
        } else if b == b' ' || b == b'\n' {
            if sign {
                num = -num;
            }
            sums[num_idx] += num;

            num = 0;
            sign = false;

            if b == b'\n' {
                if PART2 {
                    num_idx = 0;
                } else {
                    num_idx = 20;
                }
            } else {
                if PART2 {
                    num_idx += 1;
                } else {
                    num_idx -= 1;
                }
            }
        } else {
            // Potential overflow
            num = (num * 10) + i32::from(b - b'0');
        }
    }

    sums.iter()
        .zip(LAGRANGE.iter())
        .map(|(&sum, &coefficient)| i64::from(sum) * i64::from(coefficient))
        .sum::<i64>()
}

// Could potentially be multiple tables selected by the amount of numbers in a sequence
const LAGRANGE: [i32; 21] = calc_lagrange::<21>();

const fn calc_lagrange<const N: usize>() -> [i32; N] {
    let mut buf = [0i32; N];
    let mut i = 0;
    while i < N {
        let mut j = 0;
        let mut nominator = 1;
        let mut denominator = 1;
        let xi = -(i as i128);
        while j < N {
            if j == i {
                j += 1;
                continue;
            }
            let xj = -(j as i128);
            nominator *= 1 - xj;
            denominator *= xi - xj;
            j += 1;
        }
        // We could handle fractional reciprocals with fixed point but if we don't have to...
        // We can't assert in const context
        //assert_eq!(nominator % denominator, 0);
        //i32::try_from(nominator / denominator).unwrap()
        buf[i as usize] = (nominator / denominator) as i32;

        i += 1;
    }

    buf
}
