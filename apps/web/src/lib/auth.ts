/**
 * Authentication utilities.
 * [CB §27] — Optional multi-user authentication
 */

const TOKEN_KEY = 'paugeran_auth_token';

export interface AuthState {
  isAuthenticated: boolean;
  token: string | null;
  user: {
    id: string;
    email: string;
    name: string;
    role: 'admin' | 'user';
  } | null;
}

export const defaultAuthState: AuthState = {
  isAuthenticated: false,
  token: null,
  user: null,
};

export function loadAuthState(): AuthState {
  try {
    const token = localStorage.getItem(TOKEN_KEY);
    if (token) {
      const user = JSON.parse(localStorage.getItem('paugeran_user') || 'null');
      return { isAuthenticated: true, token, user };
    }
  } catch {
    // ignore parse errors
  }
  return defaultAuthState;
}

export function saveAuthState(state: AuthState): void {
  if (state.token) {
    localStorage.setItem(TOKEN_KEY, state.token);
    if (state.user) {
      localStorage.setItem('paugeran_user', JSON.stringify(state.user));
    }
  } else {
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem('paugeran_user');
  }
}

export function clearAuthState(): void {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem('paugeran_user');
}
