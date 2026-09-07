import { createSignal } from 'solid-js';
import type { Preferences } from '../lib/preferences';
import { loadPreferences, savePreferences } from '../lib/preferences';

const initial = loadPreferences();

export const [preferences, setPreferences] = createSignal<Preferences>(initial);

export const preferenceStore = {
  get preferences() {
    return preferences();
  },
  update: (updates: Partial<Preferences>) => {
    const updated = { ...preferences(), ...updates };
    setPreferences(updated);
    savePreferences(updated);
  },
};
