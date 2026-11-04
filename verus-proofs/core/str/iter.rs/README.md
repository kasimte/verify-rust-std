# Challenge 22 — `Chars::advance_by`, unbounded, via Verus

## What this proves

`Chars::advance_by(n)` is the one Challenge 22 function with an input-dependent loop, so it is
the only one whose unbounded verification needs a loop *contract* rather than loop-free
reasoning. Kani's loop contracts cannot carry it at the current pin: the chunk loop drives a
`slice::Iter` whose internal pointer a havoc-based invariant cannot re-establish, and the
`.sum()` char-start count is a loop inside a library combinator with no contract attach site.

`advance_by.rs` discharges `advance_by`'s **safety obligation** — the computed byte offsets
passed to `self.iter.advance_by(k).unwrap_unchecked()` never exceed the remaining length — for
**arbitrary string length and arbitrary skip count**, with `decreases` giving termination (no
unwind bound) and loop invariants giving the bound. The pointer operations themselves are in the
`slice` module, which Challenge 22 lets a solution assume safe and correct (assumption 1), so
this is a value-level proof over the byte sequence.

Verified with Verus `0.2026.09.27` (`verification results:: 9 verified, 0 errors`); the
invariant is adequacy-checked (removing the offset clamp makes the proof fail, as expected).

## Scope and what remains

- This proof establishes the **value-level loop safety** that Kani cannot. It is a faithful
  model of `advance_by`'s offset logic, not yet attached to the shipped `library/core/src/str/
  iter.rs` body.
- Completing it to a body-attached proof needs (a) the Verus tool integration
  (model-checking/verify-rust-std#554, open, in progress) to define the `#[verus_spec]`
  attachment model, and (b) `vstd` specs for the `str`/`slice::Iter`/`as_chunks` calls in the
  real body. The invariant and `decreases` math here transfer unchanged; only the wrapper
  changes.
- Until #554 lands, this proof does not run in repository CI. It is included as the
  unbounded-`advance_by` component of a Kani + Verus combination solution (the other 14 functions
  are verified unbounded by Kani in `library/alloc/src/str.rs`).
