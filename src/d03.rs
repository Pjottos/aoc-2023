use crate::harness::Harness;

use std::arch::x86_64::*;

pub fn run(h: &mut Harness) {
    h.begin(3)
        /////        .input_override(
        /////            "467..114..
        /////...*......
        /////..35..633.
        /////......#...
        /////617*......
        /////.....+.58.
        /////..592.....
        /////......755.
        /////...$.*....
        /////.664.598..
        /////",
        /////        )
        .run_part(1, |text| unsafe {
            let bytes = text.as_bytes();
            let line_len = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;
            let line_count = bytes.len() / line_len;
            let chunk_count = (line_len - 1) / 32;
            let chunk_remainder = (line_len - 1) % 32;
            assert!(line_len < 256, "lines too long");

            // We can wrap because the mask won't be used if the remainder is 0
            let remainder_mask = u32::MAX.wrapping_shr(32 - chunk_remainder as u32);

            let mut row_symbol_neighbours = [
                _mm256_setzero_si256(),
                _mm256_setzero_si256(),
                _mm256_setzero_si256(),
            ];
            let mut row_digits = [
                _mm256_setzero_si256(),
                _mm256_setzero_si256(),
                _mm256_setzero_si256(),
            ];
            // We set a maximum of 3 digit numbers
            let mut sum_parts = [0; 3];
            for l in 0..line_count + 1 {
                let pipeline_tail = l % 3;
                let pipeline_head = (l + 1) % 3;
                let pipeline_cur = (l + 2) % 3;
                if l < line_count {
                    for i in 0..chunk_count + (chunk_remainder != 0) as usize {
                        // For the remainder we could be reading past the end of the input, but that's OK because the harness
                        // allocates padding at the end so that we can read 63 bytes past the end.
                        let cur = _mm256_loadu_si256(
                            bytes.as_ptr().add((l * line_len) + (i * 32)) as *const _
                        );
                        let mask = if i == chunk_count {
                            remainder_mask
                        } else {
                            u32::MAX
                        };
                        let digit_mask = _mm256_and_si256(
                            _mm256_cmpgt_epi8(cur, _mm256_set1_epi8(b'0' as i8 - 1)),
                            _mm256_cmpgt_epi8(_mm256_set1_epi8(b'9' as i8 + 1), cur),
                        );
                        let digit_bits = _mm256_movemask_epi8(digit_mask) as u32 & mask;
                        let symbol_mask = _mm256_or_si256(
                            _mm256_andnot_si256(
                                _mm256_cmpeq_epi8(cur, _mm256_set1_epi8(b'.' as i8)),
                                _mm256_cmpgt_epi8(_mm256_set1_epi8(b'/' as i8 + 1), cur),
                            ),
                            _mm256_and_si256(
                                _mm256_cmpgt_epi8(cur, _mm256_set1_epi8(b':' as i8)),
                                _mm256_cmpgt_epi8(_mm256_set1_epi8(b'@' as i8 + 1), cur),
                            ),
                        );
                        let symbol_bits = _mm256_movemask_epi8(symbol_mask) as u32 & mask;

                        // SAFETY: we checked the line length and alignment is higher for __m256i than for u32
                        (&mut row_digits[pipeline_tail] as *mut _ as *mut u32)
                            .add(i)
                            .write(digit_bits);
                        // We write the symbol bits first, then convert it into neighbours in one go later
                        (&mut row_symbol_neighbours[pipeline_tail] as *mut _ as *mut u32)
                            .add(i)
                            .write(symbol_bits);
                    }

                    // Convert to neighbour mask
                    let symbol_bits = row_symbol_neighbours[pipeline_tail];
                    let symbol_neighbours =
                        _mm256_or_si256(expand_frontier(symbol_bits), symbol_bits);
                    row_symbol_neighbours[pipeline_tail] = symbol_neighbours;
                } else {
                    row_digits[pipeline_tail] = _mm256_setzero_si256();
                    row_symbol_neighbours[pipeline_tail] = _mm256_setzero_si256();
                }
                if l != 0 {
                    let top_symbol_neighbours = row_symbol_neighbours[pipeline_head];
                    let cur_symbol_neighbours = row_symbol_neighbours[pipeline_cur];
                    let bottom_symbol_neighbours = row_symbol_neighbours[pipeline_tail];
                    let cur_row_digits = row_digits[pipeline_cur];
                    let mut neighbour_digits = _mm256_or_si256(
                        _mm256_or_si256(
                            _mm256_and_si256(top_symbol_neighbours, cur_row_digits),
                            _mm256_and_si256(cur_symbol_neighbours, cur_row_digits),
                        ),
                        _mm256_and_si256(bottom_symbol_neighbours, cur_row_digits),
                    );
                    // We do a sort of flood fill to make the neighbour matches fill the entire number
                    // At least one digit is already marked, we expand 2 times in both directions to ensure all 3 digit
                    // numbers are filled
                    for _ in 0..2 {
                        neighbour_digits = _mm256_and_si256(
                            _mm256_or_si256(expand_frontier(neighbour_digits), neighbour_digits),
                            cur_row_digits,
                        );
                    }
                    //println!(
                    //    "{:012b}{:064b}{:064b} digits",
                    //    _mm256_extract_epi64::<2>(cur_row_digits),
                    //    _mm256_extract_epi64::<1>(cur_row_digits),
                    //    _mm256_extract_epi64::<0>(cur_row_digits),
                    //);

                    #[inline]
                    fn process_bits(
                        mut bits: u64,
                        next_bits: &mut u64,
                        bytes: &[u8],
                        sum_parts: &mut [u32; 3],
                    ) {
                        if bits != 0 {
                            let mut zeros = bits.leading_zeros();
                            bits <<= zeros;
                            let mut shift = zeros;
                            let mut end = 63 - zeros;
                            let mut num_len = bits.leading_ones();
                            bits <<= num_len;
                            shift += num_len;
                            // We need to handle numbers crossing boundaries
                            // Adjust the offset past the current chunk and clear the bits in the next chunk mask so we don't
                            // read the number again
                            if zeros == 0 {
                                let extra = next_bits.trailing_ones();
                                *next_bits &= !((1 << extra) - 1);
                                end += extra;
                                num_len += extra;
                            }
                            loop {
                                for i in 0..num_len as usize {
                                    let digit = u32::from(bytes[end as usize - i] - b'0');
                                    sum_parts[i] += digit;
                                }

                                if bits == 0 {
                                    break;
                                }
                                zeros = bits.leading_zeros();
                                bits <<= zeros;
                                shift += zeros;
                                end = 63 - shift;
                                num_len = bits.leading_ones();
                                bits <<= num_len;
                                shift += num_len;
                            }
                        }
                    }

                    let cur_line_offset = (l - 1) * line_len;
                    let line_bytes = &bytes[cur_line_offset..];
                    let mut bits = _mm256_extract_epi64::<0>(neighbour_digits) as u64;
                    let mut next_bits = _mm256_extract_epi64::<1>(neighbour_digits) as u64;
                    //println!("{bits:064b}");
                    process_bits(bits, &mut next_bits, &line_bytes[0 * 64..], &mut sum_parts);
                    if line_len >= 64 {
                        bits = next_bits;
                        next_bits = _mm256_extract_epi64::<2>(neighbour_digits) as u64;
                        process_bits(bits, &mut next_bits, &line_bytes[1 * 64..], &mut sum_parts);
                    }
                    if line_len >= 128 {
                        bits = next_bits;
                        next_bits = _mm256_extract_epi64::<3>(neighbour_digits) as u64;
                        process_bits(bits, &mut next_bits, &line_bytes[2 * 64..], &mut sum_parts);
                    }
                    if line_len >= 192 {
                        bits = next_bits;
                        next_bits = 0;
                        process_bits(bits, &mut next_bits, &line_bytes[3 * 64..], &mut sum_parts);
                    }
                }
            }

            sum_parts[0] + (sum_parts[1] * 10) + (sum_parts[2] * 100)
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

#[inline]
fn expand_frontier(bits: __m256i) -> __m256i {
    unsafe {
        // Because we operate on 64 bit lanes we need to insert 6 bits on the borders
        let carry_bits_lsb = _mm256_slli_epi64::<63>(bits);
        let carry_bits_msb = _mm256_srli_epi64::<63>(bits);
        let carry_bits = _mm256_or_si256(
            _mm256_permute4x64_epi64::<0b11111001>(carry_bits_lsb),
            _mm256_permute4x64_epi64::<0b10010000>(carry_bits_msb),
        );
        let carry_mask = _mm256_setr_epi64x(-2, -1, -1, 0x7FFFFFFFFFFFFFFF);
        _mm256_or_si256(
            _mm256_or_si256(_mm256_slli_epi64::<1>(bits), _mm256_srli_epi64::<1>(bits)),
            _mm256_and_si256(carry_bits, carry_mask),
        )
    }
}
