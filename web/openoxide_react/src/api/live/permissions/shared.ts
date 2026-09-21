import {createServiceClient} from '../query';
import {PermissionService} from '#/gen/connect/openoxide/api/v1/permission_pb';

export const permissionClient = createServiceClient(PermissionService);
