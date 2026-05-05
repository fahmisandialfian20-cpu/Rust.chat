import { test, expect } from '@playwright/test';
import {
  bootstrapOwner,
  registerUser,
  createSpace,
  createChannel,
  sendMessage,
  listMessages,
  createInvite,
  acceptInvite,
  createRole,
  assignRole,
  updateRole,
  listVisibleChannels,
} from './helpers';

test.describe('Journey A: Hoster Setup', () => {
  let hosterToken: string;
  let hosterUserId: string;
  let spaceId: string;
  let channelId: string;

  test('A1-A2: Bootstrap owner and create space', async ({ request }) => {
    const auth = await bootstrapOwner(request);
    hosterToken = auth.access_token;
    hosterUserId = auth.user.id;

    const space = await createSpace(request, hosterToken, 'Team Chat', 'Main team space');
    spaceId = space.id;

    expect(space.name).toBe('Team Chat');
  });

  test('A3: Create public channel "general"', async ({ request }) => {
    const channel = await createChannel(request, hosterToken, spaceId, 'general', 'public');
    channelId = channel.id;

    expect(channel.name).toBe('general');
    expect(channel.visibility).toBe('Public');
  });

  test('A4-A5: Send message and verify', async ({ request }) => {
    const msg = await sendMessage(request, hosterToken, channelId, 'Hello team!');
    expect(msg.content).toBe('Hello team!');
    expect(msg.author_user_id).toBe(hosterUserId);

    const messages = await listMessages(request, hosterToken, channelId);
    expect(messages.some((m) => m.content === 'Hello team!')).toBeTruthy();
  });
});

test.describe('Journey B: Invite Flow', () => {
  let hosterToken: string;
  let spaceId: string;
  let channelId: string;
  let vipRoleId: string;
  let inviteCode: string;
  let userBToken: string;
  let userBId: string;

  test.beforeAll(async ({ request }) => {
    const auth = await bootstrapOwner(request);
    hosterToken = auth.access_token;
  });

  test('B1-B3: Create space, private channel, and VIP role', async ({ request }) => {
    const space = await createSpace(request, hosterToken, 'Community', 'Community space');
    spaceId = space.id;

    const channel = await createChannel(request, hosterToken, spaceId, 'vip', 'private');
    channelId = channel.id;
    expect(channel.visibility).toBe('Private');

    const role = await createRole(request, hosterToken, spaceId, 'VIP', [
      'ViewChannel',
      'ReadMessages',
      'SendMessages',
    ]);
    vipRoleId = role.role.id;
    expect(role.permission_keys).toContain('ViewChannel');
  });

  test('B4-B6: Generate invite, register User B, accept invite', async ({ request }) => {
    const invite = await createInvite(request, hosterToken, spaceId);
    inviteCode = invite.code;
    expect(inviteCode).toBeTruthy();

    const auth = await registerUser(request, 'userb', 'password456', inviteCode);
    userBToken = auth.access_token;
    userBId = auth.user.id;

    await acceptInvite(request, userBToken, inviteCode);
  });

  test('B7-B8: Assign role and verify visibility', async ({ request }) => {
    await assignRole(request, hosterToken, spaceId, userBId, vipRoleId);

    const visible = await listVisibleChannels(request, userBToken, spaceId);
    const vipChannel = visible.find((c) => c.id === channelId);
    expect(vipChannel).toBeDefined();
  });

  test('B9-B10: User B sends message, Hoster reads it', async ({ request }) => {
    const msg = await sendMessage(request, userBToken, channelId, 'Hello from User B in VIP!');
    expect(msg.author_user_id).toBe(userBId);

    const hosterMessages = await listMessages(request, hosterToken, channelId);
    const found = hosterMessages.find((m) => m.content === 'Hello from User B in VIP!');
    expect(found).toBeDefined();
    expect(found!.author_user_id).toBe(userBId);
  });
});

test.describe('Journey C: Permission Enforcement', () => {
  let hosterToken: string;
  let spaceId: string;
  let channelId: string;
  let viewerRoleId: string;
  let userCToken: string;
  let userCId: string;
  let inviteCode: string;

  test.beforeAll(async ({ request }) => {
    const auth = await bootstrapOwner(request);
    hosterToken = auth.access_token;

    const space = await createSpace(request, hosterToken, 'PermTest', 'permission test space');
    spaceId = space.id;

    const channel = await createChannel(request, hosterToken, spaceId, 'general', 'public');
    channelId = channel.id;

    const invite = await createInvite(request, hosterToken, spaceId);
    inviteCode = invite.code;

    const userBAuth = await registerUser(request, 'userb-perm', 'password456', inviteCode);
    await acceptInvite(request, userBAuth.access_token, inviteCode);
    const everyoneRole = await createRole(request, hosterToken, spaceId, 'Member', [
      'ViewChannel', 'ReadMessages', 'SendMessages',
    ]);
    await assignRole(request, hosterToken, spaceId, userBAuth.user.id, everyoneRole.role.id);
    await sendMessage(request, userBAuth.access_token, channelId, 'Pre-existing message');
  });

  test('C1-C4: Register User C, create Viewer role, assign it', async ({ request }) => {
    const userCAuth = await registerUser(request, 'userc', 'password789', inviteCode);
    userCToken = userCAuth.access_token;
    userCId = userCAuth.user.id;

    await acceptInvite(request, userCToken, inviteCode);

    const viewerRole = await createRole(request, hosterToken, spaceId, 'Viewer', [
      'ViewChannel',
      'ReadMessages',
    ]);
    viewerRoleId = viewerRole.role.id;
    expect(viewerRole.permission_keys).toEqual(['ViewChannel', 'ReadMessages']);
    expect(viewerRole.permission_keys).not.toContain('SendMessages');

    await assignRole(request, hosterToken, spaceId, userCId, viewerRoleId);
  });

  test('C5: User C can read messages', async ({ request }) => {
    const messages = await listMessages(request, userCToken, channelId);
    expect(messages.length).toBeGreaterThan(0);
  });

  test('C6: User C cannot send messages', async ({ request }) => {
    const res = await request.post(
      `http://localhost:3000/api/v1/channels/${channelId}/messages`,
      {
        headers: { Authorization: `Bearer ${userCToken}` },
        data: { content: 'I should not be able to send' },
      },
    );
    expect(res.status()).toBe(403);
  });

  test('C7-C8: Update role to add SendMessages, then User C can send', async ({ request }) => {
    await updateRole(request, hosterToken, spaceId, viewerRoleId, [
      'ViewChannel',
      'ReadMessages',
      'SendMessages',
    ]);

    const msg = await sendMessage(request, userCToken, channelId, 'Now I can send!');
    expect(msg.content).toBe('Now I can send!');
    expect(msg.author_user_id).toBe(userCId);
  });
});
