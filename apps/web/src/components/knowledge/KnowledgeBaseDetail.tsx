/**
 * KnowledgeBaseDetail — Detail entri basis pengetahuan
 * [CB §24] — Supreme Legal Knowledge Base
 */

import { Component } from 'solid-js';

interface KnowledgeBaseDetailProps {
  entry: {
    id: string;
    title: string;
    fullText: string;
    hierarchyLevel: number;
    tags: string[];
    sourceUrl?: string;
  };
  onEdit: () => void;
  onDelete: () => void;
}

const KnowledgeBaseDetail: Component<KnowledgeBaseDetailProps> = (props) => {
  return (
    <div class="knowledge-base-detail">
      <div class="flex items-start justify-between mb-4">
        <div>
          <h3 class="text-xl font-semibold">{props.entry.title}</h3>
          <div class="text-sm text-gray-500 dark:text-gray-400 mt-1">
            Level Hierarki: {props.entry.hierarchyLevel}
          </div>
        </div>
        <div class="flex gap-2">
          <button
            class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50"
            onClick={props.onEdit}
          >
            Edit
          </button>
          <button
            class="px-3 py-1 text-sm border border-red-300 text-red-600 rounded hover:bg-red-50"
            onClick={props.onDelete}
          >
            Hapus
          </button>
        </div>
      </div>

      <div class="mb-4">
        <div class="text-sm font-medium mb-2">Tag</div>
        <div class="flex flex-wrap gap-2">
          {props.entry.tags.map((tag) => (
            <span class="px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded text-xs">
              {tag}
            </span>
          ))}
        </div>
      </div>

      <div>
        <div class="text-sm font-medium mb-2">Teks Lengkap</div>
        <div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg whitespace-pre-wrap font-mono text-sm">
          {props.entry.fullText}
        </div>
      </div>

      {props.entry.sourceUrl && (
        <div class="mt-4">
          <a
            href={props.entry.sourceUrl}
            target="_blank"
            rel="noopener noreferrer"
            class="text-primary-500 hover:text-primary-600 text-sm"
          >
            Lihat Sumber Asli →
          </a>
        </div>
      )}
    </div>
  );
};

export default KnowledgeBaseDetail;
