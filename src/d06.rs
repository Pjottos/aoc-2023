use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(6)
        .run_part(1, |text| {
            let mut races = vec![];
            for line in text.lines() {
                let mut parts = line.split_ascii_whitespace();
                let name = parts.next().unwrap();
                if name == "Time:" {
                    races.extend(parts.map(|n| (n.parse::<u32>().unwrap(), 0)));
                } else if name == "Distance:" {
                    races
                        .iter_mut()
                        .zip(parts.map(|n| n.parse::<u32>().unwrap()))
                        .for_each(|(race, n)| race.1 = n);
                }
            }

            let mut product = None;
            for (time, record) in races {
                // Travelled distance can be expressed as:
                // d = x(T - x)
                //   = -x^2 + Tx
                // Where x is the amount of time we hold the button and T is the time allocated for this race
                // We care about the maximum of this function so we solve for the derivative
                // 0 = -2x + T
                // x = (1/2)T
                let optimal_x = time / 2;
                let optimal_d = optimal_x * (time - optimal_x);
                // Now we need to scan what value of x was used for the record
                let mut win_ways = (optimal_d > record) as u32;
                for i in 1..optimal_x {
                    let a = optimal_x - i;
                    let b = optimal_x + i;
                    let distance_a = a * (time - a);
                    let distance_b = b * (time - b);
                    let old = win_ways;
                    if distance_a > record {
                        win_ways += 1;
                    }
                    if distance_b > record {
                        win_ways += 1;
                    }
                    if win_ways == old {
                        break;
                    }
                }
                if let Some(p) = product.as_mut() {
                    *p *= win_ways;
                } else {
                    product = Some(win_ways);
                }
            }

            product.unwrap()
        })
        .run_part(2, |text| {
            let mut lines = text.lines();
            let mut time = lines.next().unwrap().to_string();
            time.retain(|c| c.is_ascii_digit());
            let time = time.parse::<u64>().unwrap();
            let mut record = lines.next().unwrap().to_string();
            record.retain(|c| c.is_ascii_digit());
            let record = record.parse::<u64>().unwrap();
            // Travelled distance can be expressed as:
            // d = x(T - x)
            //   = -x^2 + Tx
            // Where x is the amount of time we hold the button and T is the time allocated for this race
            // We care about the maximum of this function so we solve for the derivative
            // 0 = -2x + T
            // x = (1/2)T
            let optimal_x = time / 2;
            let optimal_d = optimal_x * (time - optimal_x);
            // Now we need to scan what value of x was used for the record
            let mut win_ways = (optimal_d > record) as u32;
            for i in 1..optimal_x {
                let a = optimal_x - i;
                let b = optimal_x + i;
                let distance_a = a * (time - a);
                let distance_b = b * (time - b);
                let old = win_ways;
                if distance_a > record {
                    win_ways += 1;
                }
                if distance_b > record {
                    win_ways += 1;
                }
                if win_ways == old {
                    break;
                }
            }
            win_ways
        });
}
