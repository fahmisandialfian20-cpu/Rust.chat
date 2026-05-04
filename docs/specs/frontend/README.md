# Frontend Agent Coder Guide

Dokumen ini adalah entrypoint untuk agent coder yang akan mengeksekusi Phase 9 Frontend MVP.

## Posisi frontend

Frontend di `apps/web` adalah **web app/reference client** untuk Rust.chat, bukan landing page demo. Web app ini harus membuktikan flow produk utama dan menjadi dasar reuse untuk Tauri desktop, sambil tetap menghormati bahwa backend Rust adalah source of truth.

## Reading order wajib

1. `AGENTS.md`
2. Semua file `context/` sesuai urutan project.
3. `docs/specs/frontend/00-context7-validation.md`
4. `docs/specs/frontend/00-web-app-architecture.md`
5. `docs/specs/frontend/00-ui-ux-design-system.md`
6. `docs/specs/frontend/00-library-and-state-policy.md`
7. `docs/specs/frontend/00-plan.md`
8. Spec task pertama yang statusnya `Planned`.

## Cara kerja wajib

1. Pilih task pertama yang statusnya `Planned` dan semua dependency-nya sudah `Done`.
2. Baca spec task tersebut sampai selesai.
3. Implement hanya task itu. Jangan mengerjakan task berikutnya meskipun terlihat mudah.
4. Jalankan validasi task.
5. Update status di `docs/specs/frontend/00-plan.md` dan checkbox terkait di `TODO.md` hanya jika validasi sukses.
6. Stop dan laporkan hasil.

## Batasan penting

- Frontend tidak boleh menjadi authority permissions.
- Jangan menyembunyikan private channel berdasarkan filter lokal dari response yang tidak scoped. Private channel harus tidak pernah diterima oleh user yang tidak punya akses.
- Jangan memakai `$app/stores`; gunakan `$app/state` jika membutuhkan state dari SvelteKit.
- Komponen Svelte harus mengikuti Svelte 5 runes.
- Tailwind harus v4 CSS-first. Jangan buat `tailwind.config.js`.
- `src/app.d.ts` harus mempertahankan scaffold ambient `App` namespace: `Error`, `Locals`, `PageData`, `PageState`, dan `Platform`.
- Jangan menjalankan `npm audit fix --force` tanpa persetujuan, karena bisa downgrade/breaking.
- Jangan menambahkan backend logic ke frontend. Kalau endpoint backend belum tersedia, catat sebagai blocker di hasil task atau buat UI shell yang jelas disabled sesuai spec.

## Validasi standar setiap task

Jalankan dari root repo:

```bash
npm --prefix ./apps/web run check
npm --prefix ./apps/web run build
npm --prefix ./apps/web test
```

Jika command menghasilkan folder generated `.svelte-kit` atau `build`, boleh hapus setelah validasi supaya diff tetap fokus.

## Dokumen spec per task

- `00-context7-validation.md` — hasil validasi Context7 untuk stack frontend.
- `00-web-app-architecture.md` — arsitektur web app/reference client.
- `00-ui-ux-design-system.md` — baseline UI/UX dan accessibility.
- `00-library-and-state-policy.md` — kebijakan library, form, API helper, dan state.
- `00-plan.md` — urutan kerja dan workflow.
- `01-foundation.md` — fondasi SvelteKit/Tailwind/Zod.
- `02-bootstrap-hoster.md` — bootstrap akun Hoster pertama.
- `03-auth-forms.md` — login/register.
- `04-lobby-spaces.md` — lobby dan daftar spaces.
- `05-channel-list.md` — daftar channel yang server-scoped.
- `06-chat-area.md` — history dan composer chat.
- `07-typing-indicator.md` — typing indicator.
- `08-presence-indicator.md` — presence indicator.
- `09-role-editor.md` — admin role editor.
- `10-channel-settings.md` — admin channel settings dan feature flags.
- `11-theme-settings.md` — user theme settings.
- `12-reconnect-banner.md` — WebSocket reconnect banner.
- `13-ws-event-validation.md` — validasi semua event WebSocket.
- `99-mvp-acceptance-flows.md` — flow manual akhir untuk Hoster, Member, composer, dan realtime.
