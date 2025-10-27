# UI↔Backend Contract - Vokter Extension

## Overview

This document defines the contract between the Vokter extension UI and backend services. It ensures that any UI refactoring preserves the exact same backend function calls, maintaining compatibility while allowing modern UI improvements.

## Core Principles

1. **No Behavior Changes**: All backend interactions must work exactly the same
2. **Selector Preservation**: Existing selectors cannot be renamed or removed
3. **Function Signature Preservation**: Backend function calls must use identical parameters
4. **Event Timing Preservation**: Backend calls must happen at the same moments in user flows

## Screens and Their Backend Triggers

### 1. Send Flow (`send`)
- **Primary Trigger**: `handleSendTransaction` form submission
- **Backend Call**: `VokterContractService.execute2FATransaction()`
- **Dependencies**: 
  - Session validation (password)
  - TOTP verification
  - Guardian service integration
- **Refactor Notes**: 
  - Backend is only called on final confirmation
  - Splitting steps is safe (amount input, address validation, etc.)
  - Fee calculation happens before UI updates

### 2. Receive Flow (`receive`)
- **Primary Trigger**: `onReceive` button click
- **Backend Call**: None (UI-only operation)
- **Dependencies**: None
- **Refactor Notes**: 
  - Completely safe to refactor
  - No backend integration required

### 3. Break-Glass Recovery (`breakGlass`)
- **Primary Triggers**: 
  - `handleOpenBreakGlassModal` (opens modal)
  - `handleTOTPSubmit` (TOTP validation)
  - `handleReasonSubmit` (reason submission)
- **Backend Calls**: 
  - `GuardianServiceClient.validateBreakGlassTOTP()`
  - `BreakGlassAccess` middleware functions
- **Dependencies**: 
  - TOTP validation
  - Emergency access approval workflow
- **Refactor Notes**: 
  - Multi-step process with backend validation at each step
  - TOTP validation must happen before reason submission
  - Guardian service integration is critical

### 4. TOTP Setup (`totp`)
- **Primary Triggers**: 
  - `handleSetup2FA` (opens setup)
  - `handlePasswordSubmit` (password validation)
  - `handleNext` (advances steps)
- **Backend Calls**: 
  - `GuardianServiceClient.provisionTOTP()`
  - `GuardianServiceClient.getChallenge()`
- **Dependencies**: 
  - Wallet keypair for signature generation
  - Device fingerprint generation
  - Guardian service availability
- **Refactor Notes**: 
  - Multi-step wizard with backend calls at specific points
  - Challenge-response flow must be preserved
  - Device fingerprint consistency is critical

### 5. Wallet Setup (`walletSetup`)
- **Primary Triggers**: 
  - `onAddWallet` (opens modal)
  - `handleCreateWallet` (creates new wallet)
  - `handleImportPhrase` (imports from mnemonic)
  - `handleImportPrivateKey` (imports from private key)
- **Backend Calls**: 
  - `WalletService.createWallet()`
  - `WalletService.importWalletFromPhrase()`
  - `WalletService.importWalletFromPrivateKey()`
- **Dependencies**: 
  - Password validation
  - Guardian recovery setup (optional)
- **Refactor Notes**: 
  - Backend calls happen after form validation
  - Guardian recovery integration is optional
  - Wallet creation is atomic operation

### 6. Settings (`settings`)
- **Primary Triggers**: 
  - `handleRecoveryPasswordSubmit` (password verification)
  - `handleRecovery2FASubmit` (2FA verification)
  - `handleManageWallet` (wallet management)
- **Backend Calls**: 
  - `SessionService` password validation
  - `TOTPService` 2FA validation
- **Dependencies**: 
  - Active session
  - 2FA setup completion
- **Refactor Notes**: 
  - Settings are read-only until validation
  - Backend calls only for sensitive operations

### 7. Guardian Rotation (`guardianRotation`)
- **Primary Triggers**: 
  - `handleSubmit` (submits rotation)
  - `handleConfirm` (confirms rotation)
- **Backend Calls**: 
  - Guardian rotation smart contract functions
- **Dependencies**: 
  - Current guardian approval
  - Smart contract governance
- **Refactor Notes**: 
  - Multi-step approval process
  - Smart contract integration is critical

### 8. Emergency Recovery (`emergencyRecovery`)
- **Primary Triggers**: 
  - `handleRecoverySubmit` (submits recovery)
  - `handleNext` (advances steps)
- **Backend Calls**: 
  - `GuardianRecoveryService.recoverGuardianAccess()`
- **Dependencies**: 
  - Recovery phrase validation
  - Master password verification
- **Refactor Notes**: 
  - Recovery is time-sensitive operation
  - Backend calls must be immediate after validation

### 9. Funding (`funding`)
- **Primary Triggers**: 
  - `requestAirdrop` (requests SOL)
  - `checkBalance` (checks balance)
- **Backend Calls**: 
  - Solana RPC airdrop
  - Balance checking
