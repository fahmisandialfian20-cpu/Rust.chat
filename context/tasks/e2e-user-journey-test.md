# E2E User Journey Test

**Goal:** Prove the entire MVP core works end-to-end by scripting or automating a complete user flow from bootstrap to first message.

**Scope:** Test script or Playwright test covering full user journey.

**Non-goals:** Production deployment testing, performance testing, mobile testing.

**Priority:** High — validates all MVP components integrate correctly.

---

## User Journey to Test

### Journey A: Hoster Setup (Single User)

```
1. Fresh instance (empty DB)
2. Hoster bootstraps first account
3. Hoster creates space "Team Chat"
4. Hoster creates public channel "general"
5. Hoster sends first message "Hello team!"
6. Verify message appears in channel
```

### Journey B: Invite Flow (Two Users)

```
1. Hoster creates space "Community"
2. Hoster creates private channel "vip"
3. Hoster creates role "VIP" with ViewChannel permission
4. Hoster generates invite link
5. User B registers via invite link
6. User B joins space
7. Admin assigns "VIP" role to User B
8. User B sees "vip" channel
9. User B sends message in "vip"
10. Hoster sees User B's message
```

### Journey C: Permission Enforcement

```
1. Hoster creates role "Viewer" (ReadMessages + ViewChannel only)
2. Assign "Viewer" to User C
3. User C opens channel
4. Verify: User C sees messages but cannot send (composer disabled)
5. Hoster updates role to add SendMessages
6. Verify: User C's composer enables without page reload
```

---

## Implementation Options

### Option 1: Manual Test Script (Recommended for MVP)

Create `docs/testing/e2e-manual-test.md`:
- Step-by-step instructions
- Expected results at each step
- Screenshots or curl commands
- Can be run by any developer in 5 minutes

### Option 2: Playwright Automated Test

Create `apps/web/e2e/journey.spec.ts`:
```typescript
test('hoster creates space and sends message', async ({ page }) => {
  await page.goto('/bootstrap');
  await page.fill('[name=username]', 'hoster');
  await page.fill('[name=password]', 'secret123');
  await page.click('button[type=submit]');
  
  await page.goto('/lobby');
  await page.click('text=Create Space');
  await page.fill('[name=name]', 'Team Chat');
  await page.click('text=Create');
  
  // ... etc
});
```

**Trade-off:** Playwright is robust but requires browser automation setup. Manual script is faster to create for MVP.

---

## Files to Create

| File | Purpose |
|------|---------|
| `docs/testing/e2e-manual-test.md` | Step-by-step manual test script |
| `docs/testing/e2e-playwright-setup.md` | Setup instructions for Playwright (optional) |
| `apps/web/e2e/journey.spec.ts` | Automated Playwright tests (optional) |

---

## Acceptance Criteria

1. Manual script can be followed by developer without codebase knowledge
2. All 3 journeys (A, B, C) pass without errors
3. Each step has expected result clearly stated
4. Common failure modes documented (e.g., "If invite link doesn't work, check...")

---

## Verification

Manual:
```bash
# Follow docs/testing/e2e-manual-test.md
# Time: ~5 minutes per journey
```

Automated (if implemented):
```bash
cd apps/web
npx playwright test e2e/journey.spec.ts
```

---

## References

- `context/tasks/gap-analysis-mvp-completion.md` — Item #4
- `context/00-project-overview.md` — MVP success definition
- `apps/web/src/routes/(auth)/bootstrap/+page.svelte` — bootstrap page
- `apps/web/src/routes/(app)/lobby/+page.svelte` — lobby page

---

**Created:** 2026-05-05
**Depends on:** All other MVP tasks complete
**Estimated effort:** Small-Medium (1 session for manual, 2 for Playwright)
**Risk:** Very low — testing only, no production code changes
