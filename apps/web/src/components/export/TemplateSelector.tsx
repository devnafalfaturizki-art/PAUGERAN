/**
 * TemplateSelector — Pemilihan template ekspor
 * [CB §34] — Export Dokumen Profesional
 */

import { Component } from 'solid-js';
import type { ExportTemplate } from '@paugeran/shared';

interface TemplateSelectorProps {
  selectedTemplate: ExportTemplate;
  onTemplateChange: (template: ExportTemplate) => void;
}

const TemplateSelector: Component<TemplateSelectorProps> = (props) => {
  const templates = [
    { id: 'exploration' as ExportTemplate, label: 'Exploration', description: 'Legal Possibility Assessment' },
    { id: 'preventive' as ExportTemplate, label: 'Preventive', description: 'Legal Risk & Mitigation Matrix' },
    { id: 'dispute' as ExportTemplate, label: 'Dispute', description: 'Position Mapping & Settlement Strategy' },
    { id: 'litigation' as ExportTemplate, label: 'Litigation', description: 'Case Theory & Evidence Chart' },
    { id: 'adversarial' as ExportTemplate, label: 'Adversarial', description: 'Pre-Mortem & Vulnerability Report' },
    { id: 'neutral' as ExportTemplate, label: 'Neutral', description: 'Judicial Simulation Report' },
  ];

  return (
    <div class="template-selector">
      <h3 class="text-lg font-semibold mb-3">Template Dokumen</h3>
      <div class="space-y-2">
        {templates.map((template) => (
          <button
            class={`w-full p-3 border-2 rounded-lg text-left transition-colors ${
              props.selectedTemplate === template.id
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300'
            }`}
            onClick={() => props.onTemplateChange(template.id)}
          >
            <div class="font-medium">{template.label}</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">{template.description}</div>
          </button>
        ))}
      </div>
    </div>
  );
};

export default TemplateSelector;
