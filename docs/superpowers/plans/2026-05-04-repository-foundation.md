# Repository Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create directory structure dan basic scaffolding for Rust.chat cross-platform app

**Architecture:** Tauri + SvelteKit untuk desktop/mobile, Axum untuk backend

**Tech Stack:** Rust, Tauri 2.x, SvelteKit, PostgreSQL, Redis

---

### Task 1: Create Directory Structure

**Files:**
- Create: `apps/server/Cargo.toml`
- Create: `apps/server/src/main.rs`
- Create: `apps/web/package.json`
- Create: `apps/web/svelte.config.js`
- Create: `infra/docker-compose.dev.yml`
- Create: `docs/superpowers/specs/2026-05-04-rustchat-spec.md` (done)

- [ ] **Step 1: Create apps/server/Cargo.toml**

```toml
[package]
name = "rust-chat-server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
```

- [ ] **Step 2: Create apps/server/src/main.rs**

```rust
#[tokio::main]
async fn main() {
    println!("Rust.chat server v0.1.0");
}
```

- [ ] **Step 3: Create apps/web/package.json**

```json
{
  "name": "rust-chat-web",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  }
}
```

- [ ] **Step 4: Create apps/web/svelte.config.js**

```javascript
import adapter from '@sveltejs/adapter-static';

export default {
  kit: {
    adapter: adapter({ fallback: 'index.html' })
  }
};
```

- [ ] **Step 5: Copy docker-compose template**

```bash
cp context/templates/docker-compose.dev.yml infra/docker-compose.dev.yml
```

- [ ] **Step 6: Commit**

```bash
git add apps/ infra/ docs/
git commit -m "feat: add repository foundation structure"
```

---

### Verification

```bash
# Check structure
ls -la apps/server/
ls -la apps/web/
ls -la infra/

# Verify Cargo compiles
cd apps/server && cargo check

# Verify SvelteKit builds
cd apps/web && npm run build
```