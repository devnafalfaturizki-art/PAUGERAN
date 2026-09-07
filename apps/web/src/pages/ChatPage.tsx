/**
 * ChatPage — Halaman chat utama
 * [CB §52] — Layout & Navigasi
 */

import { Component } from 'solid-js';
import ChatWindow from '../components/chat/ChatWindow';
import CaseGraphViewer from '../components/case_graph/CaseGraphViewer';

const ChatPage: Component = () => {
  return (
    <div class="chat-page flex h-screen">
      <div class="flex-1 flex flex-col">
        <ChatWindow />
      </div>
      <aside class="w-96 border-l border-gray-200 dark:border-gray-700 p-4 hidden lg:block">
        <CaseGraphViewer nodes={[]} edges={[]} />
      </aside>
    </div>
  );
};

export default ChatPage;
