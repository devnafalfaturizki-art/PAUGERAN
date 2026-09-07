import { createSignal } from 'solid-js';
import type { ReasoningMode } from '@paugeran/shared';

export const [mode, setMode] = createSignal<ReasoningMode>('exploration');

export const modeStore = {
  get mode() {
    return mode();
  },
  setMode,
};
