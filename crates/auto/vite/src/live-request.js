export function createLiveRequestQueue({ onValue, onError }) {
  const requests = new Map();

  return function queueLiveRequest(key, request, trigger, reason) {
    let state = requests.get(key);
    if (!state) {
      state = {
        running: false,
        pending: false,
        activeTrigger: undefined,
        pendingTrigger: undefined,
        pendingRequest: undefined,
        pendingReason: undefined,
        promise: undefined,
      };
      requests.set(key, state);
    }

    if (state.running) {
      // Socket.IO invokes every React listener with the same message object.
      // Coalesce those duplicate callbacks while preserving one follow-up for
      // a genuinely newer invalidation received during an active request.
      if (!Object.is(state.activeTrigger, trigger) && !Object.is(state.pendingTrigger, trigger)) {
        state.pending = true;
        state.pendingTrigger = trigger;
        state.pendingRequest = request;
        state.pendingReason = reason;
      }
      return state.promise;
    }

    state.pending = true;
    state.pendingTrigger = trigger;
    state.pendingRequest = request;
    state.pendingReason = reason;
    state.running = true;
    state.promise = (async () => {
      try {
        while (state.pending) {
          state.pending = false;
          state.activeTrigger = state.pendingTrigger;
          const nextRequest = state.pendingRequest;
          const nextReason = state.pendingReason;
          state.pendingTrigger = undefined;
          state.pendingRequest = undefined;
          state.pendingReason = undefined;

          try {
            onValue(key, await nextRequest(), nextReason);
          } catch (error) {
            onError(key, error, nextReason);
          }
        }
      } finally {
        state.running = false;
        if (!state.pending && requests.get(key) === state) requests.delete(key);
      }
    })();

    return state.promise;
  };
}
