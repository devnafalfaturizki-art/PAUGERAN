import { createSignal } from 'solid-js';
import type { AuthState } from '../lib/auth';
import { loadAuthState } from '../lib/auth';

const initial = loadAuthState();

export const [auth, setAuth] = createSignal<AuthState>(initial);

export const authStore = {
  get auth() {
    return auth();
  },
  setAuth,
  login: (token: string, user: AuthState['user']) => {
    const state: AuthState = { isAuthenticated: true, token, user };
    setAuth(state);
  },
  logout: () => {
    setAuth({ isAuthenticated: false, token: null, user: null });
  },
};
