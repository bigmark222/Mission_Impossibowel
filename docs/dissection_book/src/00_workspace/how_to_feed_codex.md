# How to Feed Codex (Paste Protocol)

When asking Codex to update this book or analyze the workspace, paste inputs in this order for best results. Keep snippets tight; omit noise.

## Workspace-level context
1) **Root Cargo.toml** (workspace members, features, patches).  
2) **`cargo metadata --format-version 1` JSON** (or `cargo metadata` output).  
3) **Crate list**: output of `tools/extract_for_book.sh` (or at least the crate list section).

## Per-crate context (repeat for each crate you want analyzed)
1) `Cargo.toml` for that crate (features, deps, publish settings).  
2) Entrypoint source: `src/lib.rs` or `src/main.rs`.  
3) Module tree: `find src -type f` or the module section from `tools/extract_for_book.sh`.  
4) Public items: `rg "^pub " -n src` (copy for that crate).  
5) Any relevant module code for deep dives (paste file-by-file as needed).

## Flow/architecture tasks
- Dependency graph: paste the Mermaid or summary from `docs/dissection_book/src/00_workspace/dependency_graph.md` or `cargo metadata` if recomputing.
- Entry points: note binaries or Bevy plugins involved (paths to bins/plugins).

## Publishing/versioning tasks
- Latest tags/versions, lockfile diffs if relevant.
- Notes about patches (e.g., burn-core pin) or `[patch]` sections.

## General tips
- Use repo-relative paths and line numbers when referencing code (see `linking_style.md`).
- Trim large outputs; if something is huge, summarize or paste the relevant slice.
- Call out active feature flags or environment expectations that affect behavior.
