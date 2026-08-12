/**
 * Shared n8n-style card shell for all node types.
 * Handles common hover/focus styling, handle visibility.
 */

export const CARD_W = 220;

// Glass card base style
export const cardBase: React.CSSProperties = {
	width: CARD_W,
	background: '#1a1a2e',
	border: '1px solid rgba(255,255,255,0.08)',
	borderRadius: 12,
	cursor: 'pointer',
	userSelect: 'none',
	transition: 'border-color 0.15s, box-shadow 0.15s',
	fontFamily: 'inherit',
};

export const cardHover: React.CSSProperties = {
	border: '1px solid rgba(99,102,241,0.45)',
	boxShadow: '0 0 0 3px rgba(99,102,241,0.08)',
};
