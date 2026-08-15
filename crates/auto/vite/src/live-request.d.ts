export type LiveRequestQueueOptions = {
  onValue: (key: string, value: unknown, reason: string) => void;
  onError: (key: string, error: unknown, reason: string) => void;
};

export function createLiveRequestQueue(
  options: LiveRequestQueueOptions,
): (
  key: string,
  request: () => Promise<unknown>,
  trigger: unknown,
  reason: string,
) => Promise<void>;
