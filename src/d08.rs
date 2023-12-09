use crate::harness::Harness;

use std::arch::x86_64::*;

pub fn run(h: &mut Harness) {
    // We work under these assumptions that seem to hold for all inputs for this day:
    //  - The cycles in the graph contain exactly one "end" node
    //  - The end node is at the start of the cycle
    //  - The amount of instructions is prime
    //  - The length of every cycle is prime
    h.begin(8)
        .run_part(1, |text| {
            let (instruction_count, right_neighbours, _) = parse(text, false);
            let mut idx = node_index(b"AAA\0");
            let mut count = 0;
            let end = node_index(b"ZZZ\0");
            while idx != end {
                idx = right_neighbours[idx as usize];
                count += 1;
            }
            count * instruction_count
        })
        .run_part(2, |text| {
            let (instruction_count, right_neighbours, mut idxs) = parse(text, true);

            let mut counts = vec![0u64; idxs.len()].into_boxed_slice();
            loop {
                let mut any = false;
                for (idx, count) in idxs
                    .iter_mut()
                    .zip(counts.iter_mut())
                    .filter(|(idx, _)| **idx >> 10 != 26)
                {
                    any = true;
                    *idx = right_neighbours[*idx as usize];
                    *count += 1;
                }
                if !any {
                    break;
                }
            }

            counts
                .iter()
                .fold(instruction_count as u64, |mul, &count| mul * count)
        });
}

fn node_index(name: &[u8]) -> u16 {
    let int = u32::from_le_bytes(name[..4].try_into().unwrap());
    unsafe { _pext_u32(int, 0x00_1F_1F_1F) as u16 }
}

fn parse(text: &str, find_starts: bool) -> (usize, Box<[u16]>, Box<[u16]>) {
    let bytes = text.as_bytes();
    let instruction_count = bytes.iter().position(|&b| b == b'\n').unwrap();
    let mut starts = vec![];
    let mut right_neighbours = vec![0; 1 << 15].into_boxed_slice();
    let mut i = instruction_count + 2;
    while i < bytes.len() {
        let idx = node_index(&bytes[i..]);
        right_neighbours[idx as usize] = node_index(&bytes[i + 12..]);
        if find_starts && idx >> 10 == 1 {
            starts.push(idx);
        }
        i += 17;
    }
    (
        instruction_count,
        right_neighbours,
        starts.into_boxed_slice(),
    )
}
