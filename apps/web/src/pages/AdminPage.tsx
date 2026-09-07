/**
 * AdminPage — Halaman administrasi tim
 * [CB §27] — Multi-User Authentication
 */

import { Component, createSignal } from 'solid-js';
import AdminPanel from '../components/admin/AdminPanel';
import UserManagement from '../components/admin/UserManagement';
import InvitationManager from '../components/admin/InvitationManager';
import GlobalProviderManager from '../components/admin/GlobalProviderManager';
import GlobalKnowledgeBase from '../components/admin/GlobalKnowledgeBase';
import SystemConfig from '../components/admin/SystemConfig';

type AdminTab = 'users' | 'invitations' | 'providers' | 'knowledge' | 'system';

const AdminPage: Component = () => {
  const [activeTab, setActiveTab] = createSignal<AdminTab>('users');
  const [users, setUsers] = createSignal<Array<{ id: string; email: string; name: string; role: 'admin' | 'user'; createdAt: string }>>([]);
  const [invitations, setInvitations] = createSignal<Array<{ id: string; email: string; role: 'admin' | 'user'; createdAt: string }>>([]);
  const [providers, setProviders] = createSignal<Array<{ id: string; name: string; providerType: string; model: string }>>([]);
  const [knowledgeEntries, setKnowledgeEntries] = createSignal<Array<{ id: string; title: string; tags: string[] }>>([]);

  return (
    <div class="admin-page">
      <AdminPanel activeTab={activeTab()} onTabChange={setActiveTab} />
      
      <div class="mt-6">
        {activeTab() === 'users' && (
          <UserManagement
            users={users()}
            onAddUser={() => {}}
            onRemoveUser={(id: string) => setUsers(users().filter((u) => u.id !== id))}
            onUpdateRole={(id: string, role: 'admin' | 'user') => setUsers(users().map((u) => u.id === id ? { ...u, role } : u))}
          />
        )}
        
        {activeTab() === 'invitations' && (
          <InvitationManager
            invitations={invitations()}
            onInvite={(email, role) => {
              const newInvitation = {
                id: crypto.randomUUID(),
                email,
                role,
                createdAt: new Date().toISOString(),
              };
              setInvitations([...invitations(), newInvitation]);
            }}
            onCancel={(id) => setInvitations(invitations().filter((i) => i.id !== id))}
          />
        )}
        
        {activeTab() === 'providers' && (
          <GlobalProviderManager
            providers={providers()}
            onAdd={() => {}}
            onEdit={(_id: string) => {}}
            onDelete={(id) => setProviders(providers().filter((p) => p.id !== id))}
          />
        )}
        
        {activeTab() === 'knowledge' && (
          <GlobalKnowledgeBase
            entries={knowledgeEntries()}
            onAdd={() => {}}
            onEdit={(_id: string) => {}}
            onDelete={(id) => setKnowledgeEntries(knowledgeEntries().filter((e) => e.id !== id))}
          />
        )}
        
        {activeTab() === 'system' && (
          <SystemConfig
            config={{
              appName: 'PAUGERAN',
              version: '1.0.0',
              authEnabled: false,
              maxCasesPerUser: 100,
              maxDocumentSize: 10,
            }}
            onSave={(config) => console.log('Save config:', config)}
          />
        )}
      </div>
    </div>
  );
};

export default AdminPage;
