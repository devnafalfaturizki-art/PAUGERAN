export type ThemeMode = 'light' | 'dark' | 'sepia' | 'high-contrast';

export interface Preferences {
  theme: ThemeMode;
  fontSize: number;
  language: 'id' | 'en';
  highContrast: boolean;
  reducedMotion: boolean;
  screenReaderMode: boolean;
  webResearchEnabled: boolean;
  webResearchMaxDepth: number;
}

export const defaultPreferences: Preferences = {
  theme: 'light',
  fontSize: 14,
  language: 'id',
  highContrast: false,
  reducedMotion: false,
  screenReaderMode: false,
  webResearchEnabled: true,
  webResearchMaxDepth: 3,
};

export function loadPreferences(): Preferences {
  try {
    const stored = localStorage.getItem('paugeran_preferences');
    if (stored) {
      return { ...defaultPreferences, ...JSON.parse(stored) };
    }
  } catch {
    // ignore parse errors
  }
  return defaultPreferences;
}

export function savePreferences(prefs: Preferences): void {
  localStorage.setItem('paugeran_preferences', JSON.stringify(prefs));
}
