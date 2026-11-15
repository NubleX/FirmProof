# FirmProof: multichain design (2026-11-15)

Status: draft for Igor's review. Builds on `ARCHITECTURE.md`, `docs/CHARTER.md`, ADR 0001 and `Research.txt`; where
this spec differs from them, it says so and why.

## 1. What FirmProof is

A public ledger of the world's connected devices, ranked by their security and firmware flaws:
- **Every device model and firmware version** gets a "birth certificate": vendor, model, chipset, region, firmware
  hash, SBOM.
- **A security score** computed from known vulnerabilities, patch speed, update cadence, vendor history and
  certifications, with a published formula anyone can recompute.
- **Independent, international audits** recorded as signed attestations that cannot be edited afterwards.
- **Bug bounties paid in AURA** (and USDC), with coordinated disclosure built in.

It is an ID ArtCraft product: the security brand is the moat, and the ledger is the proof.

## 2. Why now

- **EU Cyber Resilience Act:** manufacturers of products with digital elements must report actively exploited
  vulnerabilities from September 2026, with the full obligations from December 2027. Vendors will need evidence of
  their security handling, and buyers will want to compare them. FirmProof is that evidence, pan-European by default.
- **US Cyber Trust Mark** and similar labels create demand for an independent, public comparison.
- **No single source** covers IoT vulnerabilities per device and firmware today (`Research.txt`, overview).

(Regulatory dates are from memory and must be checked against the official texts before they go into any pitch.)

## 3. Positioning: "a parachain of everything"

AURA and FirmProof belong to no single chain. In practice:
- **Present on many chains early**, for exposure: AURA is native on Solana and on the main EVM chains, and audits can
  be verified on whichever chain a user is on.
- **One source of truth:** the registry and scores live in FirmProof's own store, and every chain sees the same
  attestations and the same token supply.
- **The own chain comes later and is a literal parachain.** The existing Substrate scaffold
  (`chains/appchain/substrate`, pallets devices, firmware, vulns, scoring, bounties, certifications, anchors) is the
  end state. It joins Polkadot as a parachain when traffic justifies the cost of running it, and bridges out to
  Ethereum and Solana. Until then, the same logic runs as contracts and attestations on existing chains.

This changes ADR 0001 in timing only: the app-chain stays the target, but it is phase 4, not phase 1, so the product
and its audits can go live without first running validators and a bridge.

## 4. Data model (unchanged from ARCHITECTURE.md, with these additions)

- **Device identity key:** `vendorDid + model + region + firmwareHash (SHA-256)`. A firmware version is identified by
  its hash, never by its version string (vendors reuse version strings across regions).
- **Audit attestation** (new core object):
  - `auditorDid`, `firmwareHash`, `scope` (hardware, firmware, cloud, app), `method` (standard or methodology id,
    e.g. ETSI EN 303 645, OWASP ISVS), `findingsSummary` (counts by severity), `reportHash`, `reportUri` (IPFS,
    encrypted until disclosure), `date`, `signature`.
  - One schema, published on each chain's attestation service (section 6).
- **Auditor:** `did`, `country`, `accreditations[]` (verifiable credentials from accreditation bodies), `stake`
  (AURA), `record` (audits done, audits disputed, disputes lost).
- **Disclosure commitment:** `reportCommit = hash(report || salt)`, `researcherDid`, `programId`, `committedAt`.

## 5. Scoring

`ARCHITECTURE.md` "Security Scoring (v0)" is the starting formula. Changes:
- **Inputs are public:** CVE/NVD API 2.0, CISA Known Exploited Vulnerabilities, OSV, vendor advisories, and
  FirmProof's own validated bounty findings. Every input carries its source and date.
- **Exploited counts more than scored:** a vulnerability on the CISA KEV list (actively exploited) weighs more than
  its CVSS alone suggests.
- **Reproducible:** the scoring engine is open source and versioned; each published score names the formula version
  and the input snapshot hash, so anyone can recompute it.
- **Audits change the score only through findings and certifications,** never through who paid for the audit.

## 6. Multichain layers

