# Book Style Guide

## Chapters & structure
- Top-level: workspace overview, dependency graph, crate index, feature flags, build/dev workflow.
- Per-crate sections under `10_crates/<crate>`: overview, public API, lifecycle/data flow, module map, error model, performance notes, examples, design review.
- Cross-crate: canonical flows, integration contracts, docs.rs alignment, linking style, quality gates, changelog, reader’s guide.

## Naming conventions
- Paths: `10_crates/<crate>/0X_*.md` for crate pages; `00_workspace/*.md` for workspace-wide topics.
- Headings: Title Case; keep consistent order per crate.
- Templates: reuse `_templates/crate_template.md` when creating new crate pages.

## Cross-link style
- Use relative links: `../<crate>/01_overview.md` for intra-book references.
- Source links: `crate/path.rs:L123` format when pointing to GitHub code.
- Mention related crates/flows inline; avoid duplicate content by linking to existing sections.

## Auto-generated vs manual
- Auto-generated (preferred when data available): crate index, dependency graph, feature flags list, pub API tables, module maps, and canonical flow diagrams.
- Manual curation: lifecycle narratives, design reviews, performance notes, examples, error model nuances, integration contracts.

## Examples & code blocks
- Prefer small, focused snippets; mark non-runnable blocks with `ignore`/`no_run`.
- Include inputs/outputs when helpful; tie examples to public API.

## Diagrams
- Use Mermaid for dependency graphs and flows; place inline in relevant pages.
- Keep diagrams minimal and update when APIs/flows change.

## Quality
- Each page should answer: what is this, how to use/extend it, boundaries, failure modes.
- Keep prose concise; favor tables/diagrams over long text.
- Note NdArray default and feature-gated GPU paths where relevant.
