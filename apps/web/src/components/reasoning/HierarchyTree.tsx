/**
 * HierarchyTree — Visualisasi hierarki hukum
 * [CB §31] — Legal Hierarchy Display
 */

import { Component } from 'solid-js';
import { LEGAL_HIERARCHY_LEVELS } from '@paugeran/shared';

interface HierarchyTreeProps {
  currentLevel: number;
  regulations: Array<{
    name: string;
    level: number;
    children?: Array<{ name: string; level: number }>;
  }>;
}

const HierarchyTree: Component<HierarchyTreeProps> = (props) => {
  const getLevelColor = (level: number) => {
    const colors = [
      'bg-red-500',
      'bg-orange-500',
      'bg-yellow-500',
      'bg-green-500',
      'bg-blue-500',
      'bg-indigo-500',
      'bg-purple-500',
    ];
    return colors[Math.min(level - 1, colors.length - 1)];
  };

  return (
    <div class="hierarchy-tree">
      <h4 class="text-sm font-semibold mb-2">Hierarki Norma</h4>
      <div class="space-y-2">
        {props.regulations.map((reg) => {
          const levelName = LEGAL_HIERARCHY_LEVELS[reg.level - 1] || `Level ${reg.level}`;
          const color = getLevelColor(reg.level);
          return (
            <div>
              <div class="flex items-center gap-2">
                <span class={['w-3 h-3 rounded-full', color].join(' ')} />
                <span class="text-sm">{reg.name}</span>
                <span class="text-xs text-gray-500">
                  [{reg.level}] {levelName}
                </span>
              </div>
              {reg.children && reg.children.map((child) => {
                const childColor = getLevelColor(child.level);
                return (
                  <div class="flex items-center gap-2 ml-6">
                    <span class={['w-2 h-2 rounded-full', childColor].join(' ')} />
                    <span class="text-xs text-gray-600">{child.name}</span>
                  </div>
                );
              })}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default HierarchyTree;
