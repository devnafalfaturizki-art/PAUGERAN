import { For, Show, createSignal, onMount } from 'solid-js';
import type { CaseState, CaseSummary, ReasoningMode } from '../../../packages/shared/src';
import { analyzeMessage, createCase as createCaseRequest, listCases, updateCaseMode } from './lib/api';

type Message = { role: 'system' | 'user'; content: string };

type CaseItem = CaseSummary & { meta: string };

const fallbackCases: CaseItem[] = [
  { id: '1', title: 'Sengketa perjanjian kerja', meta: 'Diperbarui 12 menit lalu', state: 'dispute' as CaseState, mode: 'dispute', updatedAt: new Date().toISOString() },
  { id: '2', title: 'Review kontrak vendor', meta: 'Kemarin', state: 'preventive' as CaseState, mode: 'preventive', updatedAt: new Date().toISOString() },
  { id: '3', title: 'Pertanyaan waris keluarga', meta: '3 hari lalu', state: 'exploration' as CaseState, mode: 'exploration', updatedAt: new Date().toISOString() },
];

function toCaseItem(item: CaseSummary): CaseItem {
  return { ...item, meta: new Date(item.updatedAt).toLocaleDateString('id-ID') };
}

const modes: { value: ReasoningMode; label: string; tone: string }[] = [
  { value: 'exploration', label: 'Exploration', tone: 'amber' },
  { value: 'preventive', label: 'Preventive', tone: 'green' },
  { value: 'dispute', label: 'Dispute', tone: 'red' },
  { value: 'litigation_prep', label: 'Litigation prep', tone: 'blue' },
  { value: 'adversarial', label: 'Adversarial', tone: 'violet' },
  { value: 'neutral', label: 'Neutral', tone: 'slate' },
];

const stateLabels: Record<CaseState, string> = {
  unknown: 'Belum dipetakan', exploration: 'Eksplorasi', preventive: 'Preventif', dispute: 'Sengketa', litigation: 'Litigasi', resolved: 'Selesai',
};

