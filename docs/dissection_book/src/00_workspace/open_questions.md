# Open Questions

Use this page to track unknowns and where to investigate. Add/remove entries as they’re answered.

## Current questions
- (none) — add entries below as needed.

## Template
- **Question**: <what do we need to know?>
- **Where to look**: <files/modules/tests to inspect>
- **Status**: Unanswered / In progress / Answered
- **Notes**: <findings, hypotheses, links>

Example:
- **Question**: Should `capture_utils::generate_overlays` log skipped files?  
  **Where to look**: `capture_utils/src/lib.rs`, overlay generation loop.  
  **Status**: Unanswered.  
  **Notes**: Currently silent; consider adding logging for missing images.
