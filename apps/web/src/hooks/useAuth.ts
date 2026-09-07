import { createSignal, onMount } from 'solid-js';
import { loadAuthState, saveAuthState, clearAuthState, type AuthState } from '../lib/auth';

export function useAuth() {
  const [auth, setAuth] = createSignal<AuthState>(loadAuthState());

  onMount(() => {
    setAuth(loadAuthState());
  });

  const login = async (email: string, password: string) => {
    const response = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password }),
    });
    
    if (!response.ok) {
      throw new Error('Login failed');
    }

    const data = await response.json();
    const newAuth: AuthState = {
      isAuthenticated: true,
      token: data.token,
      user: data.user,
    };
    setAuth(newAuth);
    saveAuthState(newAuth);
  };

  const logout = () => {
    setAuth({ isAuthenticated: false, token: null, user: null });
    clearAuthState();
  };

  return {
    get auth() {
      return auth();
    },
    login,
    logout,
    isAuthenticated: () => auth().isAuthenticated,
  };
}