export default function App() {
  const [caseItems, setCaseItems] = createSignal<CaseItem[]>(fallbackCases);
  const [selectedCase, setSelectedCase] = createSignal(fallbackCases[0]);
  const [mode, setMode] = createSignal<ReasoningMode>('dispute');
  const [draft, setDraft] = createSignal('');
  const [messages, setMessages] = createSignal<Message[]>([
    { role: 'system', content: 'PAUGERAN siap memetakan posisi perkara. Mulai dengan fakta yang paling penting.' },
  ]);
  const activeMode = () => modes.find((item) => item.value === mode()) ?? modes[0];

  onMount(async () => {
    try {
      const data = await listCases();
      const loadedCases = data.map(toCaseItem);
      if (loadedCases.length > 0) {
        setCaseItems(loadedCases);
        setSelectedCase(loadedCases[0]);
        setMode(loadedCases[0].mode);
      }
    } catch {
      // The fallback keeps the workspace usable while the API is unavailable.
    }
  });

  async function createCase() {
    const title = window.prompt('Judul perkara baru');
    if (!title?.trim()) return;
    const created = toCaseItem(await createCaseRequest(title));
    setCaseItems((current) => [created, ...current]);
    setSelectedCase(created);
    setMode(created.mode);
  }

  async function updateMode(nextMode: ReasoningMode) {
    setMode(nextMode);
    const updated = toCaseItem(await updateCaseMode(selectedCase().id, nextMode));
    setSelectedCase(updated);
    setCaseItems((current) => current.map((item) => item.id === updated.id ? updated : item));
  }

  async function submitMessage(event: SubmitEvent) {
    event.preventDefault();
    const content = draft().trim();
    if (!content) return;
    setMessages((current) => [...current, { role: 'user', content }]);
    setDraft('');
    const analysis = await analyzeMessage(selectedCase().id, content);
    const questions = analysis.clarifyingQuestions.map((question) => `- ${question}`).join('\n');
    setMessages((current) => [...current, {
      role: analysis.role,
      content: `${analysis.content}\n\nKepastian awal: ${analysis.certaintyScore.toFixed(2)}\nPertanyaan klarifikasi:\n${questions}`,
    }]);
  }

  return (
    <main class="app-shell">
      <aside class="sidebar">
        <div class="brand"><span class="brand-mark">P</span><div><strong>PAUGERAN</strong><small>Supreme legal reasoning</small></div></div>
        <button class="new-case" type="button" onClick={createCase}><span>+</span> Perkara baru</button>
        <div class="case-heading"><span>PERKARA AKTIF</span><button type="button" aria-label="Cari perkara">/</button></div>
        <nav class="case-list" aria-label="Daftar perkara">
          <For each={caseItems()}>{(item) => <button classList={{ 'case-item': true, active: selectedCase().id === item.id }} type="button" onClick={() => { setSelectedCase(item); setMode(item.mode); }}><span class="case-dot" data-state={item.state}></span><span><strong>{item.title}</strong><small>{item.meta}</small></span></button>}</For>
        </nav>
        <div class="sidebar-footer"><button type="button">Pengaturan</button><button type="button">Bantuan</button><span class="status"><i></i> Mesin lokal aktif</span></div>
      </aside>

      <section class="workspace">
        <header class="topbar"><div><span class="eyebrow">PERKARA / {selectedCase().id.padStart(2, '0')}</span><h1>{selectedCase().title}</h1></div><div class="top-actions"><button class="icon-button" type="button" aria-label="Cari">⌕</button><button class="icon-button" type="button" aria-label="Notifikasi">◌</button><button class="avatar" type="button" aria-label="Profil pengguna">DF</button></div></header>
        <div class="mode-strip"><div class="mode-label"><span class={`mode-dot ${activeMode().tone}`}></span><span>Mode penalaran</span><strong>{activeMode().label}</strong></div><select value={mode()} onChange={(event) => updateMode(event.currentTarget.value as ReasoningMode)} aria-label="Pilih mode penalaran"><For each={modes}>{(item) => <option value={item.value}>{item.label}</option>}</For></select><div class="state-chip">State: <strong>{stateLabels[selectedCase().state]}</strong></div></div>
        <div class="chat-area"><div class="chat-intro"><span class="intro-kicker">ANALISIS TERLACAK</span><h2>Bangun posisi hukum dari fakta.</h2><p>Setiap kesimpulan akan ditautkan ke isu, norma, bukti, dan tingkat kepastian.</p></div><div class="message-list"><For each={messages()}>{(message) => <article class={`message ${message.role}`}><span class="message-label">{message.role === 'system' ? 'PAUGERAN' : 'ANDA'}</span><p>{message.content}</p></article>}</For></div></div>
        <form class="composer" onSubmit={submitMessage}><textarea value={draft()} onInput={(event) => setDraft(event.currentTarget.value)} placeholder="Jelaskan fakta, tujuan, atau dokumen yang ingin dianalisis..." rows="3"></textarea><div class="composer-footer"><span>Enter untuk mengirim</span><button class="send-button" type="submit">Kirim analisis <span>→</span></button></div></form>
      </section>

      <aside class="inspector"><div class="inspector-header"><div><span class="eyebrow">CASE GRAPH</span><h2>Struktur perkara</h2></div><button class="icon-button" type="button" aria-label="Perluas graph">↗</button></div><div class="graph-card"><div class="graph-lines"><span></span><span></span><span></span></div><div class="graph-node root"><b>Kasus</b><small>{selectedCase().title}</small></div><div class="graph-node issue"><b>Isu hukum</b><small>Wanprestasi</small></div><div class="graph-node fact"><b>Fakta</b><small>Hubungan kerja</small></div><div class="graph-node evidence"><b>Bukti</b><small>Perjanjian tertulis</small></div></div><div class="inspector-section"><div class="section-title"><h3>Ringkasan posisi</h3><span class="certainty">0.62</span></div><p class="muted">Posisi awal cukup didukung oleh dokumen, namun kronologi pemutusan dan nilai kerugian masih perlu dikonfirmasi.</p><div class="progress"><span style="width: 62%"></span></div></div><div class="inspector-section"><div class="section-title"><h3>Langkah berikutnya</h3><span>03</span></div><ol class="next-steps"><li>Konfirmasi tanggal dan pihak penandatangan.</li><li>Unggah bukti komunikasi terakhir.</li><li>Uji klausul penyelesaian sengketa.</li></ol></div><Show when={messages().length > 1}><div class="trace-note">Analisis baru tersimpan di riwayat perkara.</div></Show></aside>
    </main>
  );
}