| Layer | Phase 1-3 (existing chains) | Phase 4 (own parachain) |
|---|---|---|
| AURA token | SPL token on Solana (home). Native on Base, Arbitrum, BSC through a burn-and-mint standard (Wormhole NTT or Hyperlane warp routes; pick after a security review) | Also a native asset on the parachain, bridged the same way |
| Audit attestations | EAS (Ethereum Attestation Service) on Base and Arbitrum; Solana's attestation service on Solana (to be checked) | Pallet `certifications`, mirrored to EAS |
| Registry anchors | Merkle root of the registry, written every hour to Base and Solana | Pallet `anchors`, plus the same roots to Ethereum |
| Bounty escrow | Contract on Base (EVM) and a program on Solana, funded in AURA or USDC | Pallet `bounties` |
| Payments in | Across API V5 (any token on any chain to USDC in the treasury; FirmProof's app fee on those transfers), Telegram Stars and TON through RotBot | Same |

**Choosing the token bridge** is a security decision, not a marketing one. Criteria: no wrapped copies (single
supply), rate limits per chain, a pause switch, who can upgrade the contracts, and its audit and incident history.
Bridges are the most attacked part of crypto (Ronin, Wormhole 2022, Nomad); FirmProof cannot be the next headline.

## 7. Audits and independence

- **Who can audit:** any auditor holding an accreditation credential (national accreditation bodies, recognised
  labs), or a researcher with a track record on FirmProof. Accreditation is a verifiable credential, revocable.
- **Stake:** auditors stake AURA. A dispute (a vendor or researcher shows an audit missed or misstated a finding)
  goes to a panel drawn from the Researchers house (`CHARTER.md`). A lost dispute slashes part of the stake to the
  party who proved it.
- **Conflict of interest:** the vendor may pay for an audit, but the payment goes through FirmProof's escrow, the
  auditor is drawn from a rotating pool, and the attestation records that the vendor paid.
- **International:** audits are tagged by auditor country and method, so a buyer can require, for example, an EU
  accredited lab.

## 8. Bug bounties and disclosure

1. **Program:** a vendor, FirmProof's treasury or the community funds a program for a device or vendor, in AURA or
   USDC, held in escrow, with scope and payout table.
2. **Commit:** the researcher submits only `reportCommit` on-chain. This proves who found the flaw first and when,
   without revealing it.
3. **Private report:** the encrypted report goes to the vendor and to FirmProof's triage team.
4. **Triage:** validated or rejected within a set time; duplicates resolved by commit time.
5. **Embargo:** 90 days or until a patch ships, whichever comes first (`SECURITY.md`).
6. **Reveal and pay:** the researcher reveals the report and salt; the contract checks it matches the commit and
   pays. The vulnerability then enters the score.

Safety rules (from `CHARTER.md`, kept): no exploit code is ever published before a patch; reports stay encrypted
until disclosure; extortion or weaponisation revokes credentials and forfeits stake.

## 9. Links to the rest of the ID ArtCraft stack

- **aurachecker 4.0 route layer:** prices AURA and USDC moves across chains; FirmProof uses it for treasury and
  payouts, and aurachecker uses FirmProof scores in its risk-scored routing.
- **Grim-Hunter:** wallet forensics for bounty payouts (sanctions, flagged addresses) and for auditor stakes.
- **RotBot:** `/firmproof <device>` looks up a score in Telegram; Stars and TON pay for reports and alerts.
- **idarti.com/web3:** the public explorer and vendor portal (`apps/explorer`, `apps/console`).

## 10. Phases and acceptance tests

1. **Registry and scores (off-chain first):**
   - CVE oracle and scoring engine over 1,000 device models from public sources (NVD, KEV, OSV);
   - explorer pages on idarti.com;
   - hourly Merkle anchors on Base and Solana.

   Test: anyone can recompute a published score from the snapshot.
2. **AURA multichain:** AURA native on 2-3 EVM chains through the chosen burn-and-mint bridge, after a review of its
   contracts. Test: supply across chains always equals the Solana-side total; rate limits and pause work on a
   testnet.
3. **Audits and bounties:**
   - EAS schema and Solana attestations;
   - bounty escrow with commit and reveal;
   - auditor staking and disputes;
   - a first paid program on one vendor.

   Test: a full commit, triage, embargo, reveal and payout on mainnet with a small bounty.
4. **Parachain:** finish the Substrate pallets, launch a testnet, then a Polkadot parachain slot (coretime), with
   bridges to Ethereum and Solana. Only when phase 1-3 traffic pays for it. Test: the parachain state root matches
   the anchors on Base and Solana.

## 11. Open questions for Igor

- Token bridge: Wormhole NTT, Hyperlane or LayerZero OFT? (Recommendation after a security review of each.)
- First device category for phase 1: home cameras and routers (most CVEs, most consumer interest) or industrial?
- Legal home: the charter names a Swiss Verein. Is that still the plan, given the EU focus?
- Does AURA keep its current supply and holders as is, or does FirmProof need its own token? (This spec assumes AURA.)
- Accreditation: which bodies do we approach first for auditor credentials?
