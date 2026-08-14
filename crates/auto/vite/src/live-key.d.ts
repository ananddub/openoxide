export function canonicalizeLiveValue(value: unknown): unknown;
export function liveArgsKey(value: unknown): string;
export function matchesLiveInvalidation(
  endpoint: string,
  args: unknown,
  message: { endpoint: string; args: unknown },
): boolean;
