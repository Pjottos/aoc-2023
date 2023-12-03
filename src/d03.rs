use crate::harness::Harness;

use std::iter;

pub fn run(h: &mut Harness) {
    h.begin(3)
        .run_part(1, |text| {
            let bytes = text.as_bytes();
            let line_len = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;

            let mut sum = 0;
            let mut pos = 0;
            while pos < bytes.len() {
                let mut num = 0;
                let mut num_start = 0;
                let mut last_was_digit = false;
                for i in pos..pos + line_len {
                    let b = bytes[i];
                    let digit = b.wrapping_sub(b'0');
                    let is_digit = digit < 10;
                    if is_digit {
                        num = (num * 10) + u32::from(digit);
                        if !last_was_digit {
                            num_start = i;
                        }
                    } else if last_was_digit {
                        let num_end = i;
                        let neighbours = iter::once(num_start.wrapping_sub(line_len + 1))
                            .chain((num_start..num_end).map(|j| j.wrapping_sub(line_len)))
                            .chain(iter::once(num_end.wrapping_sub(line_len)))
                            .chain(iter::once(num_start.wrapping_sub(1)))
                            .chain(iter::once(num_end))
                            .chain(iter::once(num_start + line_len - 1))
                            .chain((num_start..num_end).map(|j| j + line_len))
                            .chain(iter::once(num_end + line_len))
                            .map(|index| bytes.get(index).copied().unwrap_or(b'.'));
                        for neighbour in neighbours {
                            if neighbour != b'\n'
                                && neighbour != b'.'
                                && (neighbour.wrapping_sub(b'0') > 9)
                            {
                                sum += num;
                                break;
                            }
                        }
                        num = 0;
                    }
                    last_was_digit = is_digit;
                }

                pos += line_len;
            }

            sum
        })
        .run_part(2, |text| {
            let bytes = text.as_bytes();
            let line_len = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;

            let mut sum = 0;

            for i in 0..bytes.len() {
                if bytes[i] == b'*' {
                    let find_num = |mut index: usize| -> Option<(u32, usize)> {
                        // There is always a newline at the very end so this won't reject a number at the very end
                        if bytes.get(index)?.wrapping_sub(b'0') > 9 {
                            return None;
                        }
                        loop {
                            let new_index = match index.checked_sub(1) {
                                Some(n) => n,
                                None => break,
                            };
                            if bytes[new_index].wrapping_sub(b'0') > 9 {
                                break;
                            }
                            index = new_index;
                        }
                        let mut num = 0;
                        loop {
                            let d = bytes[index].wrapping_sub(b'0');
                            if d > 9 {
                                break Some((num, index));
                            }
                            num = (num * 10) + u32::from(d);
                            index += 1;
                        }
                    };
                    let mut num_end = 0;
                    let mut nums = [0; 2];
                    let mut num_idx = 0;
                    let mut next_idx = i.wrapping_sub(line_len + 1);
                    if let Some((num, end)) = find_num(next_idx) {
                        num_end = end;
                        nums[num_idx] = num;
                        num_idx += 1;
                    }
                    next_idx = i.wrapping_sub(line_len);
                    if num_end < next_idx {
                        if let Some((num, end)) = find_num(next_idx) {
                            num_end = end;
                            nums[num_idx] = num;
                            num_idx += 1;
                        }
                    }
                    next_idx = i.wrapping_sub(line_len - 1);
                    if num_end < next_idx {
                        if let Some((num, end)) = find_num(next_idx) {
                            if num_idx == 2 {
                                continue;
                            }
                            num_end = end;
                            nums[num_idx] = num;
                            num_idx += 1;
                        }
                    }
                    next_idx = i.wrapping_sub(1);
                    if let Some((num, end)) = find_num(next_idx) {
                        if num_idx == 2 {
                            continue;
                        }
                        num_end = end;
                        nums[num_idx] = num;
                        num_idx += 1;
                    }
                    next_idx = i + 1;
                    if let Some((num, end)) = find_num(next_idx) {
                        if num_idx == 2 {
                            continue;
                        }
                        num_end = end;
                        nums[num_idx] = num;
                        num_idx += 1;
                    }
                    next_idx = i + line_len - 1;
                    if let Some((num, end)) = find_num(next_idx) {
                        if num_idx == 2 {
                            continue;
                        }
                        num_end = end;
                        nums[num_idx] = num;
                        num_idx += 1;
                    }
                    next_idx = i + line_len;
                    if num_end < next_idx {
                        if let Some((num, end)) = find_num(next_idx) {
                            if num_idx == 2 {
                                continue;
                            }
                            num_end = end;
                            nums[num_idx] = num;
                            num_idx += 1;
                        }
                    }
                    next_idx = i + line_len + 1;
                    if num_end < next_idx {
                        if let Some((num, _)) = find_num(next_idx) {
                            if num_idx == 2 {
                                continue;
                            }
                            nums[num_idx] = num;
                            num_idx += 1;
                        }
                    }

                    if num_idx == 2 {
                        sum += nums[0] * nums[1];
                    }
                }
            }

            sum
        });
}
