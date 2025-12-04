# Funding & Roadmap Strategy

**Project**: OLA (Open Linux Authentication)
**Stage**: Alpha (MVP Complete)
**Funding Goal**: $50,000 - $100,000 (Seed/Grant)

## 1. Executive Summary

OLA aims to solve the fragmentation of Linux biometrics by building the "Windows Hello" of the open-source world. We are seeking funding to move from a working prototype (Sprint 1) to a production-grade, multi-modal authentication ecosystem (Sprint 4).

## 2. Value Proposition

*   **For Users**: Seamless, secure face unlock without configuration hell.
*   **For Enterprises**: Standardized biometric management for Linux fleets.
*   **For Distros**: A drop-in, secure component to modernize the desktop experience (GNOME/KDE integration).

## 3. Roadmap & Deliverables

| Phase | Timeline | Deliverables | Estimated Cost |
|-------|----------|--------------|----------------|
| **Phase 1: Hardening** | Q1 2026 | • CI/CD Pipeline<br>• Fuzz Testing<br>• Audit Logging<br>• Packaging (DEB/RPM) | $10,000 |
| **Phase 2: Intelligence** | Q2 2026 | • Face Detection (ONNX)<br>• Liveness Detection<br>• Neural Net Optimization | $25,000 |
| **Phase 3: Expansion** | Q3 2026 | • Fingerprint (libfprint)<br>• FIDO2/YubiKey Support<br>• PAM Module | $25,000 |
| **Phase 4: Ecosystem** | Q4 2026 | • GNOME Settings Panel<br>• Enterprise Fleet API<br>• Distro Integration | $20,000 |

## 4. Budget Breakdown (Seed Phase)

*   **Engineering (Rust/Security)**: 60%
    *   Core daemon development, security audits, performance tuning.
*   **Hardware & Testing**: 15%
    *   IR Cameras, Fingerprint readers, YubiKeys, diverse laptop testbed.
*   **Design & UX**: 15%
    *   UI/UX for setup apps, system tray indicators, accessibility.
*   **Infrastructure**: 10%
    *   CI runners, hosting, signing keys.

## 5. Potential Grant Sources

1.  **NLnet Foundation** (NGI Zero Core): Focus on privacy & open infrastructure.
2.  **Prototype Fund** (Germany): Public interest tech.
3.  **Linux Foundation**: Core Infrastructure Initiative.
4.  **GitHub Sponsors**: Community sustainability.

## 6. Sustainability Model

*   **Open Core**: The daemon is always MIT.
*   **Enterprise Support**: Paid support for fleet deployment & management tools.
*   **Dual Licensing**: (Optional) For proprietary embedded use cases.

## 7. Why Now?

Linux market share is at an all-time high. Hardware is ready (IR cameras standard in laptops). The missing piece is the software infrastructure. OLA fills this gap.
