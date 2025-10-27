### Vokter Guardian Service Technical Specification

Version 4.0 • This document covers the off‑chain Guardian authentication service architecture and operations. The wallet whitepaper covers the browser extension implementation.

—

### Executive Summary

The Vokter Guardian Service is a specialized authentication infrastructure that enables secure two‑factor verification for blockchain transactions. The service validates time‑based one‑time passwords (TOTP) off-chain and generates cryptographic cosignatures for authorized transactions using wallet‑specific guardian keys. The Solana smart contract enforces dual-signature requirements and governance rules, while the Guardian service provides the 2FA validation layer. This document provides technical specifications, operational procedures, and security frameworks for system evaluators and potential integration partners.

**Latest Security Implementations (Q3 2025):**
The Guardian Service includes comprehensive security enhancements with multi-tier DDoS protection, enhanced rate limiting systems, WebSocket security improvements, and refined input validation. These implementations strengthen the service's production readiness while maintaining high availability standards.

**V2 Governance Implementation:**
Production deployment of advanced governance capabilities including emergency guardian rotation with timelock protection, wallet migration support, and enhanced access controls. The V2 system implements a dual-PDA architecture for improved security isolation.

**Smart Contract Security Updates:**
Completed security assessment and hardening of Solana smart contracts with enhanced emergency rotation mechanisms, account versioning support, and strengthened governance controls for production deployment.

**Emergency Recovery System:**
Full implementation of Break-Glass recovery capabilities with specialized TOTP validation, recovery-specific rate limiting, and smart contract integration. This provides users with secure emergency access options during service disruption scenarios.

---

## Table of Contents

