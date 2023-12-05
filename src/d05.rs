use crate::harness::Harness;

use std::{collections::BTreeMap, ops::Bound};

pub fn run(h: &mut Harness) {
    h.begin(5)
        .run_part(1, |text| {
            let mut map = BTreeMap::<u32, (u32, u32)>::new();

            let mut lines = text.lines();
            let mut nums = lines
                .next()
                .unwrap()
                .strip_prefix("seeds: ")
                .unwrap()
                .split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            lines.next();
            while lines.next().is_some() {
                map.clear();
                for line in lines.by_ref() {
                    if line.is_empty() {
                        break;
                    }
                    let mut parts = line.split(' ').map(|n| n.parse::<u32>().unwrap());
                    let dest_start = parts.next().unwrap();
                    let source_start = parts.next().unwrap();
                    let len = parts.next().unwrap();
                    map.insert(source_start, (dest_start, len));
                }

                for num in nums.iter_mut() {
                    if let Some((&source_start, &(dest_start, len))) =
                        map.upper_bound(Bound::Included(&num)).key_value()
                    {
                        let offset = *num - source_start;
                        if offset < len {
                            *num = dest_start + offset;
                        }
                    }
                }
            }

            *nums.iter().min().unwrap()
        })
        .run_part(2, |text| {
            let mut lines = text.lines();
            let mut ranges = lines
                .next()
                .unwrap()
                .strip_prefix("seeds: ")
                .unwrap()
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .array_chunks::<2>()
                .map(|[s, l]| [s, s + l])
                .collect::<Vec<_>>();
            let mut map = BTreeMap::<u64, (u64, u64)>::new();

            lines.next();
            while lines.next().is_some() {
                map.clear();
                for line in lines.by_ref() {
                    if line.is_empty() {
                        break;
                    }
                    let mut parts = line.split(' ').map(|n| n.parse::<u64>().unwrap());
                    let dest_start = parts.next().unwrap();
                    let source_start = parts.next().unwrap();
                    let len = parts.next().unwrap();
                    map.insert(source_start, (dest_start, len));
                }

                let mut i = 0;
                while i < ranges.len() {
                    let [mut start, mut end] = ranges[i];
                    if let Some((&source_start, &(dest_start, len))) =
                        map.upper_bound(Bound::Excluded(&end)).key_value()
                    {
                        let source_end = source_start + len;
                        if start >= source_end {
                            i += 1;
                            continue;
                        }
                        // Split the range 0, 1 or 2 times
                        // We push ranges as we're going but we only use the mapping for the current range
                        // as the new split range is not in the same mapping range
                        if end > source_end {
                            ranges.push([source_end, end]);
                            end = source_end;
                        }
                        if start < source_start {
                            ranges.push([start, source_start]);
                            start = source_start;
                        }
                        let end_offset = end - source_start;
                        let start_offset = start - source_start;
                        start = dest_start + start_offset;
                        end = dest_start + end_offset;
                        ranges[i] = [start, end];
                    }

                    i += 1;
                }
            }

            *ranges.iter().map(|[start, _]| start).min().unwrap()
        });
}
