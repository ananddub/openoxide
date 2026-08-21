import { client } from "#/api/client";
import type { components } from "#/types/api.d.ts";

type CreateCertificate = components["schemas"]["CreateCertificateDto"];
type PatchCertificate = components["schemas"]["PatchCertificateDto"];
type RenewCertificate = components["schemas"]["RenewCertificateDto"];

function ensure<T>(result: { data?: T; error?: unknown }) {
  if (result.error) throw new Error(String(result.error));
  return result.data as T;
}

export async function listCertificates() {
  return ensure(await client.GET("/certificates"));
}

export async function listRemoteServers() {
  return ensure(await client.GET("/remote-servers"));
}

export async function createCertificate(body: CreateCertificate) {
  return ensure(await client.POST("/certificates", { body }));
}

export async function updateCertificate(id: number, body: PatchCertificate) {
  return ensure(
    await client.PATCH("/certificates/{id}", {
      params: { path: { id } },
      body,
    }),
  );
}

export async function deleteCertificate(id: number) {
  return ensure(
    await client.DELETE("/certificates/{id}", { params: { path: { id } } }),
  );
}

export async function listCertificateRenewals(id: number) {
  return ensure(
    await client.GET("/certificates/{id}/renewals", {
      params: { path: { id } },
    }),
  );
}

export async function renewCertificate(id: number, body: RenewCertificate) {
  return ensure(
    await client.POST("/certificates/{id}/renew", {
      params: { path: { id } },
      body,
    }),
  );
}