1. [Service Architecture and Role](#1-service-architecture-and-role)
2. [Service Interfaces and Integration](#2-service-interfaces-and-integration)
3. [Security Framework and Controls](#3-security-framework-and-controls)
4. [Infrastructure and Operations](#4-infrastructure-and-operations)
5. [Business Continuity and Recovery Procedures](#5-business-continuity-and-recovery-procedures)
6. [Threat Analysis and Risk Mitigation](#6-threat-analysis-and-risk-mitigation)
7. [Recent Security Enhancements](#7-recent-security-enhancements)
8. [V2 Governance Architecture](#8-v2-governance-architecture)
9. [Smart Contract Security](#9-smart-contract-security)
10. [Break-Glass Recovery System](#10-break-glass-recovery-system)
11. [Legal Notice and Service Disclaimers](#11-legal-notice-and-service-disclaimers)

—

## 1. Service Architecture and Role

**Primary Functions:**
- **Authentication Verification**: Validates 6‑digit TOTP codes (RFC 6238 compliant) against wallet‑specific cryptographic secrets off-chain
- **Transaction Cosigning**: Generates Ed25519 cryptographic signatures for user‑prepared transactions following successful authentication
- **Key Management**: Maintains isolated guardian keypairs with each wallet assigned a unique guardian public key
- **Security Monitoring**: Implements comprehensive logging and monitoring of all authentication activities
- **V2 Governance**: Advanced governance features including emergency rotation and wallet migration capabilities
- **Break-Glass Recovery**: Emergency recovery TOTP validation and recovery support operations

**Service Boundaries:**
- **Non‑Custodial Operation**: Service does not store or access user assets or private keys at any time
- **Transaction Independence**: Service does not initiate, modify, or broadcast blockchain transactions independently
- **Smart Contract Compliance**: Cannot circumvent on‑chain dual‑signature validation requirements
- **Isolated Operations**: Each wallet operates with dedicated guardian keys preventing cross‑wallet security dependencies
- **Recovery Support**: Provides TOTP validation for emergency recovery operations without compromising security

**Security Guarantees:**
The Guardian Service operates as a verification layer that translates off‑chain authentication success into cryptographic authorization signals. The architecture ensures that valid guardian signatures can only be generated following successful TOTP verification, creating a cryptographically verifiable link between multi‑factor authentication and transaction authorization. The smart contract enforces dual-signature requirements and governance rules, while the Guardian service provides the 2FA validation layer. For Break-Glass recovery, the service provides specialized TOTP validation while maintaining the same security standards.

—

## 2. Service Interfaces and Integration

**API Endpoints:**
- **Authentication Challenge**: Generates cryptographic challenges for wallet ownership verification using signed message protocols
- **TOTP Provisioning**: Establishes wallet‑specific 2FA secrets and generates QR codes for authenticator app integration
- **Guardian Key Management**: Provides wallet‑specific guardian public keys for smart contract integration
- **Transaction Cosigning**: Validates TOTP codes and generates guardian signatures for authorized transactions
- **V2 Governance**: Advanced governance endpoints for emergency rotation, wallet migration, and governance status
- **Break-Glass Recovery**: Specialized TOTP validation for emergency recovery operations

**Integration Protocol:**
- **HTTPS Communication**: All service communication secured with TLS encryption and certificate validation
- **Professional Domain**: Service accessible via `https://api.vokter.app` with valid SSL certificate
- **API Authentication**: Request authentication using API keys with rate limiting and request validation
- **Session Management**: Sticky session support with AWS Application Load Balancer for consistent routing
- **Device Fingerprinting**: Stable device identification for multi‑device authorization tracking

**Security Controls:**
- **Request Validation**: Comprehensive validation of all incoming requests including signature verification
- **Rate Limiting**: Protection against automated attacks with progressive rate limiting policies
- **Replay Prevention**: Time‑bounded challenges and nonce‑based request validation
- **Error Handling**: Secure error responses that prevent information leakage while providing appropriate feedback

**Operational Characteristics:**
The service implements enterprise‑grade availability and reliability standards with geographic distribution, health monitoring, and automatic failover capabilities. API interfaces are designed for backward compatibility while supporting future enhancements and security improvements.

—

## 3. Security Framework and Controls

**Authentication Security:**
- **Off‑Chain TOTP Processing**: Authentication codes processed exclusively off‑chain; only cryptographic signatures transmitted to blockchain
- **Wallet Isolation**: Each wallet assigned unique guardian keypair preventing cross‑wallet authentication dependencies
- **Signature Binding**: Guardian signatures cryptographically bound to specific transaction data including recent blockhash for replay prevention
- **Key Lifecycle Management**: Secure generation, storage, and rotation of guardian keys with appropriate access controls
- **Recovery TOTP Validation**: Specialized validation for emergency recovery operations with enhanced security controls

**Infrastructure Security:**
- **Transport Encryption**: All communications secured with TLS 1.3 and strict certificate validation policies
- **Professional Domain**: SSL certificate issued for `api.vokter.app` with proper domain validation
- **API Security**: Comprehensive request validation, authentication, and authorization mechanisms
- **Multi-Tiered Rate Limiting**: Advanced rate limiting with wallet-specific and IP-based protection preventing abuse while maintaining service availability
- **Enhanced Input Validation**: Comprehensive validation and sanitization of all inputs with rejection of malformed or suspicious requests
- **Advanced Security Headers**: 15+ security headers providing comprehensive protection against common web vulnerabilities

**Data Protection:**
- **Encryption at Rest**: TOTP secrets and sensitive data encrypted using AWS Key Management Service (KMS)
- **Access Control**: Least‑privilege access policies with role‑based permissions and audit logging
- **Key Management**: Hardware‑backed key storage with secure key derivation and rotation procedures
- **Data Retention**: Appropriate data retention policies with secure deletion of expired authentication data

**Monitoring and Observability:**
- **Comprehensive Logging**: Structured logging of all authentication attempts, failures, and security events
- **Real‑Time Monitoring**: CloudWatch integration with automated alerting for anomalous patterns
- **Performance Metrics**: Service performance tracking with availability and response time monitoring
- **Advanced Threat Detection**: Pattern recognition and behavioral analysis for potential security threats with automated response capabilities

—

## 4. Infrastructure and Operations

**Deployment Architecture:**
- **Container Platform**: Service deployed on AWS ECS Fargate providing serverless container execution with automatic scaling
- **Load Balancing**: Application Load Balancer with health checks, automatic failover, and geographic distribution
- **HTTPS Support**: Dual listeners configured for both HTTP (port 80) and HTTPS (port 443) with SSL certificate
- **Auto‑Scaling**: Dynamic scaling based on demand with performance monitoring and resource optimization
- **Network Security**: Virtual Private Cloud (VPC) deployment with security groups and network access control

**Key Management Infrastructure:**
- **AWS Key Management Service**: Hardware‑backed key storage with audit logging and least‑privilege access controls
- **Key Rotation**: Automated key rotation procedures with secure key lifecycle management
- **Access Auditing**: Comprehensive audit trails for all key access and cryptographic operations
- **IAM Policies**: Strict identity and access management with role‑based permissions and multi‑factor authentication

**Data Storage:**
- **DynamoDB**: Scalable NoSQL database with partition‑key isolation for wallet‑specific data
- **Data Isolation**: Strict data separation preventing cross‑wallet information access
- **Backup and Recovery**: Automated backup procedures with point‑in‑time recovery capabilities
- **Performance Optimization**: Read/write capacity optimization with automatic scaling based on demand

**Protection Mechanisms:**
- **Web Application Firewall**: AWS WAF protection against common web application attacks
- **DDoS Protection**: AWS Shield protection against distributed denial‑of‑service attacks
- **Rate Limiting**: Multi‑layer rate limiting with progressive blocking of suspicious activity
- **Intrusion Detection**: Automated monitoring for unauthorized access attempts and anomalous behavior

**Monitoring and Alerting:**
- **CloudWatch Integration**: Comprehensive metrics collection with real‑time monitoring dashboards
- **Automated Alerting**: Immediate alerts for service degradation, error rate increases, and security events
- **Performance Analytics**: Detailed analysis of response times, throughput, and resource utilization
- **Operational Insights**: Business intelligence dashboards for service optimization and capacity planning

—

## 5. Business Continuity and Recovery Procedures

**Guardian Key Management:**
- **Planned Rotation**: Smart contract supports guardian key rotation with 48‑hour timelock for planned maintenance and security updates
- **Emergency Procedures**: Owner‑controlled emergency rotation capabilities for immediate security response scenarios
- **V2 Emergency Rotation**: Advanced emergency rotation with 72-hour timelock, cancellation capabilities, and anti-replay protection
- **Rotation Validation**: Comprehensive validation procedures ensuring guardian key changes maintain security and operational continuity
- **Audit Logging**: Complete audit trails for all guardian key rotation activities with timestamp and actor information

**Service Continuity:**
- **High Availability Design**: Multi‑zone deployment with automatic failover and geographic redundancy
- **Maintenance Procedures**: Planned maintenance windows with advance notification and user guidance
- **Capacity Management**: Proactive capacity planning with automatic scaling to handle demand spikes
- **Disaster Recovery**: Comprehensive disaster recovery procedures with defined recovery time objectives

**Recovery Mechanisms:**
- **Offline Recovery Capabilities**: Backup authentication mechanisms for service unavailability scenarios
- **Recovery Code Generation**: Secure generation of offline recovery codes with appropriate security controls
- **Single‑Use Protection**: Recovery codes designed for one‑time use with automatic invalidation after successful recovery
- **Rate Limiting**: Recovery attempt rate limiting to prevent abuse while maintaining legitimate access
- **Break-Glass Integration**: Seamless integration with smart contract recovery mechanisms

**Communication Protocols:**
- **Status Monitoring**: Real‑time service status monitoring with user‑facing status indicators
- **Incident Communication**: Established procedures for communicating service issues and recovery timelines
- **User Guidance**: Clear user instructions for various service scenarios including degraded service operations
- **Escalation Procedures**: Defined escalation paths for different types of service incidents

—

## 6. Threat Analysis and Risk Mitigation

**Service Infrastructure Threats:**
- **Service Compromise**: Mitigated through KMS isolation, minimal IAM permissions, and hardware‑backed key storage with comprehensive audit logging
- **Unauthorized Access**: Prevention through multi‑factor authentication, least‑privilege access policies, and continuous monitoring of access patterns
- **Data Exfiltration**: Protection via encryption at rest, secure communication channels, and access control mechanisms
- **Infrastructure Attacks**: Defense through AWS native security services, network segmentation, and intrusion detection systems

**Authentication Security Threats:**
- **TOTP Brute Force**: Mitigation via progressive rate limiting, account lockouts, and reputation‑based blocking mechanisms
- **Replay Attacks**: Prevention through transaction‑specific signature binding, blockhash expiration, and nonce validation
- **Man‑in‑the‑Middle**: Protection via TLS encryption, certificate pinning, and secure communication protocols
- **Session Hijacking**: Mitigation through secure session management, automatic expiration, and cryptographic session binding

**Operational Security Threats:**
- **Insider Threats**: Mitigation through separation of duties, comprehensive audit logging, approval workflows, and background checks
- **Social Engineering**: Protection via security awareness training, verification procedures, and multi‑person approval processes
- **Supply Chain Attacks**: Defense through dependency scanning, verified builds, and secure deployment pipelines
- **Physical Security**: Protection through cloud infrastructure security, secure development environments, and access controls

**Business Continuity Threats:**
- **Service Availability**: Mitigation through redundant infrastructure, automatic failover, and disaster recovery procedures
- **Denial of Service**: Protection via AWS Shield, rate limiting, and traffic analysis with automatic blocking
- **Regulatory Changes**: Adaptation through compliance monitoring, legal review processes, and flexible architecture design
- **Key Compromise**: Response through emergency rotation procedures, incident response plans, and forensic capabilities

—

## 7. Recent Security Enhancements

### 7.1 Wallet-Specific Rate Limiting

**Enhanced Protection**: Implementation of wallet-specific rate limiting in addition to existing IP-based protection
**Business Impact**: Prevents rate limit bypass attempts while maintaining service availability for legitimate users
**Technical Implementation**: Each wallet address has independent rate limits (10 requests per 5 minutes per wallet)
**Security Benefits**: Granular control over request patterns and enhanced protection against automated attacks

### 7.2 Advanced Input Validation & Sanitization

**Comprehensive Protection**: Enhanced input validation with sanitization against XSS, injection, and data tampering attacks
**Business Impact**: Maintains data integrity and prevents potential service disruption
**Technical Implementation**: Multi-layer validation with pattern detection and behavioral analysis
**Security Benefits**: Protection against common web application vulnerabilities while maintaining performance

### 7.3 Enhanced Security Headers

**Comprehensive Coverage**: Implementation of 15+ security headers providing defense-in-depth protection
**Business Impact**: Enhanced security posture suitable for enterprise and institutional use
**Technical Implementation**: Content Security Policy, HSTS, frame protection, and additional security measures
**Security Benefits**: Protection against clickjacking, MIME sniffing, XSS, and other common attack vectors

### 7.4 Production-Grade CORS Policies

**Strict Origin Validation**: Enhanced CORS policies with strict origin checking in production environments
**Business Impact**: Improved security while maintaining compatibility with legitimate client applications
**Technical Implementation**: Origin validation, method restriction, and controlled header exposure
**Security Benefits**: Prevention of unauthorized cross-origin access while supporting legitimate integrations

### 7.5 Advanced Threat Detection

**Behavioral Analysis**: Implementation of behavioral analysis and anomaly detection capabilities
**Business Impact**: Proactive threat detection and response capabilities
**Technical Implementation**: Pattern recognition, input complexity analysis, and encoding attempt detection
**Security Benefits**: Early detection of potential security threats and automated response capabilities

### 7.6 Security Monitoring & Logging

**Enhanced Visibility**: Comprehensive security event logging and monitoring capabilities
**Business Impact**: Improved incident response and compliance capabilities
**Technical Implementation**: Security event tracking, audit trails, and automated alerting
**Security Benefits**: Quick threat identification and comprehensive audit capabilities

### 7.7 AWS Infrastructure Security

**Enhanced IAM Policies**: Comprehensive IAM role configuration with least-privilege access principles
**Business Impact**: Improved infrastructure security and compliance with security best practices
**Technical Implementation**: Task execution roles, task roles, and service-linked roles with appropriate permissions
**Security Benefits**: Reduced attack surface and enhanced access control for production environments

### 7.8 Secrets Management

**AWS Secrets Manager Integration**: Secure storage and retrieval of sensitive configuration values
**Business Impact**: Enhanced security for production deployments with proper secrets management
**Technical Implementation**: Automatic secrets loading on service startup with fallback mechanisms
**Security Benefits**: Secure handling of encryption keys and API credentials with audit logging

—

## 8. V2 Governance Architecture

### 8.1 Two-PDA Model

**Architecture Overview**: The V2 system introduces a two-PDA architecture separating wallet operations from governance functions
**Business Impact**: Enhanced security through separation of concerns and improved governance capabilities
**Technical Implementation**: 
- `VokterWallet` PDA: Core wallet functionality and state management
- `VokterGovernance` PDA: Governance operations including emergency rotation and migration

**Security Benefits**: Isolated governance functions prevent cross-contamination of wallet operations and governance activities

### 8.2 Emergency Guardian Rotation

**Advanced Rotation System**: Owner-controlled emergency rotation with comprehensive security controls
**Business Impact**: Immediate response capabilities for security incidents while maintaining operational security
**Technical Implementation**: 
- 72-hour timelock for rotation execution
- Owner-only initiation with guardian signature requirement
- Cancellation capability during timelock period
- Anti-replay protection using operation sequence numbers

**Security Benefits**: Prevents unauthorized rotation while enabling rapid response to security threats

### 8.3 Wallet Migration to V2

**Seamless Upgrade Path**: Automatic migration from V1 to V2 architecture without service interruption
**Business Impact**: Future-proof architecture supporting enhanced security features and governance capabilities
**Technical Implementation**: 
- Automatic version detection and migration
- Backward compatibility for existing V1 wallets
- Enhanced security features for V2 wallets
- Governance PDA initialization during migration

**Security Benefits**: Gradual security enhancement without disrupting existing user operations

### 8.4 Governance API Endpoints

**Comprehensive Governance Interface**: RESTful API endpoints for all V2 governance operations
**Business Impact**: Programmatic access to governance functions for advanced users and integrations
**Technical Implementation**: 
- `/api/v2/governance/status/{walletAddress}`: Current governance status
- `/api/v2/governance/migrate`: Wallet migration to V2
- `/api/v2/governance/schedule-rotation`: Emergency rotation scheduling
- `/api/v2/governance/cancel-rotation`: Rotation cancellation
- `/api/v2/governance/execute-rotation`: Rotation execution
- `/api/v2/governance/challenge`: Challenge-response verification

**Security Benefits**: Secure governance operations with proper authentication and authorization controls

—

## 9. Smart Contract Security

### 9.1 Security Model Overview

**Dual-Signature Enforcement**: The smart contract enforces that both owner and guardian signatures are required for transactions
**Business Impact**: Enhanced security through multi-signature requirements while maintaining operational flexibility
**Technical Implementation**: 
- All transactions require both owner and guardian signatures
- Guardian service provides 2FA validation off-chain
- Smart contract verifies signature existence and governance rules
- Governance operations managed through dedicated PDA structure

**Security Benefits**: Strong protection against single-point-of-failure scenarios while enabling advanced governance features

### 9.2 Account Versioning

**Future-Proof Architecture**: Version-based account management supporting multiple wallet generations
**Business Impact**: Long-term architecture sustainability with backward compatibility
**Technical Implementation**: 
- Version field in wallet accounts
- Conditional feature activation based on version
- Migration instructions for version upgrades
- Backward compatibility for existing V1 wallets

**Security Benefits**: Gradual security enhancement without disrupting existing operations

### 9.3 Enhanced Error Handling

**Comprehensive Error Management**: Detailed error codes and messages for improved debugging and user experience
**Business Impact**: Better user experience and developer integration capabilities
**Technical Implementation**: 
- Specific error codes for different failure scenarios
- Detailed error messages for debugging
- Proper error propagation through instruction calls
- User-friendly error descriptions

**Security Benefits**: Improved security through better error handling and reduced information leakage

### 9.4 Program ID Updates

**Current Program ID**: `72ptQpPen2neQ9esXAM6FWUajrXfVHgoi6EitR7jWjq` (Devnet)
**Business Impact**: Updated smart contract deployment with enhanced security features
**Technical Implementation**: 
- New program ID for V2 smart contract
- Updated client configurations across all components
- Backward compatibility considerations
- Deployment verification and testing

**Security Benefits**: Latest security enhancements and bug fixes from the hardened smart contract

### 9.5 Security Architecture Summary

**2FA Implementation**: Two-factor authentication is enforced off-chain by the Guardian service through TOTP validation
**Smart Contract Role**: The program verifies guardian signatures exist and enforces governance rules
**Security Model**: Hybrid approach combining off-chain 2FA validation with on-chain signature verification
**Risk Mitigation**: Multiple layers of security including rate limiting, input validation, and comprehensive monitoring

—

## 10. Break-Glass Recovery System

### 10.1 System Overview

**Emergency Recovery Architecture**: The Break-Glass Recovery System provides emergency recovery capabilities when the Guardian service becomes permanently unavailable
**Business Impact**: Ensures business continuity and user access during service disruption scenarios
**Technical Implementation**: 
- Specialized TOTP validation endpoints for recovery operations
- Recovery-specific rate limiting and security controls
- Integration with smart contract recovery mechanisms
- Comprehensive audit logging of all recovery attempts

**Security Benefits**: Maintains security standards while providing emergency recovery options

### 10.2 Recovery TOTP Validation

**Specialized Authentication**: Dedicated TOTP validation for emergency recovery operations
**Business Impact**: Secure recovery authentication without compromising regular service operations
**Technical Implementation**: 
- `/api/guardian/break-glass/validate-totp` endpoint
- Enhanced validation for recovery-specific scenarios
- Recovery attempt rate limiting and monitoring
- Comprehensive audit trail for all recovery operations

**Security Benefits**: Isolated recovery authentication with enhanced security controls

### 10.3 Recovery Rate Limiting

**Recovery-Specific Controls**: Specialized rate limiting for emergency recovery operations
**Business Impact**: Prevents abuse of recovery mechanisms while maintaining legitimate access
**Technical Implementation**: 
- Separate rate limits for recovery TOTP validation
- Enhanced monitoring for recovery attempt patterns
- Progressive blocking for suspicious recovery activity
- Integration with overall security monitoring systems

**Security Benefits**: Protection against recovery mechanism abuse while maintaining service availability

### 10.4 Recovery Integration

**Smart Contract Integration**: Seamless integration with on-chain recovery mechanisms
**Business Impact**: End-to-end recovery workflow from TOTP validation to blockchain execution
**Technical Implementation**: 
- TOTP validation for recovery initiation
- Challenge-response verification for recovery authorization
- Integration with smart contract recovery instructions
- Comprehensive error handling and user guidance

**Security Benefits**: Secure recovery workflow with multiple validation layers

### 10.5 Recovery Monitoring

**Comprehensive Tracking**: Complete monitoring of all recovery operations and attempts
**Business Impact**: Enhanced security monitoring and incident response capabilities
**Technical Implementation**: 
- Recovery attempt logging and analysis
- Pattern detection for suspicious recovery activity
- Integration with security monitoring systems
- Automated alerting for recovery-related security events

**Security Benefits**: Early detection of potential recovery mechanism abuse

—

## 11. Legal Notice and Service Disclaimers

**Service Description:**
The Vokter Guardian Service provides authentication infrastructure for blockchain transaction authorization. The service operates as a non‑custodial authentication layer and does not hold, control, or have access to user assets or private keys at any time.

**No Investment Advice:**
This document provides technical information about authentication service architecture and operations. The information contained herein does not constitute investment advice, financial guidance, or recommendations regarding any investment strategy or financial decisions. Users should conduct independent research and consult qualified professionals before making any decisions.

**Service Limitations:**
While the service is designed for high availability and reliability, no guarantees are provided regarding uptime, performance, or availability. Service levels, features, and operational characteristics may be subject to change based on technical requirements, regulatory considerations, or operational needs.

**No Asset Custody:**
The Guardian Service does not take custody of user assets, does not control user funds, and cannot initiate or prevent blockchain transactions independently. The service provides only authentication verification and cryptographic cosigning capabilities as part of a dual‑signature authorization framework.

**Technology Risks:**
Authentication services involve technical complexity and potential failure modes. Users should understand the technology, maintain appropriate backup procedures, and assess risks according to their individual circumstances. No technology system can be guaranteed to be completely secure or error‑free.

**Regulatory Considerations:**
Authentication services and blockchain technologies may be subject to evolving regulatory requirements in different jurisdictions. The service architecture is designed to support compliance requirements, but users are responsible for understanding applicable regulations in their location.

**Forward‑Looking Statements:**
Any statements regarding future service capabilities, features, or development represent current intentions and are subject to change based on technical, operational, and regulatory factors. No guarantee is provided regarding the implementation or timing of any future capabilities.

**Contact Information:**
Technical Inquiries: security@vokter.com  
Document Classification: Public Technical Documentation  
Last Updated: August 2025  
Version: 3.0 Production Release with V2 Governance Features and Break-Glass Recovery System




