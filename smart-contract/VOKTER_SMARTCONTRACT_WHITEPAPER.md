## Vokter Wallet Smart Contract Whitepaper (Devnet Open-Source Review)

### Version
Current contract: `programs/vokter-wallet/src/lib.rs` (Anchor, Rust)

### Abstract
Vokter Wallet is a 2‑of‑2 multisig wallet where the owner and an off‑chain Guardian service co‑authorize sensitive operations. The on‑chain program enforces signer, PDA, and state constraints; the Guardian service performs out‑of‑band risk checks and TOTP validation. This document describes the on‑chain design, constraints, invariants, threat model, controls, instruction set, and audit readiness for devnet open‑source review.

### Goals
- Reliable 2‑of‑2 enforcement (owner + guardian) for transfers and V2 governance.
- Owner‑only Break‑Glass recovery with a clear 7‑day (seconds) timelock.
- Predictable, simple, and auditable controls with minimal trust assumptions.
- Safe init/migrate/close flows and rent hygiene.
- Comprehensive emergency recovery system for service disruption scenarios.

### Non‑Goals
- On‑chain TOTP or device biometrics (these are off‑chain in the Guardian service).
- Token custody beyond SOL (SPL support is out of scope for this release).

---

## Architecture Overview

### Accounts and PDAs
- State PDA `VokterWallet` (seeds: `[b"vokter_wallet", owner]`)
  - Fields: `owner`, `guardian`, `version` (1/2), `status` (`Active`/`RotationPending`), `state_bump`, `vault_bump`, `last_emergency_rotation: Option<i64>`, `break_glass_initiated_at: Option<i64>`, daily counters, deprecated V1 2FA fields (layout‑only), `_reserved`.
- Vault PDA `SystemAccount` (seeds: `[b"vokter_vault", owner]`)
  - Owned by the System Program; receives SOL deposits and performs CPI transfers with PDA signing.
- Governance PDA `VokterGovernance` (seeds: `[b"vokter_governance", wallet]`)
  - Fields: `wallet`, `op_seq`, `pending_guardian`, `rotation_eta_slot`, `schedule_cooldown_slot`, `status`, `_reserved`.

### Roles
- Owner: end‑user keypair. Must sign high‑risk ops.
- Guardian: off‑chain service keypair (per‑wallet), derived/managed with AWS controls. Must sign V2 rotation and transfers. Not required for Break‑Glass initiate/execute.

### Versions
- V1: legacy rotation (disabled for V2), legacy 2FA fields are inert (kept for layout).
- V2: governance PDA, emergency rotation with timelock and cancellation, strict signer enforcement, Break-Glass recovery system.

---

## Security Model and Invariants

### Core Invariants
- Transfers require 2‑of‑2 (owner + guardian) and pass destination, PDA, and fee/limit checks.
- V2 schedule/cancel/execute: enforce version, status, signer(s), and timelock.
- Break‑Glass: owner‑only initiate/execute; owner or guardian may cancel; 7‑day timelock in seconds.
- Close wallet: V2 requires owner+guardian and empty vault; V1 requires owner, empty vault, and ≤0.001 SOL wallet lamports.
- No rent leaks: rotation request accounts use `close = owner`; wallet close uses `close = owner`.

### Replay/Nonce Controls
- Durable nonce transactions are rejected.
- Governance `op_seq` uses checked math for comparison and increment.

### PDA Binding and Anti‑Aliasing
- `has_one` constraints bind `owner`/`guardian`.
- Vault/state/governance PDAs use deterministic seeds and are re‑derived and verified at runtime.
- Destination account aliasing checks prevent sending to the wallet, vault, or this program.

### Timelocks
- Break‑Glass: seconds‑based 7 days.
- V2 rotation: slot‑based ~72 hours (constants documented in code).

### Fees and Limits
- Fee rounding uses upward rounding within min/max bounds.
- Daily transaction count and daily fee cap with checked arithmetic.

### Events and Logging Hygiene
- Events avoid sensitive data. Break‑Glass reason is capped (≤200 bytes).
- Note: `TwoFAEnabled` includes a 32‑byte device fingerprint (legacy V1); see Privacy note below.

---

## Instruction Set (High‑Level)

### Initialize
Creates `VokterWallet` PDA, binds `owner` and `guardian`, initializes counters, and ensures vault PDA exists. Version starts at V1; V2 enabled via migrate.

### Transfer (SOL)
Requires 2‑of‑2. Validates PDA linkage, destination anti‑aliasing, durable nonce prohibition, fee bounds, and daily limits. Executes CPI transfers from vault PDA (with PDA signer seeds) to destination and treasury.

### Close Wallet
- V2: owner+guardian sign; vault must be empty.
- V1: owner sign; vault must be empty; wallet lamports ≤0.001 SOL for rent recovery.

