use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(2)
        .run_part(1, |text| {
            let target_red = 12;
            let target_green = 13;
            let target_blue = 14;

            let mut id_sum = 0;

            'game: for line in text.lines() {
                let (game, samples) = line.split_once(':').unwrap();
                let game_id = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();
                for sample in samples.split(';') {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for datum in sample.split(',') {
                        let (raw_num, color) = datum.trim_start().split_once(' ').unwrap();
                        let num = raw_num.parse::<u32>().unwrap();
                        match color {
                            "red" => red += num,
                            "green" => green += num,
                            "blue" => blue += num,
                            _ => panic!("invalid color"),
                        }
                    }

                    if red > target_red || green > target_green || blue > target_blue {
                        continue 'game;
                    }
                }

                id_sum += game_id;
            }

            id_sum
        })
        .run_part(2, |text| {
            let mut power_sum = 0;

            for line in text.lines() {
                let (_, samples) = line.split_once(':').unwrap();
                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;
                for sample in samples.split(';') {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for datum in sample.split(',') {
                        let (raw_num, color) = datum.trim_start().split_once(' ').unwrap();
                        let num = raw_num.parse::<u32>().unwrap();
                        match color {
                            "red" => red += num,
                            "green" => green += num,
                            "blue" => blue += num,
                            _ => panic!("invalid color"),
                        }
                    }
                    max_red = max_red.max(red);
                    max_green = max_green.max(green);
                    max_blue = max_blue.max(blue);
                }

                power_sum += max_red * max_green * max_blue;
            }

            power_sum
        });
}
