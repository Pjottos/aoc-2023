use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(9)
        .run_part(1, |text| {
            let mut sum = 0;
            let mut nums = vec![];
            let mut num = 0;
            let mut sign = false;
            for &b in text.as_bytes() {
                if b == b'-' {
                    sign = true;
                } else if b == b' ' {
                    nums.push(if sign { -num } else { num });
                    num = 0;
                    sign = false;
                } else if b != b'\n' {
                    num = (num * 10) + i32::from(b - b'0');
                } else {
                    nums.push(if sign { -num } else { num });
                    num = 0;
                    sign = false;

                    while nums.iter().any(|&n| n != 0) {
                        for n in 0..nums.len() - 1 {
                            nums[n] = nums[n + 1] - nums[n];
                        }
                        sum += nums.pop().unwrap();
                    }
                    nums.clear();
                }
            }
            sum
        })
        .run_part(2, |text| {
            let mut sum = 0;
            let mut nums = vec![];
            let mut num = 0;
            let mut sign = false;
            for &b in text.as_bytes() {
                if b == b'-' {
                    sign = true;
                } else if b == b' ' {
                    nums.push(if sign { -num } else { num });
                    num = 0;
                    sign = false;
                } else if b != b'\n' {
                    num = (num * 10) + i32::from(b - b'0');
                } else {
                    nums.push(if sign { -num } else { num });
                    num = 0;
                    sign = false;

                    nums.reverse();

                    while nums.iter().any(|&n| n != 0) {
                        for n in 0..nums.len() - 1 {
                            nums[n] = nums[n + 1] - nums[n];
                        }
                        sum += nums.pop().unwrap();
                    }
                    nums.clear();
                }
            }
            sum
        });
}
