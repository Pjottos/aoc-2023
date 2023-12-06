use crate::harness::Harness;

use std::arch::x86_64::*;

pub fn run(h: &mut Harness) {
    h.begin(1)
        .run_part(1, |text| unsafe { run_one(text) })
        .run_part(2, |text| {
            let cheese = text
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            unsafe { run_one(&cheese) }
        });
}

unsafe fn run_one(text: &str) -> u32 {
    // 8 bit sums, needs to be flushed every 28 chunks to prevent overflow
    let mut sum_low = _mm256_setzero_si256();
    let mut sum_high = _mm256_setzero_si256();
    let mut final_sum_low = 0;
    let mut final_sum_high = 0;
    let mut last_digit_idx = 0;
    let mut need_high_digit = false;
    let bytes = text.as_bytes();
    let chunks = bytes.array_chunks::<32>();

    // We need to kickstart with scalar code to find the first digit in the first line
    for (i, &b) in bytes.iter().enumerate() {
        assert!(b != b'\n');
        let digit = b.wrapping_sub(b'0');
        if digit <= 9 {
            final_sum_high += u32::from(digit);
            last_digit_idx = i;
            break;
        }
    }
    // Make sure we found the first digit
    assert!(final_sum_high != 0);

    for (i, chunk) in chunks.clone().enumerate() {
        // SAFETY: in bounds
        let raw = _mm256_loadu_si256(chunk.as_ptr() as *const _);
        let is_newline = _mm256_cmpeq_epi8(raw, _mm256_set1_epi8(b'\n' as i8));
        let is_newline_mask = _mm256_movemask_epi8(is_newline) as u32;
        let new_digit = _mm256_sub_epi8(raw, _mm256_set1_epi8(b'0' as i8));

        // AVX2 doesn't have unsigned compare so unfortunately we have to make 2 comparisons
        let is_valid = _mm256_and_si256(
            _mm256_cmpgt_epi8(new_digit, _mm256_set1_epi8(-1)),
            _mm256_cmpgt_epi8(_mm256_set1_epi8(10), new_digit),
        );
        let is_valid_mask = _mm256_movemask_epi8(is_valid) as u32;

        // Calculate the mask for low and high sums
        // We are silently ignoring lines that don't have any digits as they do not have a number to contribute to the
        // final sum. It be desirable to error out if that happens
        let mut newline_bits = is_newline_mask;
        let digit_bits = is_valid_mask;
        if need_high_digit {
            let next_idx = digit_bits.trailing_zeros();
            if next_idx != 32 {
                final_sum_high += u32::from(bytes[(i * 32) + next_idx as usize] - b'0');
                need_high_digit = false;
            }
        }
        let mut low_mask = 0;
        let mut high_mask = 0;
        while newline_bits != 0 {
            //println!("d: {digit_bits:032b}");
            //println!("n: {is_newline_mask:032b}");
            let newline_idx = newline_bits.trailing_zeros();
            // Bits in this mask are set from the lsb up to the position of the first newline character
            // Cannot overflow because we checked there is at least one newline bit
            let newline_bit = 1_u32 << newline_idx;
            let line_mask = newline_bit - 1;
            // We need to clear the bit to keep using leading/trailing zero counts
            newline_bits &= !newline_bit;
            //println!("{line_mask:032b}");
            // Find the last digit in a line, closest to the newline on the right
            let low_digit = 0x80000000_u32
                .checked_shr((digit_bits & line_mask).leading_zeros())
                .unwrap_or_else(|| {
                    // The digit is not in this chunk, luckily we jotted down the index when we were processing the
                    // chunk that did have it.
                    // We use get_unchecked because the potential side effect from panicking on the bounds check
                    // causes this block to compile as a branch.
                    final_sum_low += u32::from(*bytes.get_unchecked(last_digit_idx) - b'0');
                    //println!("using {last_digit_idx} for last digit");
                    0
                });
            low_mask |= low_digit;
            //println!("l: {low_digit:032b}");
            // Find the first digit in the next line
            let high_digit = 1_u32
                .checked_shl((digit_bits & !line_mask).trailing_zeros())
                .unwrap_or_else(|| {
                    // Set a flag to account for the high digit when we find it in a future chunk
                    need_high_digit = true;
                    0
                });
            high_mask |= high_digit;
            //println!("h: {high_digit:032b}");
            //println!();
        }
        // In the best case each chunk has one newline and a digit at both sides of the newline but because the lines are
        // variably sized we need to be able to refer back to the digits at the outsides of chunks in case the newline
        // is not in the same chunk as either digit
        let last_idx = digit_bits.leading_zeros();
        if last_idx != 32 {
            last_digit_idx = (i * 32) + (31 - last_idx as usize);
        }

        //println!("{low_mask:032b}");
        //println!("{high_mask:032b}");
        let low = widen_mask(low_mask);
        sum_low = _mm256_add_epi8(sum_low, _mm256_and_si256(new_digit, low));
        let high = widen_mask(high_mask);
        sum_high = _mm256_add_epi8(sum_high, _mm256_and_si256(new_digit, high));

        // Theoretically, the worst case value of 9, 29 times (261) would overflow a 8 bit simd sum element
        // so we accumulate into a larger sum to be safe
        if i % 28 == 27 {
            final_sum_high += u32::from(horizontal_sum(sum_high));
            final_sum_low += u32::from(horizontal_sum(sum_low));
            sum_high = _mm256_setzero_si256();
            sum_low = _mm256_setzero_si256();
        }
    }

    final_sum_high += u32::from(horizontal_sum(sum_high));
    final_sum_low += u32::from(horizontal_sum(sum_low));

    let mut digit = 0;
    for &b in chunks.remainder() {
        let is_newline = b == b'\n';
        let new_digit = b.wrapping_sub(b'0');
        let is_valid = new_digit <= 9;
        if is_valid {
            digit = new_digit;
        }
        if need_high_digit && is_valid {
            final_sum_high += u32::from(new_digit);
            need_high_digit = false;
        }
        if is_newline {
            final_sum_low += u32::from(digit);
            need_high_digit = true;
        }
    }

    final_sum_low + (final_sum_high * 10)
}