### Migrate to V2
Initializes governance PDA (once). Enables V2 rotation features and Break-Glass recovery system.

### V2 Emergency Rotation
- Schedule: owner+guardian sign; governance must be active; uses checked math for `op_seq` and `eta_slot`/cooldown slot computations.
- Cancel: owner **or** guardian may cancel (boolean OR authorization). Context allows guardian‑only cancel.
- Execute: after timelock; owner+guardian sign; updates guardian and clears pending state.

### Break‑Glass (Owner‑Only Recovery)
- Initiate: owner‑only; V2‑only; Active‑only; sets `break_glass_initiated_at`; emits bounded reason.
- Cancel: owner **or** guardian; clears state; emits event.
- Execute: owner‑only; requires ≥7 days in seconds; validates `new_guardian` (not default/owner/current); updates guardian; clears state; emits event.

---

## Break-Glass Recovery System

### System Overview
The Break-Glass Recovery System is an emergency recovery mechanism designed to restore wallet access when the Guardian service becomes permanently unavailable. This system provides a last-resort recovery option while maintaining security through time-based delays and comprehensive validation.

**Purpose**: Enable wallet recovery when Guardian service is permanently offline
**Security Model**: Owner-only initiation with 7-day timelock and guardian validation
**Use Cases**: Guardian service compromise, infrastructure failure, regulatory shutdown

### Recovery Workflow

#### Step 1: Recovery Initiation
```
User Identifies Service Issue → Opens Break-Glass Modal → Selects Target Wallet → 
Acknowledges Warning → Enters TOTP Code → Provides Recovery Reason → 
Smart Contract Transaction → 7-Day Timelock Begins
```

#### Step 2: Timelock Period
- **Duration**: 7 days (604,800 seconds) using blockchain timestamp
- **Activities**: User can cancel recovery, Guardian service can cancel if restored
- **Monitoring**: Public blockchain events show recovery status
- **Security**: No immediate access during timelock period

#### Step 3: Recovery Execution
```
Timelock Expires → User Provides New Guardian → Smart Contract Validation → 
Guardian Update → Recovery Complete → Normal Operations Resume
```

### Security Features

**Access Control**:
- **Initiation**: Owner-only with TOTP verification
- **Cancellation**: Owner OR guardian can cancel during timelock
- **Execution**: Owner-only after timelock expiration
- **Validation**: New guardian must be different from current and owner

**Timelock Protection**:
- **Duration**: 7-day delay prevents immediate access
- **Implementation**: Uses blockchain timestamp for accuracy
- **Validation**: Checked arithmetic prevents overflow attacks
- **Flexibility**: Cancellation possible during timelock period

**Data Protection**:
- **Reason Capping**: Recovery reason limited to 200 characters
- **Event Privacy**: No sensitive data leaked in blockchain events
- **State Management**: Clean state transitions with proper cleanup

### Smart Contract Integration

**Instructions**:
- `initiate_break_glass`: Owner-only recovery initiation with reason documentation
- `cancel_break_glass`: Owner or guardian cancellation during timelock
- `execute_break_glass`: Owner-only execution after timelock expiration

**Contexts**:
- `InitiateBreakGlass`: Owner signature required, V2 wallet validation
- `CancelBreakGlass`: Owner or guardian authorization, state cleanup
- `ExecuteBreakGlass`: Owner signature required, timelock validation

**Events**:
- `BreakGlassInitiated`: Recovery initiation with reason and timestamp
- `BreakGlassCancelled`: Recovery cancellation with actor information
- `BreakGlassExecuted`: Recovery completion with new guardian

**Error Codes**:
- `BreakGlassNotInitiated`: Attempt to execute/cancel without initiation
- `BreakGlassAlreadyActive`: Attempt to initiate when already active

---

## Formalized Controls (Checklist)

1. Signers
   - Transfers: `owner: Signer`, `guardian: Signer`.
   - V2 schedule/execute: `owner: Signer`, `guardian: Signer`.
   - V2 cancel: authorization `(owner || guardian)`; context permits guardian‑only cancel.
   - Break‑Glass: initiate/execute owner‑only; cancel `(owner || guardian)`.

2. PDA and State Integrity
   - State/vault/governance PDAs derived from `[owner]` or `[wallet]` seeds; bumps checked.
   - Governance schedule uses `mut` (no re‑init after migrate) and linkage constraint to wallet.

3. Timelock Integrity
   - Break‑Glass: seconds with checked add/sub; V2: slots and cooldowns.

4. Replay/Nonce and Op‑Seq
   - Durable nonce blocked explicitly.
   - `op_seq` comparison and increment use checked math.

5. Financial Safety
   - Fee upward rounding with caps and ratio bound.
   - Daily limits with checked math.

