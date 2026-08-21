import { client } from "#/api/client";
import type { components } from "#/types/api.d.ts";

type CreateAiSetting = components["schemas"]["CreateAiSettingDto"];
type UpdateAiSetting = components["schemas"]["UpdateAiSettingDto"];

function ensure<T>(result: { data?: T; error?: unknown }) {
  if (result.error) throw new Error(String(result.error));
  return result.data as T;
}

export async function listAiSettings() {
  return ensure(await client.GET("/api/ai/settings"));
}

export async function createAiSetting(body: CreateAiSetting) {
  return ensure(await client.POST("/api/ai/settings", { body }));
}

export async function updateAiSetting(id: number, body: UpdateAiSetting) {
  return ensure(
    await client.PUT("/api/ai/settings/{id}", {
      params: { path: { id } },
      body,
    }),
  );
}

export async function deleteAiSetting(id: number) {
  return ensure(
    await client.DELETE("/api/ai/settings/{id}", { params: { path: { id } } }),
  );
}

export async function discoverAiModels(body: {
  api_url: string;
  api_key: string;
}) {
  return ensure(await client.POST("/api/ai/models/discover", { body }));
}

export async function discoverSettingModels(id: number) {
  return ensure(
    await client.GET("/api/ai/settings/{id}/models", {
      params: { path: { id } },
    }),
  );
}

export async function testAiConnection(body: {
  api_url: string;
  api_key: string;
  model: string;
}) {
  return ensure(await client.POST("/api/ai/connection/test", { body }));
}

export async function testAiSetting(id: number) {
  return ensure(
    await client.POST("/api/ai/settings/{id}/test", {
      params: { path: { id } },
    }),
  );
}
