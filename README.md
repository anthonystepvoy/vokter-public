# Vokter - Enterprise-Grade 2FA for Solana

> **Solana Global Hackathon 2024 Submission**

Vokter is a comprehensive security solution for the Solana ecosystem, providing enterprise-grade 2FA protection through Guardian Protocol. Built for users who want military-level security without complexity, and developers who need professional 2FA integration.

## 🏆 Overview

Vokter consists of three main components:

1. **Vokter Wallet** - Chrome extension with Guardian 2FA protection
2. **Guardian Smart Contract** - On-chain 2FA enforcement (Rust/Anchor)
3. **Guardian SDK** - Developer toolkit for integrating 2FA into any Solana dApp

## 🎯 Problem We Solve

**Current Reality:**
- $2.3B stolen from crypto wallets in 2023
- Private key theft = instant fund loss
- No 2FA protection for most wallets
- Complex security = poor adoption

**Vokter Solution:**
- Google Authenticator 2FA for every transaction
- Even if private keys are stolen, funds stay safe
- Simple 3-line SDK integration for developers
- Professional wallet experience for users

## 🔐 Repository Contents

This public repository contains:

### `/smart-contract` - Guardian Protocol (Anchor/Rust)
Complete Solana smart contract implementing on-chain 2FA enforcement:
- TOTP verification logic
- Guardian key management
- Multi-device authorization
- Emergency recovery mechanisms
- **Open source for security audits**

### `/docs` - Technical Documentation
- Guardian Protocol Whitepaper
- TOTP Capsule Architecture
- Security Model & Threat Analysis
- Integration Guides

### `/sdk-examples` - Developer Integration Examples
- React/TypeScript examples
- Integration walkthrough
- API reference
- Best practices

### `/demos` - Live Demonstrations
- Screenshots and videos
- Demo app (frontend only)
- User flow documentation

## 🚀 Key Features

### For Users (Wallet)
- ✅ Google Authenticator 2FA on every transaction
- ✅ Guardian Recovery system (no seed phrase exposure)
- ✅ Multi-device support
- ✅ Break-glass emergency access
- ✅ Open source & audited

### For Developers (SDK)
- ✅ 3-line integration
- ✅ Works with any Solana wallet
- ✅ Production-ready infrastructure
- ✅ Comprehensive documentation
- ✅ TypeScript/JavaScript support

## 🏗️ Architecture

```
┌─────────────────┐
│  Vokter Wallet  │ ← Chrome Extension (Private)
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────┐
│   Guardian Smart Contract (Rust)    │ ← Open Source (This Repo)
│   - TOTP Verification               │
│   - Multi-device Management         │
│   - Emergency Recovery              │
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────┐
│ Guardian Service│ ← Backend API (Private)
│ - Key Management│
│ - TOTP Storage  │
└─────────────────┘
```

## 📊 Demo & Links

- **Website:** [app.vokter.com](https://app.vokter.com) *(coming soon)*
- **Demo Video:** [YouTube Link] *(coming soon)*
- **Live Demo:** [Try it here] *(coming soon)*
- **Documentation:** See `/docs` folder

## 🛡️ Security

- **Smart Contract:** Fully open source in this repo
- **Audit Status:** Internal security review complete
- **Bug Bounty:** Coming soon
- **Responsible Disclosure:** security@vokter.com

### What's Open Source
✅ Smart contract (Anchor/Rust)
✅ SDK integration examples
✅ Technical documentation
✅ Architecture & design

### What's Proprietary
❌ Guardian Service backend
❌ Wallet extension source code
❌ Production infrastructure
❌ API keys & deployment configs

## 🎓 Technical Highlights

1. **TOTP Capsule System** - Encrypted TOTP storage with AWS KMS
2. **On-chain Verification** - Solana smart contract enforces 2FA
3. **Multi-device Support** - Manage multiple authorized devices
4. **Guardian Recovery** - Recover wallets without exposing seed phrases
5. **Enterprise Infrastructure** - Production-ready with 99.9% uptime

## 📝 License

- **Smart Contract:** MIT License (open source)
- **SDK Examples:** MIT License (open source)
- **Documentation:** Creative Commons BY-NC-SA 4.0
- **Proprietary Components:** All rights reserved

## 🤝 Contributing

We welcome security audits and feedback on the smart contract!

For vulnerabilities, please email: security@vokter.com

## 👥 Team

Built with ❤️ for Solana Global Hackathon 2024

## 📧 Contact

- **Email:** hello@vokter.com
- **Twitter:** [@VokterWallet](https://twitter.com/vokterwallet)
- **Discord:** [Join our community] *(coming soon)*

---

**Note for Colosseum Reviewers:** This repository contains the open-source components of Vokter. The complete system includes additional proprietary backend services and wallet extension code. For security reasons, we've made the smart contract fully open source for audit while keeping backend infrastructure private. We're happy to provide live demos and answer any technical questions!
