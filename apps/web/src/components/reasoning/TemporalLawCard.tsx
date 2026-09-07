/**
 * TemporalLawCard — Kartu penerapan hukum temporal
 * [CB §9] — Temporal Law Application System
 */

import { Component } from 'solid-js';

interface TemporalLawCardProps {
  eventDate: string;
  applicableRegulations: Array<{
    name: string;
    status: 'active' | 'revoked' | 'amended';
    effectiveDate: string;
  }>;
}

const TemporalLawCard: Component<TemporalLawCardProps> = (props) => {
  return (
    <div class="temporal-law-card border border-blue-200 dark:border-blue-700 bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
      <div class="flex items-center gap-2 mb-2">
        <span class="text-blue-600 dark:text-blue-400">📅</span>
        <h4 class="font-semibold">Lex Temporis — Hukum yang Berlaku</h4>
      </div>

      <div class="text-sm mb-3">
        <strong>Tanggal Kejadian:</strong> {new Date(props.eventDate).toLocaleDateString('id-ID')}
      </div>

      <div class="space-y-2">
        {props.applicableRegulations.map((reg) => (
          <div class="flex items-center justify-between p-2 bg-white dark:bg-gray-800 rounded">
            <div>
              <div class="font-medium">{reg.name}</div>
              <div class="text-xs text-gray-500">
                Berlaku sejak: {new Date(reg.effectiveDate).toLocaleDateString('id-ID')}
              </div>
            </div>
            <span
              class={`px-2 py-1 rounded text-xs ${
                reg.status === 'active'
                  ? 'bg-green-100 text-green-800'
                  : reg.status === 'revoked'
                  ? 'bg-red-100 text-red-800'
                  : 'bg-yellow-100 text-yellow-800'
              }`}
            >
              {reg.status === 'active' ? 'Aktif' : reg.status === 'revoked' ? 'Dicabut' : 'Diubah'}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default TemporalLawCard;
