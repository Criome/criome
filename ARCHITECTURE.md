# ARCHITECTURE — criome

The runtime pillar — the umbrella name for what runs on a node
(sema + lojix). This repo is the **namespace spec**.

```
criome (runtime)
   ├── sema   — records, meaning, schema, patterns
   └── lojix  — artifacts, build, compile, store, deploy
   └── (skin) — nexus: text request language spanning both
```

The `criome` name appears in:

- **`criomed`** — sema's engine daemon (CANON-MISSING; the
  authoritative writer to sema, the validator gate).
- **`criome-net`** — peer-to-peer protocol between criomed
  instances (post-MVP sketch).

This repo holds the spec; concrete daemons and protocols live
in their own repos.

## What this repo defines

- The runtime taxonomy: criome ⊇ {sema, lojix}; nexus is the
  communication skin spanning both.
- The two-axis framing: every daemon has both a **runtime
  identity** (criome) and a **family lineage** (sema-family or
  lojix-family).

## What this repo does not define

- The criomed binary — CANON-MISSING; lands when Stage A
  scaffolds.
- The validator pipeline — that's criomed's internals.
- Sema records — those are in
  [nexus-schema](https://github.com/LiGoldragon/nexus-schema).

## Status

**Namespace spec.** Stable. Operational specs land in their
respective repos when scaffolded.

## Cross-cutting context

- Three-pillar framing:
  [mentci-next/docs/architecture.md §1 + §4 + §8](https://github.com/LiGoldragon/mentci-next/blob/main/docs/architecture.md)
