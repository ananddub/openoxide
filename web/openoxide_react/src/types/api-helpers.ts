// Response/mutation payload types. These were previously derived from the
// frozen OpenAPI schema (types/api.d.ts); since the ConnectRPC migration the
// wire payloads still carry the same snake_case fields, so the types now
// stand alone as structural definitions. Keep field names in sync with the
// backend DTO JSON the live hooks parse.

type Payload = Record<string, any>;

export type ApplicationResponse = Payload;
export type ComposeResponse = Payload;
export type DatabaseResponse = Payload;
export type DeploymentResponse = Payload;
export type EnvironmentResponse = Payload;
export type ProjectResponse = Payload;
export type RemoteServerResponse = Payload;
export type SshKeyResponse = Payload;
export type ScheduleResponse = Payload;
export type DomainResponse = Payload;
export type DestinationResponse = Payload;
export type RegistryResponse = Payload;
export type SwarmNode = Payload;
export type SwarmInfo = Payload;
export type SwarmTokens = Payload;

export type PatchApplication = Payload;
export type PatchCompose = Payload;
