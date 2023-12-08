use crate::harness::Harness;

use num::integer::lcm;

use std::collections::HashMap;

pub fn run(h: &mut Harness) {
    h.begin(8)
        .run_part(1, |text| {
            let mut nodes = HashMap::new();
            let mut lines = text.lines();
            let instructions = lines.next().unwrap().as_bytes();
            lines.next();
            for line in lines {
                let (node, targets) = line.split_once('=').unwrap();
                let (l, r) = targets.split_once(',').unwrap();
                let l = l.strip_prefix(" (").unwrap();
                let r = r.trim().strip_suffix(")").unwrap();
                nodes.insert(node.trim(), (l, r));
            }
            let mut key = "AAA";
            let mut count = 0;
            while key != "ZZZ" {
                let dir = instructions[count % instructions.len()];
                let paths = nodes[key];
                if dir == b'L' {
                    key = paths.0;
                } else {
                    key = paths.1;
                }
                count += 1;
            }
            count
        })
        .run_part(2, |text| {
            let mut nodes = HashMap::new();
            let mut lines = text.lines();
            let instructions = lines.next().unwrap().as_bytes();
            lines.next();
            for line in lines {
                let (node, targets) = line.split_once('=').unwrap();
                let (l, r) = targets.split_once(',').unwrap();
                let l = l.strip_prefix(" (").unwrap();
                let r = r.trim().strip_suffix(")").unwrap();
                nodes.insert(node.trim(), (l, r));
            }

            let mut keys = nodes
                .keys()
                .copied()
                .filter(|key| key.ends_with("A"))
                .collect::<Vec<_>>();
            let mut counts = vec![0; keys.len()];
            //println!("{count} {keys:?}");
            loop {
                let mut any = false;
                for (key, count) in keys
                    .iter_mut()
                    .zip(counts.iter_mut())
                    .filter(|(key, _)| !key.ends_with("Z"))
                {
                    let dir = instructions[*count % instructions.len()];
                    any = true;
                    let paths = nodes[*key];
                    if dir == b'L' {
                        *key = paths.0;
                    } else {
                        *key = paths.1;
                    }
                    *count += 1;
                }
                //println!("{count} {keys:?}");
                if !any {
                    break;
                }
            }
            counts.iter().fold(1, |mul, &count| lcm(mul, count))
        });
}
