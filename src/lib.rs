use std::{cmp, ptr};

#[inline(always)]
unsafe fn u64(ptr: *const u8) -> u64 {
    ptr::read_unaligned(ptr as *const u64)
}

#[inline(always)]
unsafe fn u32(ptr: *const u8) -> u32 {
    ptr::read_unaligned(ptr as *const u32)
}

#[inline(always)]
unsafe fn u128(ptr: *const u8) -> u128 {
    ptr::read_unaligned(ptr as *const u128)
}

pub fn bytes_diff_naive(a: &[u8], b: &[u8]) -> usize {
    let alen = a.len();
    let blen = b.len();
    let shortest = if alen < blen { alen } else { blen };
    if a.as_ptr() == b.as_ptr() {
        return if alen == blen { blen } else { shortest };
    }

    let mut index: usize = 0;
    while index < shortest {
        if unsafe { a.get_unchecked(index) != b.get_unchecked(index) } {
            return index;
        }
        index += 1
    }

    return if alen == blen { blen } else { shortest };
}

/// https://github.com/tikv/agatedb/blob/master/src/util.rs#L22-L52
#[inline]
pub fn bytes_diff_original<'a>(base: &[u8], target: &'a [u8]) -> usize {
    let end = cmp::min(base.len(), target.len());
    let mut i = 0;
    unsafe {
        while i + 8 <= end {
            if u64(base.as_ptr().add(i)) != u64(target.as_ptr().add(i)) {
                break;
            }
            i += 8;
        }
        if i + 4 <= end && u32(base.as_ptr().add(i)) == u32(target.as_ptr().add(i)) {
            i += 4;
        }
        while i < end {
            if base.get_unchecked(i) != target.get_unchecked(i) {
                return i;
            }
            i += 1;
        }
        end
    }
}

#[inline]
pub fn bytes_diff_original_128<'a>(base: &[u8], target: &'a [u8]) -> usize {
    let end = cmp::min(base.len(), target.len());
    let mut i = 0;
    unsafe {
        while i + 16 <= end {
            if u128(base.as_ptr().add(i)) != u128(target.as_ptr().add(i)) {
                break;
            }
            i += 16;
        }
        if i + 8 <= end && u64(base.as_ptr().add(i)) == u64(target.as_ptr().add(i)) {
            i += 8;
        }
        if i + 4 <= end && u32(base.as_ptr().add(i)) == u32(target.as_ptr().add(i)) {
            i += 4;
        }
        while i < end {
            if base.get_unchecked(i) != target.get_unchecked(i) {
                return i;
            }
            i += 1;
        }
        end
    }
}

#[inline]
pub fn bytes_diff_chunked_original<'a>(base: &[u8], target: &'a [u8]) -> usize {
    let mut sum: usize = 0;
    for (l, r) in base.chunks_exact(128).zip(target.chunks_exact(128)) {
        let n = bytes_diff_original(l, r);
        sum += n;
        if n != 128 {
            break;
        }
    }
    unsafe {
        sum += bytes_diff_original(base.get_unchecked(sum..), target.get_unchecked(sum..));
    }

    sum
}

#[inline]
pub fn bytes_diff_hybrid_original<'a>(base: &[u8], target: &'a [u8]) -> usize {
    let end = cmp::min(base.len(), target.len());
    if end <= 128 {
        return bytes_diff_original(base, target);
    }
    bytes_diff_chunked_original(base, target)
}

#[inline]
pub fn bytes_diff_bitwise<'a>(base: &[u8], target: &'a [u8]) -> usize {
    let end = cmp::min(base.len(), target.len());
    let mut i: usize = 0;
    unsafe {
        while i + 8 <= end {
            let a = u64(base.as_ptr().add(i));
            let b = u64(target.as_ptr().add(i));
            let lz3 = ((a ^ b).leading_zeros() >> 3) as usize;

            i += lz3;
            if lz3 != 8 {
                // println!("lz3 is not 8: {}, i: {}", lz3, i);
                break;
            }
            // println!("lz3 is 8: {}, i: {}", lz3, i);
        }
    }
    while i < end {
        if unsafe { base.get_unchecked(i) != target.get_unchecked(i) } {
            return i;
        }
        i += 1;
    }
    end
}

