/**
 * KnowledgeBaseImport — Import dokumen ke basis pengetahuan
 * [CB §24] — Supreme Legal Knowledge Base
 */

import { Component, createSignal } from 'solid-js';

interface KnowledgeBaseImportProps {
  onImport: (file: File) => void;
  onCancel: () => void;
}

const KnowledgeBaseImport: Component<KnowledgeBaseImportProps> = (props) => {
  const [isDragging, setIsDragging] = createSignal(false);

  const handleDrop = (e: DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    const file = e.dataTransfer?.files[0];
    if (file) {
      props.onImport(file);
    }
  };

  return (
    <div class="knowledge-base-import">
      <h3 class="text-lg font-semibold mb-4">Import Dokumen</h3>

      <div
        class={`border-2 border-dashed rounded-lg p-8 text-center transition-colors ${
          isDragging()
            ? 'border-primary-500 bg-primary-50'
            : 'border-gray-300 dark:border-gray-600'
        }`}
        onDragOver={(e) => {
          e.preventDefault();
          setIsDragging(true);
        }}
        onDragLeave={() => setIsDragging(false)}
        onDrop={handleDrop}
      >
        <div class="text-4xl mb-2">📄</div>
        <div class="font-medium mb-1">Seret dokumen ke sini</div>
        <div class="text-sm text-gray-500 dark:text-gray-400 mb-3">
          atau klik untuk memilih file
        </div>
        <input
          type="file"
          accept=".pdf,.docx,.txt"
          class="hidden"
          id="file-upload"
          onChange={(e) => {
            const file = e.currentTarget.files?.[0];
            if (file) props.onImport(file);
          }}
        />
        <label
          for="file-upload"
          class="inline-block px-4 py-2 bg-primary-500 text-white rounded-lg cursor-pointer hover:bg-primary-600 transition-colors"
        >
          Pilih File
        </label>
        <div class="text-xs text-gray-400 mt-3">
          Format yang didukung: PDF, DOCX, TXT
        </div>
      </div>

      <div class="mt-4 flex justify-end">
        <button
          class="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
          onClick={props.onCancel}
        >
          Batal
        </button>
      </div>
    </div>
  );
};

export default KnowledgeBaseImport;
