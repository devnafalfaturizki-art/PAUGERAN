/**
 * SettingsPage — Halaman pengaturan
 * [CB §28] — Kustomisasi Antarmuka
 */

import { Component } from 'solid-js';
import SettingsPanel from '../components/settings/SettingsPanel';
import { usePreferences } from '../hooks/usePreferences';

const SettingsPage: Component = () => {
  const prefs = usePreferences();

  return (
    <div class="settings-page max-w-4xl mx-auto p-6">
      <h1 class="text-3xl font-bold mb-8">Pengaturan</h1>
      <SettingsPanel
        theme={prefs.theme}
        fontSize={prefs.fontSize}
        language={prefs.language}
        highContrast={prefs.highContrast}
        reducedMotion={prefs.reducedMotion}
        screenReaderMode={false}
        webResearchEnabled={true}
        webResearchMaxDepth={3}
        providers={[]}
        knowledgeEntries={[]}
        onThemeChange={prefs.updateTheme}
        onFontSizeChange={prefs.updateFontSize}
        onLanguageChange={(lang: string) => prefs.updateLanguage(lang as 'id' | 'en')}
        onAccessibilityToggle={prefs.toggleAccessibility}
        onWebResearchToggle={() => {}}
        onWebResearchDepthChange={() => {}}
        onAddProvider={() => {}}
        onEditProvider={() => {}}
        onDeleteProvider={() => {}}
        onTestProvider={() => {}}
        onAddKnowledge={() => {}}
        onEditKnowledge={() => {}}
        onDeleteKnowledge={() => {}}
        onSearchKnowledge={() => {}}
        onBackup={() => {}}
        onRestore={() => {}}
        onExport={() => {}}
      />
    </div>
  );
};

export default SettingsPage;
