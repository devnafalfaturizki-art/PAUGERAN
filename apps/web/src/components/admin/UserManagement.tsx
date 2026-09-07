/**
 * UserManagement — Manajemen pengguna tim
 * [CB §27] — Multi-User Authentication
 */

import { Component } from 'solid-js';

interface User {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'user';
  createdAt: string;
}

interface UserManagementProps {
  users: User[];
  onAddUser: () => void;
  onRemoveUser: (id: string) => void;
  onUpdateRole: (id: string, role: 'admin' | 'user') => void;
}

const UserManagement: Component<UserManagementProps> = (props) => {
  return (
    <div class="user-management">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold">Manajemen Pengguna</h3>
        <button
          class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          onClick={props.onAddUser}
        >
          Tambah Pengguna
        </button>
      </div>

      {props.users.length === 0 ? (
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          Belum ada pengguna dalam tim.
        </div>
      ) : (
        <div class="overflow-x-auto">
          <table class="w-full border-collapse">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-700">
                <th class="text-left py-2 px-4">Nama</th>
                <th class="text-left py-2 px-4">Email</th>
                <th class="text-left py-2 px-4">Role</th>
                <th class="text-left py-2 px-4">Dibuat</th>
                <th class="text-right py-2 px-4">Aksi</th>
              </tr>
            </thead>
            <tbody>
              {props.users.map((user) => (
                <tr class="border-b border-gray-100 dark:border-gray-800">
                  <td class="py-3 px-4">{user.name}</td>
                  <td class="py-3 px-4">{user.email}</td>
                  <td class="py-3 px-4">
                    <span
                      class={`px-2 py-1 rounded text-xs ${
                        user.role === 'admin'
                          ? 'bg-purple-100 text-purple-800'
                          : 'bg-gray-100 text-gray-800'
                      }`}
                    >
                      {user.role === 'admin' ? 'Admin' : 'User'}
                    </span>
                  </td>
                  <td class="py-3 px-4 text-sm text-gray-500">
                    {new Date(user.createdAt).toLocaleDateString('id-ID')}
                  </td>
                  <td class="py-3 px-4 text-right">
                    <button
                      class="text-red-600 hover:text-red-800 text-sm"
                      onClick={() => props.onRemoveUser(user.id)}
                    >
                      Hapus
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default UserManagement;
