# Secure IoT Ledger

A public-good, blockchain-backed registry of IoT devices, firmware lineages, and security posture—with an open scoring model (CVE history, patch responsiveness, manufacturer reputation) and a built‑in bug‑bounty incentive layer for researchers.

---

## Vision
Create a globally verifiable "device birth certificate" for every IoT product line and regional firmware variant. Provide a living security score that reflects real-world risk and reward the ecosystem (researchers, vendors, integrators) for improving it.

---

## High-level Features
- **Global device registry**: Manufacturers, models, hardware chipsets, regions, and firmware versions.
- **Security score**: Computed from CVEs, CVSS severities, time-to-patch, update cadence, and certifications.
- **Responsible disclosure & bounties**: On-chain submissions, triage workflow, and token rewards.
- **Hybrid on/off-chain data**: On-chain identities, hashes & indexes; off-chain artifacts (firmware, SBOMs, reports) in IPFS/Arweave.
- **Standards-first**: W3C DID/VC for identities & attestations; SPDX/CycloneDX for SBOMs; OpenAPI/GraphQL APIs.
- **Interoperable**: Designed to anchor to/bridge with major L1s (Ethereum) while running a performant app-chain/rollup.

---

## Quick Start
```bash
# 1) Create repository
mkdir secure-iot-ledger && cd $_

# 2) Initialize
git init -b main

# 3) Bootstrap workspaces (Node + Rust recommended)
# requires: pnpm>=9, Node>=20, Rust toolchain, Foundry or Hardhat
pnpm -w init
pnpm -w add -D turbo typescript tsx eslint prettier

# 4) Bring in subpackages (see repo layout below)
# (You can copy/paste the scaffolded folders & package.json files)

# 5) First build
pnpm -w install
pnpm -w build
```

---

## Repository Layout
```
secure-iot-ledger/
├─ apps/
│  ├─ explorer/                # Web app: search devices, scores, vendors
│  ├─ console/                 # Admin & manufacturer portal
│  └─ api-gateway/             # Public REST/GraphQL over indexers
├─ chains/
│  ├─ contracts/               # Solidity/Foundry for L1 anchors & bounty
│  └─ appchain/                # Cosmos-SDK/Substrate rollup or Celestia rollup node
├─ services/
│  ├─ indexer/                 # Chain -> Postgres/Elastic, GraphQL
│  ├─ scoring-engine/          # CVE ingestion, score recompute jobs
│  ├─ cve-oracle/              # Pulls CVE/NVD feeds, vendor advisories
│  ├─ firmware-registry/       # Firmware/SBOM hashing & IPFS pinning
│  └─ attestation/             # DID/VC verifier & credential issuer
├─ packages/
│  ├─ proto/                   # gRPC/Protobuf schemas (internal RPC)
│  ├─ sdk-js/                  # JS/TS client SDK for apps & integrators
│  ├─ sdk-rs/                  # Rust SDK for gateways/backends
│  └─ schema/                  # OpenAPI/GraphQL schemas, JSON Schemas
├─ .github/
│  ├─ workflows/               # CI: build, test, lint, security scans
│  └─ ISSUE_TEMPLATE/          # bug_report.md, feature_request.md
├─ docs/                       # ADRs, specs, threat models, whitepaper
├─ SECURITY.md                 # vuln disclosure policy
├─ CONTRIBUTING.md             # how to contribute
├─ CODE_OF_CONDUCT.md          # community standards
├─ GOVERNANCE.md               # decision-making model
├─ LICENSE                     # Apache-2.0 (default)
└─ README.md                   # this file
```

---

## Architecture Snapshot
```
[Manufacturers] ──submit devices/firmware──▶ [App-chain / Rollup]
   │                                      ▲          │
   │  attestations (DID/VC)               │          │ anchors
   ▼                                      │          ▼
[Attestation svc] ──▶ issue VCs ──────────┘      [Ethereum L1]
   ▲
   │
[Researchers] ──submit vulns▶ [Bounty contracts] ─▶ payouts
   │                                │
   │  encrypted reports             ▼
   └──────────────▶ [Triage/Oracle/Scoring] ──▶ [Scores + Indexer + Explorer]
```

---

