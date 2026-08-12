export interface TraefikFileNode {
	name: string;
	relative_path: string;
	size: number;
	is_readonly: boolean;
	modified_at: number;
}

export interface TraefikFileTreeNode {
	name: string;
	relative_path: string;
	node_type: 'file' | 'directory';
	size: number;
	is_readonly: boolean;
	modified_at: number;
	children: TraefikFileTreeNode[];
}

export interface TraefikFileContent {
	path: string;
	content: string;
	is_readonly: boolean;
}

export interface TraefikHealthResponse {
	is_healthy: boolean;
	rawdata_status: string;
	configuration_errors: string[];
}

export interface RemoteServerItem {
	id: number;
	name: string;
	ip_address?: string;
}
