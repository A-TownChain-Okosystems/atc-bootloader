# atc-bootloader

UEFI Bootloader für ShivaCore — der bare-metal Rust-Kernel von GlobusOS.

## Features (geplant)
- UEFI Firmware-Initialisierung
- GPT-Partition-Tabellen-Parsing
- Kernel-Image-Loading (ELF64)
- Kernel-Module-Loading (.km)
- Secure Boot (Signature-Verification)
- Memory-Map-Handoff an Kernel
- VBE/GOP Framebuffer-Setup
- Kommandozeilen-Parameter

## Build
```bash
cargo build --target x86_64-unknown-uefi
```

## Abhängigkeiten
- [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore) — Kernel, der geladen wird

## Status
- Initial: Repo erstellt 05.08.2026
- Sprache: Rust (no_std, UEFI-Target)

---
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
