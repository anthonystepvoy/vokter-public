# Guardian SDK Examples

This folder contains integration examples for the Guardian SDK.

## Quick Start Example

```typescript
import { GuardianAdapter } from '@vokter/guardian-sdk';
import { Connection } from '@solana/web3.js';

// Initialize Guardian
const guardian = new GuardianAdapter({
  connection: new Connection('https://api.devnet.solana.com'),
  guardianServiceUrl: 'https://api.vokter.com'
});

// Setup 2FA (one-time)
const { qrCode, secret } = await guardian.setup2FA();
console.log('Scan this QR code with Google Authenticator:', qrCode);

// Initialize wallet on-chain
await guardian.initializeWallet(totpCode);

// Send protected transaction
const signature = await guardian.send({
  to: recipientAddress,
  amount: 0.1,
  totpCode: userTotpCode
});
```

## Integration Steps

1. **Install SDK** (coming soon)
```bash
npm install @vokter/guardian-sdk
```

2. **Setup 2FA**
- Generate QR code for user
- User scans with Google Authenticator
- Initialize wallet on Solana blockchain

3. **Protected Transactions**
- Every transaction requires TOTP code
- Guardian service verifies code
- Smart contract enforces on-chain

## Features

- ✅ Google Authenticator integration
- ✅ Multi-device support
- ✅ Emergency recovery
- ✅ TypeScript support
- ✅ React hooks (coming soon)

## Live Demo

See `demo-app/` folder for a complete React example (coming soon)

## Documentation

For complete API reference, see `/docs` folder

## Support

- Email: support@vokter.com
- Discord: [Coming soon]

---

**Note:** This SDK requires access to Guardian Service infrastructure. Contact us for API keys and integration support.
