import assert from "node:assert/strict";
import test from "node:test";

import { createLiveRequestQueue } from "./live-request.js";

function deferred() {
  let resolve;
  const promise = new Promise((done) => {
    resolve = done;
  });
  return { promise, resolve };
}

test("coalesces duplicate consumers for the same trigger", async () => {
  const gate = deferred();
  const values = [];
  let requests = 0;
  const queue = createLiveRequestQueue({
    onValue: (_key, value) => values.push(value),
    onError: (_key, error) => {
      throw error;
    },
  });
  const trigger = {};
  const request = async () => {
    requests++;
    await gate.promise;
    return "ready";
  };

  const first = queue("compose:2", request, trigger, "initial");
  const second = queue("compose:2", request, trigger, "initial");
  gate.resolve();
  await Promise.all([first, second]);

  assert.equal(requests, 1);
  assert.deepEqual(values, ["ready"]);
});

test("runs one catch-up request for a newer invalidation", async () => {
  const gate = deferred();
  const values = [];
  let requests = 0;
  const queue = createLiveRequestQueue({
    onValue: (_key, value) => values.push(value),
    onError: (_key, error) => {
      throw error;
    },
  });
  const request = async () => {
    requests++;
    if (requests === 1) await gate.promise;
    return requests;
  };
  const initial = {};
  const changed = {};

  const running = queue("deployments", request, initial, "initial");
  queue("deployments", request, changed, "invalidation");
  queue("deployments", request, changed, "invalidation");
  gate.resolve();
  await running;

  assert.equal(requests, 2);
  assert.deepEqual(values, [1, 2]);
});

test("allows retry after a failed request", async () => {
  const errors = [];
  const values = [];
  const queue = createLiveRequestQueue({
    onValue: (_key, value) => values.push(value),
    onError: (_key, error) => errors.push(error.message),
  });

  await queue(
    "app:3",
    async () => {
      throw new Error("offline");
    },
    "initial",
    "initial",
  );
  await queue("app:3", async () => "recovered", "retry", "reconnect");

  assert.deepEqual(errors, ["offline"]);
  assert.deepEqual(values, ["recovered"]);
});