fn horizontal_sum(bytes: __m256i) -> u16 {
    unsafe {
        let lo = _mm256_unpacklo_epi8(bytes, _mm256_setzero_si256());
        let hi = _mm256_unpackhi_epi8(bytes, _mm256_setzero_si256());
        let sum1 = _mm256_add_epi16(lo, hi);
        let sum2 = _mm_add_epi16(
            _mm256_extracti128_si256::<0>(sum1),
            _mm256_extracti128_si256::<1>(sum1),
        );
        let sum3 = _mm_add_epi16(sum2, _mm_shuffle_epi32::<0b00001110>(sum2));
        let sum4 = _mm_add_epi16(sum3, _mm_shuffle_epi32::<0b00000001>(sum3));
        let sum5 = _mm_add_epi16(sum4, _mm_shufflelo_epi16::<0b00000001>(sum4));
        _mm_extract_epi16::<0>(sum5) as u16
    }
}

#[inline]
fn widen_mask(mask: u32) -> __m256i {
    unsafe {
        let vmask1 = _mm256_set1_epi32(mask as i32);
        let shuffle = _mm256_setr_epi64x(
            0x0000000000000000,
            0x0101010101010101,
            0x0202020202020202,
            0x0303030303030303,
        );
        let vmask2 = _mm256_shuffle_epi8(vmask1, shuffle);
        let bit_mask = _mm256_set1_epi64x(0x7F_BF_DF_EF_F7_FB_FD_FE);
        let vmask3 = _mm256_or_si256(vmask2, bit_mask);
        let vmask4 = _mm256_cmpeq_epi8(vmask3, _mm256_set1_epi8(-1));
        vmask4
    }
}
