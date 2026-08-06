# 📋 Komponenten-Plan — atc-bootloader

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** atc-bootloader  
**Name:** ATC Bootloader — Stage 1/2 Boot  
**Beschreibung:** Zweistufiger Bootloader für A-TownChain OS. Stage 1: Firmware init, Stage 2: Kernel handshake, ATCFS mount.  
**Layer:** L0 — Boot  
**Sprint:** 2.4  
**ATC-Standards:** ATC-01, ATC-24

---

## Komponenten

### 1. stage1.atc

**Beschreibung:** Stage 1: BIOS/UEFI init, memory map, firmware handoff

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 2. stage2.atc

**Beschreibung:** Stage 2: Kernel loading, ATCFS mount, device tree, init userspace

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 3. config.atc

**Beschreibung:** Boot configuration: kernel params, boot order, recovery mode

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 4. kernel_handshake.atc

**Beschreibung:** Kernel handshake protocol: version check, capability negotiation

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 5. recovery.atc

**Beschreibung:** Recovery boot: fallback kernel, emergency shell, disk repair

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

## Implementierungs-Reihenfolge

1. `stage1.atc` — Stage 1
2. `stage2.atc` — Stage 2
3. `config.atc` — Boot configuration
4. `kernel_handshake.atc` — Kernel handshake protocol
5. `recovery.atc` — Recovery boot

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

## Dokumentations-Requirements

- ARCHITECTURE.md: Architektur-Baum + Komponenten-Übersicht ✅
- COMPONENT_PLAN.md: Dieser Plan ✅
- FILE_REGISTER.md: Datei-Liste ✅
- STATUS.md: Aktueller Status ✅
- ROADMAP.md: Sprint-Zuordnung ✅
- CHANGELOG.md: Änderungs-Historie ✅

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
