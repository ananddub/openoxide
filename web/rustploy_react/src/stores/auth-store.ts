import {create} from 'zustand';
import type {User} from '#/types/user';

type AuthState = {
	user: User | null;
	isAuth: boolean;
	setAuth: (user: User) => void;
	logout: () => void;
};

export const useAuthStore = create<AuthState>(set => ({
	user: null,
	isAuth: false,
	// Methods
	setAuth: user => set({user, isAuth: true}),
	logout: () => set({user: null, isAuth: false}),
}));
