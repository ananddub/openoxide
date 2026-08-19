import {createContext, useContext} from 'react';

export interface CanvasActions {
	inspect: (node: {
		id: number;
		type: string;
		name: string;
		status?: string;
		dbType?: string;
	}) => void;
	toggleExpand: (composeId: number) => void;
}

export const CanvasActionsContext = createContext<CanvasActions>({
	inspect: () => {},
	toggleExpand: () => {},
});

export const useCanvasActions = () => useContext(CanvasActionsContext);