6. Close and Rent Hygiene
   - Rotation request: `close = owner`.
   - Wallet: `close = owner`; vault must be empty.

7. Events & Privacy
   - No secrets in events; Break‑Glass reason bounded.
   - Privacy note on legacy `device_fingerprint` field.

8. Break-Glass Recovery
   - Owner-only initiation with TOTP verification
   - 7-day timelock using checked arithmetic
   - Reason length capped at 200 characters
   - New guardian validation (not default/owner/current)

---

## Threat Model (Abbreviated)

### Adversaries
- External attacker controlling only owner or only guardian key.
- Malicious dApp attempting aliasing or program‑owned sink destinations.
- Replay via durable nonce or sequence drift.
- Recovery mechanism abuse attempts.

### Sample Scenarios and Outcomes
- Owner compromised only: cannot transfer or rotate guardian in V2 without guardian signature; Break‑Glass execute remains owner‑only but requires 7 days and is publicly observable.
- Guardian compromised only: cannot transfer; cannot schedule/execute V2 rotation without owner signature; cannot Break‑Glass initiate/execute.
- Either party offline: V2 cancel supports owner‑only or guardian‑only paths; Break‑Glass cancel supports either.
- Recovery abuse: 7-day timelock prevents immediate access; rate limiting prevents excessive attempts.

---

## Residual Risk and Privacy Notes

### Residual Risk (program‑only)
- Low. Controls are explicit and use checked arithmetic, PDA cross‑validation, and constrained contexts.
- Break-Glass recovery provides emergency access while maintaining security through timelocks and validation.

### Privacy
- The legacy `TwoFAEnabled` event includes a 32‑byte device fingerprint. This is a V1‑compatibility artifact; consider hashing or omitting in future revisions.
- Break-Glass events include bounded reason text (≤200 bytes) for audit purposes.

---

## Audit Readiness and Evidence

The following have been verified in code and through iterative audit feedback:
- Break‑Glass: owner‑only initiate/execute; owner or guardian cancel; seconds‑based 7 days with checked math; bounded reason; clean state resets.
- V2 rotation: no governance re‑init on schedule; guardian signer enforced by contexts; cancel authorizes owner OR guardian; `op_seq` comparison and increment use checked math; slot‑based execution timelock.
- Transfers: 2‑of‑2; durable nonce blocked; daily limits and fee caps with checked math; PDA‑based CPI from vault.
- Close flows and rent hygiene: enforced and correct.
- Recovery system: comprehensive validation, timelock enforcement, and state management.

---

## Testing and Open‑Source Review Plan

### Suggested Anchor Tests (to accompany the program)
- Break‑Glass lifecycle: initiate → cancel; initiate → execute before/after 7 days; reason length > 200 rejected; guardian default check.
- V2 rotation lifecycle: migrate → schedule → cancel (owner‑only and guardian‑only) → execute after timelock; op_seq near wraparound.
- Transfer: destination aliasing checks; fee/limit edge cases; durable nonce rejection; PDA re‑derivation failures.
- Close: V2 with non‑empty vault rejected; V1 ≤0.001 SOL; vault empty required in all paths.
- Recovery edge cases: timelock validation, guardian validation, state transitions.

### Community Review Guidelines
- Focus on signer/authority correctness, PDA derivations, checked arithmetic, timelock enforcement, account LEN alignment, and rent hygiene.
- Report any aliasing opportunities, unchecked math sites, or missing `has_one`/seed constraints.
- Review Break-Glass recovery logic for security vulnerabilities and edge cases.

---

## Devnet Launch Guidance

### Program ID and Deployment
- Use a devnet program ID; set `declare_id!("<DEVNET_PROGRAM_ID>")` before deployment.
- Confirm governance/rotation constants for devnet monitoring windows.

### Operational Playbooks
- Separate playbooks for Break‑Glass (seconds) and V2 rotation (slots).
- Monitoring for rotation scheduled/cancelled/executed and Break‑Glass initiated/cancelled/executed events.
- Recovery system monitoring and rate limiting configuration.

### Guardian Service Integration (Overview)
- Guardian signatures are verified on‑chain by Signer constraints; authorization and TOTP occur off‑chain under AWS controls (KMS/Secrets Manager/IAM). No local fallback.
- Break-Glass recovery uses specialized TOTP validation endpoints with enhanced security controls.

---

## Roadmap (Post‑Devnet)
- Optional event privacy change for legacy `device_fingerprint`.
- Optional destination policy tightening (e.g., System Program‑owned recipients only, with an "advanced" override).
- SPL token integration and typed transfers.
- Extended governance and parameterization.
- Enhanced recovery system features and monitoring.

---

## Legal Notice
This document is for technical review. Smart contracts may contain defects and should be used with care. No warranties are expressed or implied.


