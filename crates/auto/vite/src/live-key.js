export function canonicalizeLiveValue(value) {
  if (typeof value === "bigint") {
    return value <= BigInt(Number.MAX_SAFE_INTEGER) && value >= BigInt(Number.MIN_SAFE_INTEGER)
      ? Number(value)
      : value.toString();
  }
  if (Array.isArray(value)) return value.map(canonicalizeLiveValue);
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalizeLiveValue(value[key])]),
    );
  }
  return value;
}

export function liveArgsKey(value) {
  return JSON.stringify(canonicalizeLiveValue(value));
}

export function matchesLiveInvalidation(endpoint, args, message) {
  return (
    message.endpoint === endpoint &&
    (message.args == null || liveArgsKey(message.args) === liveArgsKey(args))
  );
}
