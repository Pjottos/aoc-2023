use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(6)
        .run_part(1, |text| {
            // Any overhead hurts on such a tiny amount of computation
            let bytes = text.as_bytes();
            assert_eq!(bytes.len(), 74);
            let mut times = [0; 4];
            let mut records = [0; 4];
            for r in 0..4 {
                let mut num = 0;
                for d in 0..4 {
                    let digit = bytes[11 + (r * 7) + d] & 0x0F;
                    num = (num * 10) + u32::from(digit);
                }
                times[r] = num;
                num = 0;
                for d in 0..4 {
                    let digit = bytes[37 + 11 + (r * 7) + d] & 0x0F;
                    num = (num * 10) + u32::from(digit);
                }
                records[r] = num;
            }

            let mut product = 1;
            for i in 0..4 {
                let time = u64::from(times[i]);
                let record = u64::from(records[i]);
                product *= win_ways(time, record);
            }

            product
        })
        .run_part(2, |text| {
            let bytes = text.as_bytes();
            assert_eq!(bytes.len(), 74);

            let mut time = 0;
            for i in 11..36 {
                if bytes[i] & 0x10 != 0 {
                    time = (time * 10) + u64::from(bytes[i] & 0xF)
                }
            }
            let mut record = 0;
            for i in 37 + 11..74 {
                if bytes[i] & 0x10 != 0 {
                    record = (record * 10) + u64::from(bytes[i] & 0xF)
                }
            }

            win_ways(time, record)
        });
}

fn win_ways(time: u64, record: u64) -> u64 {
    // Travelled distance can be expressed as:
    // d = x(T - x)
    //   = -x^2 + Tx
    // Where x is the amount of time we hold the button and T is the time allocated for this race
    // We want to split this function in 2 monotonic parts, so we solve for the derivative
    // 0 = -2x + T
    // 2x = T
    // x = (1/2)T

    // This is the theoretical best time to hold the button but we have to account for the fact it may not
    // be integer. It may be 0.5 below the actual best time
    let optimal_x = time / 2;
    // Now we find the highest integer time that beats the record
    let max_bit = optimal_x.ilog2();
    let mut x = optimal_x;
    for bit in (0..=max_bit).rev() {
        let a = x + (1 << bit);
        let distance = a * (time - a);
        if distance > record {
            x = a;
        }
    }
    // For even time values we can also attain optimal_x
    ((x - optimal_x) * 2) + ((time + 1) & 1)
}
