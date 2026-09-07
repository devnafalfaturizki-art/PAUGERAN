/**
 * InvitationManager — Manajemen undangan tim
 * [CB §27] — Team invitation system
 */

import { Component, createSignal } from 'solid-js';

interface InvitationManagerProps {
  invitations: Array<{
    id: string;
    email: string;
    role: 'admin' | 'user';
    createdAt: string;
  }>;
  onInvite: (email: string, role: 'admin' | 'user') => void;
  onCancel: (id: string) => void;
}

const InvitationManager: Component<InvitationManagerProps> = (props) => {
  const [email, setEmail] = createSignal('');
  const [role, setRole] = createSignal<'admin' | 'user'>('user');

  const handleInvite = () => {
    if (email().trim()) {
      props.onInvite(email().trim(), role());
      setEmail('');
    }
  };

  return (
    <div class="invitation-manager">
      <h3 class="text-lg font-semibold mb-4">Undangan Tim</h3>

      <div class="flex gap-3 mb-6">
        <input
          type="email"
          placeholder="Email pengguna"
          class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
          value={email()}
          onInput={(e) => setEmail(e.currentTarget.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleInvite()}
        />
        <select
          class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
          value={role()}
          onChange={(e) => setRole(e.currentTarget.value as 'admin' | 'user')}
        >
          <option value="user">User</option>
          <option value="admin">Admin</option>
        </select>
        <button
          class="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          onClick={handleInvite}
        >
          Kirim Undangan
        </button>
      </div>

      {props.invitations.length === 0 ? (
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          Belum ada undangan yang dikirim.
        </div>
      ) : (
        <div class="space-y-3">
          {props.invitations.map((invitation) => (
            <div class="flex items-center justify-between border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div>
                <div class="font-medium">{invitation.email}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  Role: {invitation.role === 'admin' ? 'Admin' : 'User'} |{' '}
                  {new Date(invitation.createdAt).toLocaleDateString('id-ID')}
                </div>
              </div>
              <button
                class="px-3 py-1 text-sm border border-red-300 text-red-600 rounded hover:bg-red-50"
                onClick={() => props.onCancel(invitation.id)}
              >
                Batalkan
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default InvitationManager;
