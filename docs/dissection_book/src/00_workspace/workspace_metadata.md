# Workspace Metadata

## Workspace metadata (root [Cargo.toml](https://github.com/via-balaena/CortenForge/blob/main/Cargo.toml))
| Item | Details |
| --- | --- |
| resolver | `2` |
| Patch overrides | <ul><li>local paths for all cortenforge crates (workspace dev convenience).</li></ul> |
| Publish status | <ul><li>most crates publishable</li><li>`colon_sim_tools` marked `publish = false`.</li></ul> |

## Notes
- resolver = 2 
    - opts into Cargo’s 2021+ feature resolver, 
        - which prevents unwanted feature unification across the workspace.

            - Under the old resolver, if one crate enabled a feature on a shared dependency, every other crate using that dependency inherited it automatically. That often resulted in unexpected behavior, larger binaries, and optional dependencies being pulled in without intent. With resolver 2, each crate resolves features independently, so only the features a crate explicitly opts into are activated.

- Burn-core is fixed in 0.14.1; no vendored patch is required.

- `colon_sim_tools` stays unpublished because it includes app-specific bins; plan is to split shared helpers from app-facing pieces.