## Data Model (Core Objects)
**Manufacturer**
- `did` (DID URI), `name`, `website`, `pgp_pubkey`, `contact_security@`
- `reputationScore` (aggregate over portfolio, 0–100)

**DeviceModel**
- `deviceId` (NFT/DID), `manufacturerDid`
- `category` (camera, tv, hub, vehicle ECU, …)
- `chipset` (VID/PID, SoC), `region` (ISO), `releaseDate`
- `firmwareRefs[]` (array of FirmwareVersion IDs)
- `certifications[]` (VCs: PSA, CC, EU label …)
- `securityScore` (0–100), `scoreBreakdown{…}`

**FirmwareVersion**
- `fwId`, `deviceId`, `version`, `releaseDate`
- `hash` (SHA-256), `artifactUri` (ipfs://), `sbomUri` (ipfs:// SPDX/CycloneDX)
- `changelog`, `supersedes`

**Vulnerability**
- `vulnId` (CVE-YYYY-NNNN or Ledger-ID), `deviceId`, `fwRange`
- `reportedBy` (researcher DID), `severity` (CVSS v3/v4), `status` (Open/Triaged/Patched)
- `disclosure{reportedAt, vendorAckAt, publicAt}`
- `references[]` (urls, ipfs://report)

---

## Security Scoring (v0)
- **Base = 100**
- **Vuln penalty**: `Σ (w_sev * decay(age_days))` where `w_sev` = {Low:1, Med:3, High:7, Critical:12}; `decay(d)=1.0` for d≤90 then linearly →0.5 by 365d, →0.2 after.
- **Patch responsiveness**: bonus `+k` if `time_to_fix ≤ 30d`, partial if 31–90d, none if >90d.
- **Update cadence**: bonus for ≥ quarterly security releases.
- **Manufacturer reputation**: ± up to 10 based on portfolio average & variance.
- **Certifications**: +2 to +5 per verified certification (time-boxed).
- Clamp to [0,100]. Scores recomputed on relevant events.

---

## Smart Contracts (chains/contracts)
- `DeviceRegistry.sol` — Registers manufacturers, device models, firmware refs (hashes/URIs), emits events.
- `ScoreOracle.sol` — Accepts signed score updates from scoring-engine (with quorum/threshold, on-chain guardrails).
- `BountyHub.sol` — Program creation, submission escrow, triage state machine, payout with time-locked disclosure.
- `Certifications.sol` — Verifiable credential anchors (hashes), revocation registry.
- `RootAnchor.sol` — Periodic Merkle root of app-chain state anchored to L1.

> **Choice**: Foundry (forge + cast) with OpenZeppelin. Add EAS (Ethereum Attestation Service) optional for attestations.

---

## App-chain (chains/appchain)
- Cosmos SDK or Substrate-based rollup with modules:
  - `identity` (DID mapping), `devices`, `firmware`, `vulns`, `scores`, `certs`.
  - ABCI++ or rollup DA on Celestia; periodic L1 checkpoints.
- gRPC + Tendermint RPC exposed to indexer.

---

## Services
- **cve-oracle**: NVD, Mitre, vendor RSS; normalize → map to device models via chipset/strings; de-dup; sign feed.
- **scoring-engine**: recompute scores; submit signed updates on-chain; Kafka-in, Postgres-out.
- **firmware-registry**: ingest binaries/SBOMs, hash, store in IPFS/Arweave, verify reproducible builds (future).
- **indexer**: consumes chain events → Postgres/Elastic; provides GraphQL for apps.
- **attestation**: DID/VC verification, issuance workflows for cert bodies.

---

## APIs (packages/schema)
- **GraphQL** (Explorer):
```graphql
type Query {
  device(id: ID!): Device
  searchDevices(q: String!, region: String, vendor: ID): [Device]
  vulnerabilities(device: ID!, status: Status, since: Date): [Vuln]
}
```
- **REST (Gateway)**: `GET /v1/devices/:id`, `GET /v1/devices?chipset=…`, `GET /v1/vendors/:did/scores`, `POST /v1/vulns` (encrypted payload)
- **SDKs**: `sdk-js`, `sdk-rs` with typed clients & verification helpers.

---

## Local Dev (example)
```bash
# Contracts
cd chains/contracts
forge install
forge test

# App-chain (Cosmos SDK)
cd ../appchain
make start-devnet   # spins local validators + REST

# Services
pnpm -w run dev:indexer
pnpm -w run dev:scoring
pnpm -w run dev:gateway

# Web apps
pnpm -w run dev:explorer
```

---

## Security & Responsible Disclosure
See **SECURITY.md**. Coordinated disclosure with optional encrypted on-chain stub referencing an off-chain report. Default embargo: 90 days or until patch availability, whichever sooner.

---

## Roadmap (Milestones)
- **M0**: Repo scaffold, CI, coding standards, ADRs baseline.
- **M1**: DeviceRegistry + Explorer MVP (search vendor/model, view firmware, view score placeholder).
- **M2**: CVE Oracle + Scoring Engine → on-chain ScoreOracle integration.
- **M3**: BountyHub (minimal triage, payouts), manufacturer portal.
- **M4**: Certifications anchoring, DID/VC interop.
- **M5**: Rollup/App-chain public testnet + L1 anchoring.

---

## Contributing
See **CONTRIBUTING.md** for branching, commit style (Conventional Commits), code review, and security practices. All contributors must sign DCO.

---

## License
Apache-2.0. See **LICENSE**.

---

# Companion Files (copy into repo root)

## SECURITY.md
```md
# Security Policy

## Reporting a Vulnerability
Please email security@secure-iot-ledger.org and optionally open an **Encrypted Stub** issue (see template). We support PGP: `fingerprint TBD`.

- Prefer coordinated disclosure (default embargo 90 days or until patch release).
- Critical issues affecting active users may qualify for expedited bounties via on-chain BountyHub once validated.

## Scope
- Chains/contracts, services/*, apps/*, SDKs. Social engineering and third‑party hosting are out of scope unless explicitly stated.

## Bounties
See docs/bounties.md and on-chain programs in `chains/contracts/BountyHub.sol`.
```

## CONTRIBUTING.md
```md
# Contributing

## Dev Environment
- Node 20+, pnpm 9+, Rust stable, Foundry, Docker. `pnpm -w install` to bootstrap.

## Workflow
- Branch off `main` using `feat/*`, `fix/*`, `chore/*`.
- Conventional Commits. PRs require passing CI and at least one approval.

## Code Quality
- ESLint/Prettier for TS, Clippy for Rust, Solhint for Solidity. `pnpm -w test` must pass.

## Security
- Never commit secrets. Use `.env.example`. Enable 2FA on GitHub. Report vulns via SECURITY.md.
```

## CODE_OF_CONDUCT.md
```md
# Code of Conduct
We follow the Contributor Covenant v2.1.
```

## GOVERNANCE.md
```md
# Governance

- Maintainers group (odd number) stewards roadmap and releases.
- RFC/ADR process for significant changes (docs/adrs/NNNN-title.md).
- Security Response Team (SRT) handles embargoed issues.
```

## LICENSE (Apache-2.0)
```text
Apache License 2.0 — https://www.apache.org/licenses/LICENSE-2.0
```

## .github/ISSUE_TEMPLATE/bug_report.md
```md
---
name: Bug report
aabout: Report a problem to help us improve
title: "[bug] "
labels: bug
---
**Describe the bug**
**Steps to reproduce**
**Expected behavior**
**Screenshots/logs**
**Environment** (commit, OS)
**Additional context**
```

## .github/ISSUE_TEMPLATE/feature_request.md
```md
---
name: Feature request
about: Suggest an idea
labels: enhancement
---
**Problem Statement**
**Proposed Solution**
**Alternatives**
**Additional context**
```

## .github/workflows/ci.yml
```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:

jobs:
  build-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with: { version: 9 }
      - uses: actions/setup-node@v4
        with: { node-version: 20 }
      - name: Install
        run: pnpm -w install --frozen-lockfile
      - name: Lint & Test
        run: |
          pnpm -w lint || true
          pnpm -w test || true
```

## docs/adrs/0001-architecture.md
```md
# ADR 0001: Initial Architecture
- Status: Proposed
- Context: Need a scalable ledger with device registry, scoring, bounties.
- Decision: Hybrid app-chain with L1 anchoring; DID/VC; on/off-chain split.
- Consequences: Requires indexer and oracle services; enables interoperability.
```

