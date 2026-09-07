/**
 * AdminPanel — Panel administrasi utama
 * [CB §27] — Multi-User Authentication & Team Management
 */

import { Component } from 'solid-js';

interface AdminPanelProps {
  activeTab: 'users' | 'invitations' | 'providers' | 'knowledge' | 'system';
  onTabChange: (tab: 'users' | 'invitations' | 'providers' | 'knowledge' | 'system') => void;
}

const AdminPanel: Component<AdminPanelProps> = (props) => {
  const tabs = [
    { id: 'users' as const, label: 'Pengguna', icon: '👥' },
    { id: 'invitations' as const, label: 'Undangan', icon: '✉️' },
    { id: 'providers' as const, label: 'Penyedia LLM', icon: '🤖' },
    { id: 'knowledge' as const, label: 'Basis Pengetahuan', icon: '📚' },
    { id: 'system' as const, label: 'Sistem', icon: '⚙️' },
  ];

  return (
    <div class="admin-panel">
      <h2 class="text-2xl font-bold mb-6">Panel Administrasi</h2>
      
      <div class="flex gap-2 mb-6 overflow-x-auto pb-2">
        {tabs.map((tab) => (
          <button
            class={`px-4 py-2 rounded-lg whitespace-nowrap transition-colors ${
              props.activeTab === tab.id
                ? 'bg-primary-500 text-white'
                : 'bg-gray-200 dark:bg-gray-700 hover:bg-gray-300'
            }`}
            onClick={() => props.onTabChange(tab.id)}
          >
            <span class="mr-2">{tab.icon}</span>
            {tab.label}
          </button>
        ))}
      </div>

      <div class="admin-content">
        {/* Content will be rendered by parent based on activeTab */}
      </div>
    </div>
  );
};

export default AdminPanel;
