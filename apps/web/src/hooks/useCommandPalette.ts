import { createSignal } from 'solid-js';

export function useCommandPalette() {
  const [isOpen, setIsOpen] = createSignal(false);
  const [query, setQuery] = createSignal('');

  const open = () => setIsOpen(true);
  const close = () => {
    setIsOpen(false);
    setQuery('');
  };
  const toggle = () => setIsOpen(!isOpen());

  return {
    get isOpen() { return isOpen(); },
    get query() { return query(); },
    setQuery,
    open,
    close,
    toggle,
  };
}
