# ATC-Bootloader

Der Bootloader für A-TownChain OS — initialisiert Kernel, lädt Module, startet Chain.

## Boot-Sequenz
```
Power On
    │
    ▼
┌─────────────┐
│  BIOS/UEFI  │   Hardware-Init
└──────┬──────┘
       ▼
┌─────────────┐
│  Bootloader │   Stage 1: MBR → Stage 2
│  (atc-bl)   │   Stage 2: Kernel-Image laden
└──────┬──────┘
       ▼
┌─────────────┐
│  Kernel     │   ShivaCore Kernel init
│  (ShivaOS) │   MMU, Scheduler, Drivers
└──────┬──────┘
       ▼
┌─────────────┐
│  Chain Init │   Bootstrap Node, Genesis
│  (ATC)     │   Consensus, Networking
└──────┬──────┘
       ▼
┌─────────────┐
│  Userspace  │   Shell, Services, Agents
└─────────────┘
```

## Komponenten
- **Stage 1** — MBR/UEFI Boot Sector (512 bytes)
- **Stage 2** — Filesystem driver, Kernel-Image loader
- **Kernel Jump** — Übergabe an ShivaCore Kernel (atc-shivacore)
- **Module Pre-Load** — Essential LKMs laden (kalloc, ksched)
- **Chain Bootstrap** — Bootstrap Node (#14) starten

## Verwandte Repos
- [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore) — Kernel
- [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel) — Kernel Module (Python)

[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]
