import { createSignal, onMount } from 'solid-js';
import { loadPreferences, savePreferences } from '../lib/preferences';
import type { Preferences } from '../lib/preferences';

const initial = loadPreferences();

export const [theme, setTheme] = createSignal<Preferences['theme']>(initial.theme);
export const [fontSize, setFontSize] = createSignal<Preferences['fontSize']>(initial.fontSize);
export const [highContrast, setHighContrast] = createSignal(initial.highContrast);
export const [reducedMotion, setReducedMotion] = createSignal(initial.reducedMotion);
export const [language, setLanguage] = createSignal<Preferences['language']>(initial.language);

export function usePreferences() {
  onMount(() => {
    const prefs = loadPreferences();
    setTheme(prefs.theme);
    setFontSize(prefs.fontSize);
    setHighContrast(prefs.highContrast);
    setReducedMotion(prefs.reducedMotion);
    setLanguage(prefs.language);
  });

  const updateTheme = (newTheme: string) => {
    setTheme(newTheme as Preferences['theme']);
    savePreferences({ ...loadPreferences(), theme: newTheme as Preferences['theme'] });
  };

  const updateFontSize = (size: number) => {
    setFontSize(size);
    savePreferences({ ...loadPreferences(), fontSize: size });
  };

  const updateLanguage = (lang: 'id' | 'en') => {
    setLanguage(lang);
    savePreferences({ ...loadPreferences(), language: lang });
  };

  const toggleAccessibility = (setting: string, value: boolean) => {
    const current = loadPreferences();
    const updated = { ...current, [setting]: value } as Preferences;
    savePreferences(updated);
    
    switch (setting) {
      case 'highContrast':
        setHighContrast(value);
        break;
      case 'reducedMotion':
        setReducedMotion(value);
        break;
      case 'screenReaderMode':
        // Handle screen reader mode
        break;
    }
  };

  return {
    get theme() { return theme(); },
    get fontSize() { return fontSize(); },
    get language() { return language(); },
    get highContrast() { return highContrast(); },
    get reducedMotion() { return reducedMotion(); },
    updateTheme,
    updateFontSize,
    updateLanguage,
    toggleAccessibility,
  };
}
