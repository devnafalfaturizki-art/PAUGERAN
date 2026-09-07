/**
 * NormConflictCard — Kartu konflik norma
 * [CB §8] — Normative Conflict Resolution Engine
 */

import { Component } from 'solid-js';
import type { NormConflict } from '@paugeran/shared';

interface NormConflictCardProps {
  conflict: NormConflict;
}

const NormConflictCard: Component<NormConflictCardProps> = (props) => {
  return (
    <div class="norm-conflict-card border border-yellow-200 dark:border-yellow-700 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg p-4">
      <div class="flex items-center gap-2 mb-2">
        <span class="text-yellow-600 dark:text-yellow-400">⚠️</span>
        <h4 class="font-semibold">Konflik Norma Terdeteksi</h4>
      </div>

      <div class="space-y-2 text-sm">
        <div>
          <strong>Peraturan A:</strong> {props.conflict.provisionA}
        </div>
        <div>
          <strong>Peraturan B:</strong> {props.conflict.provisionB}
        </div>
        <div>
          <strong>Jenis Konflik:</strong>{' '}
          <span class="capitalize">{props.conflict.conflictType.replace(/_/g, ' ')}</span>
        </div>
        <div>
          <strong>Resolusi:</strong> {props.conflict.resolution}
        </div>
        <div class="mt-2 p-2 bg-white dark:bg-gray-800 rounded">
          <strong>Peraturan yang Berlaku:</strong> {props.conflict.applicableProvision}
        </div>
      </div>
    </div>
  );
};

export default NormConflictCard;
