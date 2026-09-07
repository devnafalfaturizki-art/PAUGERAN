/**
 * SettingsPanel — Panel pengaturan utama
 * [CB §28] — Kustomisasi Antarmuka
 */

import { Component } from 'solid-js';
import ThemeSelector from './ThemeSelector';
import FontSelector from './FontSelector';
import AccessibilitySettings from './AccessibilitySettings';
import LanguageSelector from './LanguageSelector';
import WebResearchSettings from './WebResearchSettings';
import DataManager from './DataManager';
import LLMProviderManager from './LLMProviderManager';
import KnowledgeBaseManager from './KnowledgeBaseManager';

interface SettingsPanelProps {
  theme: string;
  fontSize: number;
  language: string;
  highContrast: boolean;
  reducedMotion: boolean;
  screenReaderMode: boolean;
  webResearchEnabled: boolean;
  webResearchMaxDepth: number;
  providers: Array<{ id: string; name: string; providerType: string; model: string; isGlobal?: boolean }>;
  knowledgeEntries: Array<{ id: string; title: string; tags: string[] }>;
  onThemeChange: (theme: string) => void;
  onFontSizeChange: (size: number) => void;
  onLanguageChange: (language: string) => void;
  onAccessibilityToggle: (setting: string, value: boolean) => void;
  onWebResearchToggle: (enabled: boolean) => void;
  onWebResearchDepthChange: (depth: number) => void;
  onAddProvider: () => void;
  onEditProvider: (id: string) => void;
  onDeleteProvider: (id: string) => void;
  onTestProvider: (id: string) => void;
  onAddKnowledge: () => void;
  onEditKnowledge: (id: string) => void;
  onDeleteKnowledge: (id: string) => void;
  onSearchKnowledge: (query: string) => void;
  onBackup: () => void;
  onRestore: () => void;
  onExport: () => void;
}

const SettingsPanel: Component<SettingsPanelProps> = (props) => {
  return (
    <div class="settings-panel space-y-8">
      <ThemeSelector currentTheme={props.theme} onThemeChange={props.onThemeChange} />
      <FontSelector fontSize={props.fontSize} onFontSizeChange={props.onFontSizeChange} />
      <LanguageSelector language={props.language} onLanguageChange={props.onLanguageChange} />
      <AccessibilitySettings
        highContrast={props.highContrast}
        reducedMotion={props.reducedMotion}
        screenReaderMode={props.screenReaderMode}
        onToggle={props.onAccessibilityToggle}
      />
      <WebResearchSettings
        enabled={props.webResearchEnabled}
        maxDepth={props.webResearchMaxDepth}
        onToggle={props.onWebResearchToggle}
        onMaxDepthChange={props.onWebResearchDepthChange}
      />
      <LLMProviderManager
        providers={props.providers}
        onAdd={props.onAddProvider}
        onEdit={props.onEditProvider}
        onDelete={props.onDeleteProvider}
        onTest={props.onTestProvider}
      />
      <KnowledgeBaseManager
        entries={props.knowledgeEntries}
        onAdd={props.onAddKnowledge}
        onEdit={props.onEditKnowledge}
        onDelete={props.onDeleteKnowledge}
        onSearch={props.onSearchKnowledge}
      />
      <DataManager
        onExport={props.onExport}
        onRestore={props.onRestore}
        onBackup={props.onBackup}
        onImport={() => {}}
      />
    </div>
  );
};

export default SettingsPanel;
