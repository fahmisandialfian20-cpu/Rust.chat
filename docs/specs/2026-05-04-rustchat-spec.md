# Rust.chat Specification - Phase 0: Repository Foundation

> **Date:** 2026-05-04
> **Status:** Draft

## 1. Overview

Rust.chat is a self-hosted chat application (Discord/Telegram-like) with cross-platform support via Tauri.

## 2. Architecture Update

### Changed from Web-only to Cross-Platform

| Before | After |
|--------|-------|
| SvelteKit (web only) | Tauri + SvelteKit |
| Deploy to VPS | Desktop (Windows/macOS/Linux) + Mobile (iOS/Android) |
| Browser WebSocket | WebSocket (web) + Tauri IPC (native) |

### Tech Stack Validation

**Context7 Verified:**
- Tauri 2.x supports SvelteKit frontend
- Use `@sveltejs/adapter-static` with SPA fallback
- Desktop: Windows, macOS, Linux
- Mobile: iOS, Android (via Tauri)

## 3. Directory Structure

```
rust.chat/
├── apps/
│   ├── server/          # Rust Axum backend
│   └── web/           # SvelteKit + Tauri frontend
├── infra/            # Docker Compose
├── context/          # sudah ada
├── docs/            # specs dan plans
├── .env.example      # template
└── README.md
```

## 4. Dependencies

### Frontend (apps/web/)

```json
{
  "dependencies": {
    "@sveltejs/adapter-static": "latest",
    "@tauri-apps/api": "^2"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2"
  }
}
```

### Build Targets

- **Web:** SvelteKit static build → CDN/VPS
- **Desktop:** Tauri → .exe/.dmg/.AppImage
- **Mobile:** Tauri → .apk/.ipa

## 5. First Implementation

Phase 0 focus: Create directory structure dan basic scaffolding.

See: `docs/superpowers/plans/2026-05-04-repository-foundation.md`