#[inline]
pub fn bytes_diff_functional_naive<'a>(base: &[u8], target: &'a [u8]) -> usize {
    let mut end = 0;
    for (b, t) in base.iter().zip(target.iter()) {
        if b != t {
            return end;
        }
        end += 1;
    }
    end
}

#[inline]
pub fn bytes_diff_functional<'a>(base: &[u8], target: &'a [u8]) -> usize {
    target
        .iter()
        .zip(base.iter())
        .take_while(|(a, b)| a == b)
        .count()
}
#[inline]
pub fn bytes_diff_chunked8<'a>(base: &[u8], target: &'a [u8]) -> usize {
    bytes_diff_chunked::<8>(base, target)
}

#[inline]
pub fn bytes_diff_chunked64<'a>(base: &[u8], target: &'a [u8]) -> usize {
    bytes_diff_chunked::<64>(base, target)
}

#[inline]
pub fn bytes_diff_chunked32<'a>(base: &[u8], target: &'a [u8]) -> usize {
    bytes_diff_chunked::<32>(base, target)
}

#[inline]
pub fn bytes_diff_chunked16<'a>(base: &[u8], target: &'a [u8]) -> usize {
    bytes_diff_chunked::<16>(base, target)
}

#[inline]
pub fn bytes_diff_chunked128<'a>(base: &[u8], target: &'a [u8]) -> usize {
    bytes_diff_chunked::<128>(base, target)
}
#[inline]
fn bytes_diff_chunked<const N: usize>(xs: &[u8], ys: &[u8]) -> usize {
    let off = xs
        .chunks_exact(N)
        .zip(ys.chunks_exact(N))
        .take_while(|(x, y)| x == y)
        .count()
        * N;
    off + &xs[off..]
        .iter()
        .zip(&ys[off..])
        .take_while(|(x, y)| x == y)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_implementations_agree_simple() {
        let aaa: Vec<u8> = std::iter::repeat(10u8).take(4).collect();
        let bbb: Vec<u8> = std::iter::repeat(20u8).take(4).collect();
        let mut key1 = vec![];
        let mut key2 = vec![];
        key1.extend_from_slice(&aaa[..]);
        key1.extend_from_slice(&bbb[..]);
        key2.extend_from_slice(&aaa[1..]);
        key2.extend_from_slice(&bbb[1..]);
        let expected = 3;
        assert_eq!(bytes_diff_naive(&key1, &key2), expected);
        assert_eq!(bytes_diff_functional_naive(&key1, &key2), expected);
        assert_eq!(bytes_diff_original(&key1, &key2), expected);
        assert_eq!(bytes_diff_original_128(&key1, &key2), expected);
        assert_eq!(bytes_diff_chunked8(&key1, &key2), expected);
        assert_eq!(bytes_diff_chunked16(&key1, &key2), expected);
        assert_eq!(bytes_diff_chunked32(&key1, &key2), expected);
        assert_eq!(bytes_diff_chunked64(&key1, &key2), expected);
        assert_eq!(bytes_diff_chunked128(&key1, &key2), expected);
        assert_eq!(bytes_diff_bitwise(&key1, &key2), expected);
        assert_eq!(bytes_diff_chunked_original(&key1, &key2), expected);
    }

    #[test]
    fn learn_bitwise() {
        let a: usize = 0b0001;
        let b: usize = 0b0011;
        assert_eq!(a ^ a, 0b000);
        assert_eq!(a & b, 0b0001);
        let mut count = 0;
        // let mut mask = u8::MAX;
        let items = [0u8, 0, 0, 0, 10, 20];
        for item in items {
            let lz = item.leading_zeros() >> 3;
            count += lz;
        }

        assert_eq!(count, 4);
    }
}
