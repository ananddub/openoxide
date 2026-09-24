// Response/mutation payload types. The wire payloads carry the backend DTO's snake_case fields, so these are structural definitions matching the JSON the live hooks parse. Keep field names in sync with backend DTOs.

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