- **Dependencies**: 
  - Network connectivity
  - Wallet address validity
- **Refactor Notes**: 
  - Network operations only
  - No Guardian service dependency

### 10. Wallet Management (`walletManagement`)
- **Primary Triggers**: 
  - `handleWalletSelect` (selects wallet)
  - `handleManageWallet` (manages wallet)
- **Backend Calls**: 
  - Wallet switching (local state)
  - Wallet metadata updates
- **Dependencies**: 
  - Wallet data availability
- **Refactor Notes**: 
  - Mostly local operations
  - Backend calls for persistence only

### 11. Export (`export`)
- **Primary Trigger**: `handleExport` (exports wallet)
- **Backend Calls**: 
  - `GuardianServiceClient` validation
  - Wallet data export
- **Dependencies**: 
  - Guardian service validation
  - Session authentication
- **Refactor Notes**: 
  - Export requires Guardian approval
  - Backend validation is mandatory

### 12. Governance (`governance`)
- **Primary Trigger**: `handleGovernanceAction` (executes action)
- **Backend Calls**: 
  - Vokter smart contract functions
  - Guardian service integration
- **Dependencies**: 
  - Smart contract availability
  - Guardian service integration
- **Refactor Notes**: 
  - Smart contract operations
  - Guardian service is mandatory

## Hard Dependencies (Must Not Rename)

### Critical Selectors
- `button[onClick*='onSend']` - Send quick action
- `form[onSubmit*='handleSendTransaction']` - Send transaction form
- `button[onClick*='handleSetup2FA']` - TOTP setup trigger
- `button[onClick*='handleOpenBreakGlassModal']` - Break glass trigger

### Critical Functions
- `handleSendTransaction` - Main send flow
- `handleSetup2FA` - TOTP setup entry point
- `handleOpenBreakGlassModal` - Emergency recovery entry point
- `onCreateWallet` - Wallet creation entry point

## Safe Refactor Notes

### Send Flow
- **Safe**: Splitting amount input, address validation, fee calculation
- **Safe**: Adding confirmation steps before backend call
- **Unsafe**: Changing when `execute2FATransaction` is called
- **Unsafe**: Modifying TOTP validation flow

### TOTP Setup
- **Safe**: Improving UI/UX of setup wizard
- **Safe**: Adding progress indicators
- **Unsafe**: Changing challenge-response sequence
- **Unsafe**: Modifying device fingerprint generation

### Break Glass
- **Safe**: Improving reason input UI
- **Safe**: Adding confirmation dialogs
- **Unsafe**: Changing TOTP validation timing
- **Unsafe**: Modifying emergency access workflow

### Wallet Setup
- **Safe**: Improving form validation UI
- **Safe**: Adding wallet preview
- **Safe**: Enhancing error handling display
- **Unsafe**: Changing wallet creation sequence

## Backend Service Integration Points

### Guardian Service
- **TOTP Provisioning**: `provisionTOTP()`
- **Challenge Generation**: `getChallenge()`
- **Break Glass Validation**: `validateBreakGlassTOTP()`
- **Transaction Signing**: `sendTransaction()`

### Vokter Contract Service
- **2FA Transaction Execution**: `execute2FATransaction()`
- **Guardian Initialization**: `initializeGuardian()`

### Wallet Service
- **Wallet Creation**: `createWallet()`
- **Wallet Import**: `importWalletFromPhrase()`, `importWalletFromPrivateKey()`

### Session Service
- **Session Management**: `isSessionActive()`, `getPassword()`, `initSession()`

## Testing Requirements

### Before Refactoring
1. Run existing UI tests to establish baseline
2. Verify all backend function calls work
3. Document current user flows

### During Refactoring
1. Use `validateLegacySelectors()` to ensure selectors exist
2. Use `emitLegacyAction()` to maintain event compatibility
3. Use `callBackend()` to ensure same function signatures

### After Refactoring
1. Verify all backend calls happen at same moments
2. Test user flows end-to-end
3. Validate no regression in functionality

## Migration Strategy

### Phase 1: Infrastructure Setup
1. Deploy `legacy-selectors.ts` and `legacy-actions.ts`
2. Set up event listeners for backward compatibility
3. Validate all selectors are present

### Phase 2: UI Refactoring
1. Refactor UI components using legacy wrappers
2. Maintain same event timing and backend calls
3. Use `emitLegacyAction()` for compatibility

### Phase 3: Validation
1. Test all user flows
2. Verify backend integration unchanged
3. Remove legacy wrappers if no longer needed

## Conclusion

The UI↔Backend Contract ensures that Vokter extension UI can be modernized while maintaining 100% compatibility with existing backend services. By following this contract, developers can confidently refactor the UI knowing that all backend interactions will continue to work exactly as before.

**Remember**: The goal is better UX, not different behavior. All backend calls must happen at the same moments with the same parameters, regardless of how the UI looks or feels.
