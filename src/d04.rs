use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(4)
        .run_part(1, |text| {
            let mut sum = 0;
            for line in text.lines() {
                let (winning, ours) = line.split_once('|').unwrap();
                let winning = winning
                    .split_once(':')
                    .unwrap()
                    .1
                    .split(' ')
                    .flat_map(|s| s.trim().parse::<u32>().ok())
                    .collect::<Vec<_>>();
                let ours = ours.split(' ').flat_map(|s| s.trim().parse::<u32>().ok());

                let mut score = 1;
                for num in ours {
                    if winning.contains(&num) {
                        score <<= 1;
                    }
                }
                score >>= 1;
                sum += score;
            }
            sum
        })
        .run_part(2, |text| {
            let mut extras = [0; 26];
            let mut sum = 0;
            for (c, line) in text.lines().enumerate() {
                let (winning, ours) = line.split_once('|').unwrap();
                let winning = winning
                    .split_once(':')
                    .unwrap()
                    .1
                    .split(' ')
                    .flat_map(|s| s.trim().parse::<u32>().ok())
                    .collect::<Vec<_>>();
                let ours = ours.split(' ').flat_map(|s| s.trim().parse::<u32>().ok());
                let mut extras_idx = c;
                let our_extras = extras[(c + 25) % 26] + 1;
                sum += our_extras;
                extras[(c + 25) % 26] = 0;
                for num in ours {
                    if winning.contains(&num) {
                        extras[extras_idx % 26] += our_extras;
                        extras_idx += 1;
                    }
                }
            }
            sum
        });
}
