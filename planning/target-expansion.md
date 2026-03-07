# Target Expansion Track (Post-v0.1)

## Purpose

Expand compatibility output coverage after `v0.1.0` without destabilizing the canonical policy model.

This track retains useful ideas from prior exploration:

- treat outputs as explicit targets
- keep policy semantics separate from tool-specific compatibility artifacts
- support CLI target visibility
- defer profile complexity until justified by real usage

---

## Constraints

1. Do not break existing `v0.1` schema contracts for core policy semantics.
2. Keep generation deterministic and checkable with `agent-policy check`.
3. Prefer additive changes over restructures.
4. Do not default-generate user-local memory/settings artifacts.

---

## Milestone A — Immediate Post-v0.1 Hardening

**Goal:** Expose target visibility and lay clean plumbing for future target additions.

### Planned additions

- Add `agent-policy list-targets` command that prints each supported target ID, its output path, and its stability tier.
- Introduce an internal `TargetId` enum (`AgentsMd`, `ClaudeMd`, `CursorRules`) to replace inline string literals in renderer dispatch. **This is a code-quality refactor, not a schema or behavior change.** The `outputs` array already uses string IDs from v0.1; the enum simply mirrors them in Rust for type-safety and easier matching.
- Add docs alignment for target tiers and matrix.

### Note

The `outputs: [target-id, ...]` array schema is already final from v0.1. Adding new targets in Milestone B requires only: (1) a new `enum` value in the JSON Schema `items`, (2) a new `TargetId` variant, (3) a new renderer module and template. No structural migration needed.

### Exit condition

- `list-targets` prints all supported targets with their output path and stability tier.
- Existing generation and check behavior is unchanged.

---

## Milestone B — First-Wave Target Expansion

**Goal:** Add highest-value repo-committable targets with low schema risk.

### Candidate targets (priority order)

1. `GEMINI.md`
2. `.github/copilot-instructions.md`
3. `.github/instructions/*.instructions.md`
4. `.junie/guidelines.md`

### Exit condition

- Each new target has:
  - renderer implementation
  - golden tests
  - check-mode drift coverage
  - README/docs examples

---

## Milestone C — Secondary Rule Ecosystem Targets

**Goal:** Add additional rule directory ecosystems after wave-one targets settle.

### Candidate targets

1. `.windsurf/rules/*.md`
2. `.clinerules/*.{md,txt}`
3. `.continue/rules/*.md`

### Exit condition

- At least two secondary ecosystems supported with stable tests and docs.
- No regressions in core output generation ergonomics.

---

## Milestone D — Evaluate Profiles and Target Overrides

**Goal:** Decide if profile-based generation is justified by real user pressure.

### Decision gate

Introduce `profiles` and/or `--targets` only if all are true:

1. Target count and user workflows exceed boolean output ergonomics.
2. There is repeated demand for reusable target bundles.
3. Migration from current `outputs.*` can be done safely.

### Preferred order

1. Add `--targets` override first (explicit and transparent).
2. Add config-level `profiles` later if needed.

---

## Milestone E — Data-Driven Registry and Template Extensibility

**Goal:** Consider registry-driven target metadata only after target model stabilizes.

This aligns with later ecosystem goals and should not precede stable target semantics.

---

## Non-Goals for This Track

- Replacing canonical policy with tool-specific schemas.
- Introducing plugin systems before target model maturity.
- Generating local-only user memory/config by default.

---

## Related Docs

- [../docs/target-support-policy.md](../docs/target-support-policy.md)
- [../docs/compatibility-matrix.md](../docs/compatibility-matrix.md)
- [roadmap.md](roadmap.md)
