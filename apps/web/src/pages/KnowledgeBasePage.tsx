/**
 * KnowledgeBasePage — Halaman basis pengetahuan hukum
 * [CB §24] — Supreme Legal Knowledge Base
 */

import { Component } from 'solid-js';
import KnowledgeBaseList from '../components/knowledge/KnowledgeBaseList';

const KnowledgeBasePage: Component = () => {
  return (
    <div class="knowledge-base-page max-w-4xl mx-auto p-6">
      <h1 class="text-3xl font-bold mb-8">Basis Pengetahuan Hukum</h1>
      <KnowledgeBaseList
        entries={[]}
        onAdd={() => {}}
        onEdit={() => {}}
        onDelete={() => {}}
        onSearch={() => {}}
        onImport={() => {}}
      />
    </div>
  );
};

export default KnowledgeBasePage;
