# Performance Notes (cli_support)

## Hot paths
- None; crate is configuration/data only.

## Allocation patterns
- Minimal: allocates paths/strings when parsing CLI args or constructing opts.

## Trait objects
- None.

## Assumptions
- Overhead is negligible; performance driven by consumer binaries.
