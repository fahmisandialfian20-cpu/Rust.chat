# UI/UX Design System Guide

This guide defines the UI/UX baseline for Rust.chat web app tasks. Agent coder should preserve these rules across all feature specs.

## Design principles

1. Permission clarity: the UI should explain when an action is disabled because of permissions or channel feature flags.
2. No private leakage: empty/forbidden states must not reveal hidden channel names, member lists, or private message content.
3. App-first layout: optimize for daily chat use, not a marketing page.
4. Fast feedback: every mutation has loading, success, and error states.
5. Accessible defaults: keyboard, focus, labels, and color contrast are required.
6. Multi-client consistency: web UI should not assume browser-only backend behavior that breaks desktop/mobile contracts.

## Primary surfaces

### Auth shell

- Bootstrap Hoster
- Login
- Register via invite
- Clear distinction between first-owner bootstrap and regular auth
- Stable error copy for invalid credentials/conflicts

### App shell

- Space sidebar or rail
- Channel sidebar
- Main content region
- Reconnect/status banner area
- User/session area
- Responsive collapse behavior for narrower screens

### Chat surface

- Channel header with name/topic/status
- Message history
- Load older history affordance
- Typing indicator region
- Composer with disabled-state explanation
- Attachment affordance only when backend and channel flags allow it

### Admin surface

- Role editor with grouped permission checklist
- Channel settings with feature flag toggles
- Clear warning for dangerous operations
- Backend permission context required before showing controls

### Settings surface

- Theme settings
- Future profile/device settings
- No arbitrary CSS input

## Responsive behavior

Desktop target:

- Three-pane layout: spaces, channels, content.
- Admin pages can use two-pane master/detail layout.

Tablet target:

- Collapsible sidebars.
- Channel list can overlay content.

Small web/mobile browser target:

- Stack navigation and content.
- Keep composer visible and reachable.
- Do not confuse this with the future Flutter mobile app; this is responsive web only.

## Component inventory

Use or create components in these categories:

- Button
- Input
- Textarea
- Label
- Field error
- Card/panel
- Dialog/alert dialog
- Sidebar/nav item
- Badge/status pill
- Avatar/user summary
- Toggle/switch
- Checkbox
- Select/listbox
- Tabs if needed for admin/settings
- Toast/banner
- Skeleton/loading block
- Empty state
- Message item/list
- Composer
- Presence indicator

Prefer shadcn-svelte/Bits UI patterns for accessibility and consistency.

## State design requirements

Every screen must define:

- loading state;
- empty state;
- error state;
- unauthorized/forbidden state when applicable;
- disabled state for blocked actions;
- success state for mutations.

## Accessibility requirements

- Inputs have labels and error messages connected semantically.
- Icon-only buttons have accessible names.
- Presence is not color-only; include text/aria labels.
- Focus rings remain visible.
- Dialogs trap focus when used.
- Keyboard navigation works in menus/lists.
- Motion should respect reduced-motion preferences where possible.

## Visual token policy

- Tokens live in `src/app.css` using Tailwind v4 `@theme` and CSS variables.
- Runtime theme changes use controlled classes/data attributes/CSS variables.
- Do not allow arbitrary user CSS.
- Use OKLCH-compatible color tokens for theme palettes.

## Copywriting rules

- Use direct action labels: `Create Hoster`, `Log in`, `Send`, `Save role`.
- Permission-denied copy should be clear but not reveal hidden data.
- Backend errors should be mapped to stable human-readable messages.
- Developer/raw error details should not be shown to normal users.
