use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(9)
        .run_part(1, |text| {
            let mut sum = 0;
            for line in text.lines() {
                let mut nums = line
                    .split(' ')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                let mut lasts = vec![];
                while !nums.iter().all(|&n| n == 0) {
                    for n in 0..nums.len() - 1 {
                        nums[n] = nums[n + 1] - nums[n];
                    }
                    lasts.push(nums.pop().unwrap());
                }
                for num in lasts.iter().rev() {
                    sum += num;
                }
            }
            sum
        })
        .run_part(2, |text| {
            let mut sum = 0;
            for line in text.lines() {
                let mut nums = line
                    .split(' ')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                nums.reverse();
                let mut firsts = vec![];
                while !nums.iter().all(|&n| n == 0) {
                    for n in 0..nums.len() - 1 {
                        nums[n] = nums[n] - nums[n + 1];
                    }
                    firsts.push(nums.pop().unwrap());
                }
                let mut next = 0;
                for num in firsts.iter().rev() {
                    next = num - next;
                }
                sum += next;
            }
            sum
        });
}
