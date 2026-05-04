const typingState = $state<Map<string, Map<string, number>>>(new Map());
let _version = $state(0);

let cleanupTimer: ReturnType<typeof setInterval> | undefined;

function ensureCleanup() {
  if (cleanupTimer) return;
  cleanupTimer = setInterval(() => {
    const now = Date.now();
    for (const [channelId, users] of typingState) {
      for (const [userId, ts] of users) {
        if (now - ts > 5000) {
          users.delete(userId);
        }
      }
      if (users.size === 0) {
        typingState.delete(channelId);
      }
    }
    _version++;
  }, 1000);
}

export function setTyping(channelId: string, userId: string, isTyping: boolean): void {
  if (isTyping) {
    if (!typingState.has(channelId)) {
      typingState.set(channelId, new Map());
    }
    typingState.get(channelId)!.set(userId, Date.now());
    ensureCleanup();
  } else {
    const users = typingState.get(channelId);
    if (users) {
      users.delete(userId);
      if (users.size === 0) {
        typingState.delete(channelId);
      }
    }
  }
  _version++;
}

export function getTypingUsers(channelId: string): string[] {
  _version;
  const users = typingState.get(channelId);
  if (!users) return [];
  const now = Date.now();
  const result: string[] = [];
  for (const [userId, ts] of users) {
    if (now - ts <= 5000) {
      result.push(userId);
    }
  }
  return result;
}

export function clearChannel(channelId: string): void {
  typingState.delete(channelId);
  _version++;
}
