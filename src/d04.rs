use crate::harness::Harness;

use std::arch::x86_64::*;

pub fn run(h: &mut Harness) {
    h.begin(4)
        .run_part(1, |text| unsafe {
            let bytes = text.as_bytes();
            let (winning_off, winning_num_count, ours_off, ours_num_count, line_off) =
                calc_line_format(bytes);

            let remainder_mask = if ours_num_count == 25 {
                0x0FFF3F7F
            } else if ours_num_count == 8 {
                0x0000071F
            } else {
                // TODO
                todo!("calculate remainder mask")
            };

            let compare_shuffles = [
                make_compare_shuffle(winning_num_count, 0),
                make_compare_shuffle(winning_num_count, 4),
                make_compare_shuffle(winning_num_count, 8),
            ];
            let mut sum = 0;
            let mut i = 0;
            while i < bytes.len() {
                i += winning_off;
                // SAFETY: we can read past the end because of the way the harness allocates the input
                let raw_winning = _mm256_loadu_si256(bytes.as_ptr().add(i) as *const _);
                let winning = pack_digits(raw_winning);
                // We can shuffle 32 bit values cheaply, so we use 3 registers with 4 32 bit values
                // to be able to check against a maximum of 12 different values
                // There is a max of 10 winning numbers but because we repeat indices in the compare shuffle mask
                // the two unused slots will contain a duplicate value
                let compares = [
                    _mm_shuffle_epi8(winning, compare_shuffles[0]),
                    _mm_shuffle_epi8(winning, compare_shuffles[1]),
                    _mm_shuffle_epi8(winning, compare_shuffles[2]),
                ];

                i += ours_off;
                let mut remaining_nums = ours_num_count;
                while remaining_nums != 0 {
                    let remainder_mask = if remaining_nums >= 30 {
                        0xFFFF7F7F_u32 as i32
                    } else {
                        remainder_mask as i32
                    };
                    // SAFETY: see above
                    let raw_ours = _mm256_loadu_si256(bytes.as_ptr().add(i) as *const _);
                    // Stores 2x4 nums and 6 "lone" nums
                    let mut packed_low = pack_digits(raw_ours);
                    // Stores 4x4 nums
                    let mut packed_high = _mm_setzero_si128();
                    remaining_nums = remaining_nums.saturating_sub(10);
                    if remaining_nums != 0 {
                        // SAFETY: see above
                        let raw_ours = _mm256_loadu_si256(bytes.as_ptr().add(i + 30) as *const _);
                        let a = pack_digits(raw_ours);
                        packed_high = a;
                        let lone = _mm_or_si128(packed_low, _mm_slli_epi32::<8>(a));
                        packed_low = _mm_blend_epi32::<0b1010>(packed_low, lone);
                        remaining_nums = remaining_nums.saturating_sub(10);
                    }
                    if remaining_nums != 0 {
                        // SAFETY: see above
                        let raw_ours = _mm256_loadu_si256(bytes.as_ptr().add(i + 60) as *const _);
                        let b = pack_digits(raw_ours);
                        let lone = _mm_or_si128(packed_low, _mm_slli_epi32::<16>(b));
                        packed_low = _mm_blend_epi32::<0b1010>(packed_low, lone);
                        packed_high = _mm_blend_epi32::<0b1010>(
                            packed_high,
                            _mm_shuffle_epi32::<0b01_00_00_00>(b),
                        );
                        remaining_nums = remaining_nums.saturating_sub(10);
                    }
                    // 2 slots are unused, it would be incredibly clunky (slow) to fill them
                    let packed = _mm256_set_m128i(packed_high, packed_low);

                    let mut compare_mask = 0;
                    for i in 0..3 {
                        // This may look slow but the compiler is smart and moves it into registers once
                        let compare = compares[i];
                        let compare = _mm256_set_m128i(compare, compare);

                        let a = _mm256_cmpeq_epi8(packed, compare);
                        compare_mask |= _mm256_movemask_epi8(a);

                        let a = _mm256_cmpeq_epi8(
                            packed,
                            _mm256_shuffle_epi32::<0b00_11_10_01>(compare),
                        );
                        compare_mask |= _mm256_movemask_epi8(a);

                        let a = _mm256_cmpeq_epi8(
                            packed,
                            _mm256_shuffle_epi32::<0b01_00_11_10>(compare),
                        );
                        compare_mask |= _mm256_movemask_epi8(a);

                        let a = _mm256_cmpeq_epi8(
                            packed,
                            _mm256_shuffle_epi32::<0b10_01_00_11>(compare),
                        );
                        compare_mask |= _mm256_movemask_epi8(a);
                    }
                    sum += (1_u64 << (compare_mask & remainder_mask).count_ones()) >> 1;
                }

                i += line_off;
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

fn calc_line_format(bytes: &[u8]) -> (usize, u8, usize, u8, usize) {
    let winning_off = bytes.iter().position(|&b| b == b':').unwrap() + 1;
    let winning_len = bytes[winning_off..]
        .iter()
        .position(|&b| b == b'|')
        .unwrap()
        - 1;
    let winning_num_count = winning_len / 3;
    assert!(winning_num_count <= 10);

    let ours_off = winning_len + 2;
    let ours_len = bytes[winning_off + ours_off..]
        .iter()
        .position(|&b| b == b'\n')
        .unwrap()
        + 1;
    let ours_num_count = ours_len / 3;

    (
        winning_off,
        winning_num_count as u8,
        ours_off,
        ours_num_count as u8,
        ours_len,
    )
}

#[inline]
fn make_compare_shuffle(winning_num_count: u8, offset: u8) -> __m128i {
    unsafe {
        let mut buf = [0; 16];
        for i in 0..16 {
            let mut v = ((i % 4) + (i / 4) + offset) % winning_num_count;
            if v > 4 {
                v += 3;
            }
            buf[i as usize] = v;
        }
        _mm_loadu_si128(buf.as_ptr() as *const _)
    }
}

/// Pack 10 two-digit numbers as single bytes into slots 0..5 and 8..13
#[inline]
fn pack_digits(raw: __m256i) -> __m128i {
    unsafe {
        let shuffle = _mm256_set_epi64x(
            0x80_80_80_0C_09_06_03_00_u64 as i64,
            0x80_80_80_0D_0A_07_04_01_u64 as i64,
            0x80_80_80_0E_0B_08_05_02_u64 as i64,
            0x80_80_80_0D_0A_07_04_01_u64 as i64,
        );

        // Space and 0 conveniently both encode to 0 this way
        let digits = _mm256_and_si256(raw, _mm256_set1_epi8(0xF));
        let shuffle_digits = _mm256_shuffle_epi8(digits, shuffle);
        let a = _mm256_extracti128_si256::<0>(shuffle_digits);
        let b = _mm256_extracti128_si256::<1>(shuffle_digits);
        let high = _mm_blend_epi32::<0b1100>(a, b);
        let low = _mm_blend_epi32::<0b0011>(a, b);
        // We only care about equality so we can create a unique bit pattern as cheaply as possible
        // 8 bit shifts do not exist but it doesn't matter here since the digit values are only 4 bits
        _mm_or_si128(
            _mm_shuffle_epi32::<0b01_00_11_10>(low),
            _mm_slli_epi32::<4>(high),
        )
    }
}
