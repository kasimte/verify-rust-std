#!/bin/sh
# Verify the advance_by safety proof with the pinned Verus release bundle.
# Verus is pending repository integration (model-checking/verify-rust-std#554); once the
# verus.yml workflow lands this runs in CI. Local: point VERUS at the release binary.
: "${VERUS:?set VERUS to the verus binary from the release bundle}"
"$VERUS" advance_by.rs
