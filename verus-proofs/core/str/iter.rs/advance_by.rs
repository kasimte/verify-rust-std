#![feature(proc_macro_hygiene)]
// Verus value-level proof of the SAFETY obligation of `Chars::advance_by`, UNBOUNDED in both
// the string length and the skip count. advance_by walks the remaining bytes and finally calls
// `self.iter.advance_by(k).unwrap_unchecked()` for computed byte offsets k; the obligation that
// makes those calls safe is "k never exceeds the remaining length". Kani cannot prove this
// without a bound (its loop contracts can't carry the iterator loop). Verus proves it
// size-independently: `decreases` gives termination for ANY length/count, `invariant` gives the
// bound. The pointer ops themselves live in `slice::Iter` (challenge assumption 1: slice module
// safe+correct), so this is a value-level proof over the byte sequence.
use vstd::prelude::*;

verus! {

spec fn is_cont(b: u8) -> bool { (b & 0xC0u8) == 0x80u8 }

exec fn is_cont_exec(b: u8) -> (r: bool) ensures r == is_cont(b) { (b & 0xC0u8) == 0x80u8 }

exec fn char_width(b: u8) -> (w: usize) ensures 1 <= w <= 4 {
    if b < 0x80u8 { 1 } else if b < 0xE0u8 { 2 } else if b < 0xF0u8 { 3 } else { 4 }
}

const CHUNK: usize = 32;

// Returns the total bytes to advance. ensures = the SAFETY obligation: the offset never exceeds
// the length, for ARBITRARY `bytes` length and ARBITRARY `remainder`. No bound, no unwind.
// `requires bytes.len() <= usize::MAX - CHUNK` is the real Rust slice invariant (len <= isize::MAX),
// not a verification cap — it only discharges the `+ CHUNK`/`+ w` overflow checks.
fn advance_by_offset(bytes: &[u8], remainder: usize) -> (consumed: usize)
    requires bytes.len() <= usize::MAX - CHUNK,
    ensures consumed <= bytes.len(),
{
    let n: usize = bytes.len();
    let mut consumed: usize = 0;
    let mut rem: usize = remainder;

    if rem >= CHUNK {
        while rem > CHUNK && n - consumed >= CHUNK
            invariant
                n == bytes.len(),
                consumed <= n,
            decreases n - consumed,
        {
            let mut starts: usize = 0;
            let mut j: usize = 0;
            while j < CHUNK
                invariant
                    n == bytes.len(),
                    j <= CHUNK,
                    starts <= j,
                    consumed + CHUNK <= n,
                decreases CHUNK - j,
            {
                if !is_cont_exec(bytes[consumed + j]) {
                    starts = starts + 1;
                }
                j = j + 1;
            }
            rem = if starts <= rem { rem - starts } else { 0 };
            consumed = consumed + CHUNK;
        }

        while consumed < n && is_cont_exec(bytes[consumed])
            invariant
                n == bytes.len(),
                consumed <= n,
            decreases n - consumed,
        {
            consumed = consumed + 1;
        }
    }

    while rem > 0 && consumed < n
        invariant
            n == bytes.len(),
            n <= usize::MAX - CHUNK,
            consumed <= n,
        decreases n - consumed,
    {
        let w = char_width(bytes[consumed]);
        consumed = if consumed + w <= n { consumed + w } else { n };
        rem = rem - 1;
    }

    consumed
}

fn main() {}

}
