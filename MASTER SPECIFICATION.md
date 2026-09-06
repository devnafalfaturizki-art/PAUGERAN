# PAUGERAN — MASTER SPECIFICATION v1.0 FINAL
## Supreme Legal Reasoning Engine Edition

**Dokumen:** Master Specification — Supreme Legal Reasoning Architecture  
**Produk:** PAUGERAN — Supreme Legal Reasoning Agent  
**Status:** FINAL — SUMBER KEBENARAN MUTLAK  
**Versi:** 1.0 FINAL  
**Tanggal Efektif:** 28 Agustus 2026  
**Sifat:** Kontrak produk yang mengikat seluruh aspek pengembangan, deployment, dan operasional  
**Kedudukan:** Sumber kebenaran tunggal (single source of truth) untuk seluruh proyek PAUGERAN  
**Dokumen Turunan:** Semua dokumen teknis, arsitektur, testing, dan operasional harus merujuk ke dokumen ini

---

## 📋 DAFTAR ISI LENGKAP

**BAGIAN I — IDENTITAS & VISI PRODUK**
1. Definisi Produk
2. Visi & Misi
3. Tujuan Produk
4. Masalah yang Diselesaikan
5. Prinsip Produk (60 Prinsip)
6. Aktor & Stakeholder

**BAGIAN II — ARSITEKTUR PENALARAN HUKUM TINGKAT TINGGI**
7. Legal Reasoning Layers (7 Lapisan Penalaran)
8. Normative Conflict Resolution Engine
9. Temporal Law Application System
10. Judicial Discretion Modeling
11. Causation & Remoteness Analysis
12. Unwritten Law & Customary Law Integration
13. Ethical Guardrails System
14. Procedural & Formal Law Checker
15. Economic & Practical Reality Check
16. Multi-Jurisdictional & Private International Law
17. Tax Implication Module
18. Corporate Governance & Regulatory Compliance
19. Strategic Timing & Forum Analysis

**BAGIAN III — SPESIFIKASI FUNGSIONAL LENGKAP**
20. Case State Machine
21. Reasoning Modes (dengan Senior Counsel Checks)
22. Case Graph & Knowledge Representation (Enhanced)
23. Dynamic Mode Switching
24. Legal Knowledge Base (Supreme Edition)
25. Penelitian Web
26. Multi-Provider LLM
27. Autentikasi & Otorisasi
28. Kustomisasi Antarmuka
29. Aksesibilitas

**BAGIAN IV — SPESIFIKASI OUTPUT PROFESIONAL**
30. Mandatory Citation Format (Tidak Boleh Rangkuman)
31. Legal Hierarchy Display
32. Professional Document Templates
33. Mode-Specific Professional Outputs (Enhanced)
34. Export Dokumen Profesional

**BAGIAN V — SPESIFIKASI TEKNIS**
35. Arsitektur Single Binary
36. Stack Teknologi
37. Supreme Adaptive Graph Engine
38. Enhanced Case Graph Database Schema
39. API Specification
40. Frontend Specification
41. Build & Distribution

**BAGIAN VI — SPESIFIKASI PERILAKU AGEN (SUPREME)**
42. Exploration Mode Behavior (Enhanced)
43. Preventive Mode Behavior (Enhanced)
44. Dispute / Pre-Litigation Mode Behavior (Enhanced)
45. Litigation Preparation Mode Behavior (Enhanced)
46. Adversarial / Courtroom Mode Behavior (Enhanced)
47. Neutral / Judicial Mode Behavior (Enhanced)
48. Uncertainty as Structured Metric (Enhanced)
49. Kontrak Perilaku Adaptif (Supreme)
50. Standar Bahasa
51. Standar Keterlacakan (Supreme)

**BAGIAN VII — SPESIFIKASI ANTARMUKA**
52. Layout & Navigasi
53. Komponen UI
54. Design System
55. Responsivitas
56. Keyboard Shortcuts & Command Palette

**BAGIAN VIII — SPESIFIKASI DEPLOYMENT & OPERASIONAL**
57. Single Binary Deployment
58. Alternative Deployment Options
59. Environment Variables
60. Backup & Recovery
61. Monitoring & Observabilitas

**BAGIAN IX — SPESIFIKASI KEAMANAN & COMPLIANCE**
62. Keamanan Data
63. Enkripsi & Privacy
64. Compliance Hukum
65. Audit Trail

**BAGIAN X — KRITERIA PENERIMAAN & TESTING**
66. Kriteria Keberhasilan (Supreme)
67. Kriteria Penerimaan (Strict)
68. Testing Strategy
69. Quality Gates

**BAGIAN XI — TATA KELOLA & DOKUMEN TURUNAN**
70. Larangan Produk
71. Dokumen Turunan
72. Perubahan & Versi
73. Penutup

---

# BAGIAN I — IDENTITAS & VISI PRODUK

---

## 1. DEFINISI PRODUK

### 1.1 Apa Itu PAUGERAN

PAUGERAN adalah **Supreme Legal Reasoning Engine** — agen kecerdasan buatan untuk pemahaman, penelitian, analisis, dan penalaran hukum Indonesia yang bekerja secara adaptif berdasarkan kondisi perkara dan tujuan pengguna, dengan kapasitas penalaran setara advokat senior, hakim, jaksa, atau profesor hukum dengan pengalaman puluhan tahun dan ribuan kasus.

### 1.2 Karakteristik Utama

**1.2.1** PAUGERAN **tidak dirancang dengan asumsi bahwa setiap persoalan hukum harus berakhir pada litigasi**. Sistem mampu menangani spektrum persoalan mulai dari seseorang yang bahkan belum yakin apakah masalahnya memiliki dimensi hukum, kebutuhan pencegahan risiko, sengketa yang masih berada pada tahap negosiasi, persiapan litigasi, hingga perkara yang telah memasuki proses persidangan.

**1.2.2** Konsep utama PAUGERAN adalah **pemisahan antara Case State dan Reasoning Mode**. Case State menggambarkan posisi atau perkembangan perkara, sedangkan Reasoning Mode menentukan cara PAUGERAN melakukan penalaran.

**1.2.3** Dengan pemisahan ini, satu perkara dapat berubah status tanpa kehilangan keseluruhan konteks dan riwayat analisisnya, sementara metode penalaran dapat berubah sesuai kebutuhan.

**1.2.4** PAUGERAN dioperasikan sebagai **satu binary universal** yang berisi seluruh aplikasi dalam satu file executable, dapat dijalankan langsung tanpa instalasi dependency eksternal.

**1.2.5** PAUGERAN memiliki **7 Lapisan Penalaran Hukum (7 Legal Reasoning Layers)** yang meniru cara berpikir seorang ahli hukum senior:
1. **Grammatical Interpretation** (Penafsiran Tekstual)
2. **Systematic Interpretation** (Penafsiran Sistematis dalam Konteks Hierarki Norma)
3. **Teleological Interpretation** (Penafsiran Berdasarkan Tujuan Hukum)
4. **Sociological Interpretation** (Penafsiran Berdasarkan Realitas Sosial)
5. **Historical Interpretation** (Penafsiran Berdasarkan Sejarah & Legislasi)
6. **Comparative Interpretation** (Penafsiran Komparatif dengan Yurisprudensi)
7. **Critical Interpretation** (Penafsiran Kritis terhadap Kelemahan & Risiko)

**1.2.6** PAUGERAN **tidak menerima rangkuman pasal atau kutipan setengah-setengah**. Setiap output harus mencantumkan:
- Teks lengkap pasal/peraturan yang relevan
- Nomor, tahun, dan tanggal pengesahan
- Status keberlakuan (aktif/dicabut/diamendemen)
- Hierarki dalam sistem peraturan perundang-undangan
- URL sumber resmi (jika dari internet)
- Tanggal akses

**1.2.7** PAUGERAN memiliki **Sistem Bobot Preseden (Precedential Weight System)** yang membedakan:
- Putusan Pengadilan Negeri/Tinggi (bobot persuasif rendah)
- Putusan Mahkamah Agung (bobot sedang-tinggi)
- Yurisprudensi Tetap MA (bobot sangat tinggi, hampir mengikat)
- Surat Edaran Mahkamah Agung/SEMA (bobot mengikat secara internal)
- Putusan Mahkamah Konstitusi (bobot mengikat erga omnes)

**1.2.8** PAUGERAN secara proaktif mendeteksi dan menyelesaikan **24 Kelemahan Fundamental Hukum** yang sering diabaikan oleh sistem AI hukum lainnya (lihat Bagian II).

### 1.3 Apa yang BUKAN PAUGERAN

**1.3.1** PAUGERAN **bukan** mesin pencari pasal yang hanya mengembalikan teks peraturan.

**1.3.2** PAUGERAN **bukan** mesin pemberi jawaban "benar/salah" tanpa penjelasan.

**1.3.3** PAUGERAN **bukan** chatbot umum yang memberikan respons instan tanpa penalaran terstruktur.

**1.3.4** PAUGERAN **bukan** pengganti advokat atau konsultan hukum profesional.

**1.3.5** PAUGERAN **bukan** sistem linear yang memaksakan semua kasus melalui satu jalur penalaran yang sama.

**1.3.6** PAUGERAN **bukan** sistem yang memberikan kepastian hukum semu berdasarkan norma yang sudah "mati suri" atau dikalahkan oleh norma lain.

**1.3.7** PAUGERAN **bukan** sistem yang mengabaikan asas-asas hukum tidak tertulis, hukum adat, atau kebiasaan yang berlaku di masyarakat.

**1.3.8** PAUGERAN **bukan** sistem yang memberikan nasihat hukum tanpa mempertimbangkan implikasi praktis, ekonomi, dan strategis.

### 1.4 Formula Inti PAUGERAN

PAUGERAN bukanlah:
```
Input → LLM → Jawaban
```

PAUGERAN adalah:
```
Case State → Facts → Evidence → Issues → Multi-Layer Legal Interpretation → Norm Conflict Resolution → Temporal Application → Judicial Discretion Analysis → Causation Check → Legal Position → Counterposition → Stress Test → Risk → Practical Enforceability → Strategic Recommendation → Professional Output with Full Citation
```

### 1.5 Model Operasi

**1.5.1** PAUGERAN dioperasikan sebagai satu binary universal yang berisi:
- Backend Rust (Axum server)
- Frontend SolidJS (static files ter-embed)
- SQLite database engine dengan Case Graph support
- Supreme Adaptive Graph Engine (multi-mode reasoning dengan 7 lapisan penalaran)
- Multi-provider LLM clients
- Supreme Legal Knowledge Base
- Semua dependencies

**1.5.2** Binary ini dapat dijalankan langsung tanpa instalasi dependency eksternal (Node.js, Docker, database server).

**1.5.3** Binary tersedia untuk:
- Linux x86_64: `paugeran-linux`
- macOS Universal (Intel + Apple Silicon): `paugeran-macos`
- Windows x86_64: `paugeran-windows.exe`

**1.5.4** Saat dijalankan, binary:
- Start server HTTP di `http://localhost:3000`
- Auto-open browser
- Tampilkan setup wizard (input API key)
- Siap digunakan

### 1.6 Autentikasi

**1.6.1** Autentikasi multi-user adalah fitur opsional yang diaktifkan melalui environment variable `AUTH_ENABLED=true`.

**1.6.2** Secara default, PAUGERAN berjalan tanpa autentikasi untuk penggunaan pribadi.

### 1.7 Dukungan Model LLM

**1.7.1** PAUGERAN mendukung berbagai penyedia dan model LLM — tidak terbatas pada model ternama.

**1.7.2** PAUGERAN dapat menggunakan model dari Anthropic, OpenAI, Groq, Together AI, Fireworks, OpenRouter, Mistral AI, DeepSeek, Ollama (lokal), LM Studio (lokal), vLLM (lokal), dan penyedia lain yang kompatibel dengan API OpenAI atau Anthropic.

### 1.8 Supreme Legal Knowledge Base

**1.8.1** PAUGERAN memiliki Supreme Legal Knowledge Base — basis pengetahuan hukum internal yang dibangun dari peraturan, pasal, putusan, yurisprudensi, SEMA, doktrin, dan hukum adat yang pernah diteliti.

**1.8.2** Basis ini memiliki fitur khusus:
- **Norm Conflict Database**: Memetakan konflik antar peraturan
- **Precedential Weight System**: Memberikan bobot pada setiap putusan
- **Judicial Pitfall Library**: Kumpulan jebakan umum di pengadilan
- **Legal Heuristics Collection**: Aturan jempol hukum dari praktisi senior
- **Temporal Law Tracker**: Melacak perubahan peraturan dari waktu ke waktu

**1.8.3** Basis ini dapat digunakan sebagai referensi untuk analisis di masa depan tanpa perlu melakukan penelitian ulang dari internet.

---

## 2. VISI & MISI

### 2.1 Visi

Menjadi **Supreme Legal Reasoning Engine** terdepan di Indonesia yang mengutamakan kejujuran intelektual, keterlacakan penuh, privasi data, dan aksesibilitas, dengan kapasitas penalaran setara ahli hukum senior dengan puluhan tahun pengalaman.

### 2.2 Misi

Memberikan kepada advokat, hakim, jaksa, legal in-house, akademisi, dan masyarakat umum sebuah agen AI yang:
- Memahami masalah sebelum menyimpulkan
- Beradaptasi dengan kondisi perkara dan tujuan pengguna
- Meneliti sumber hukum yang valid dari basis pengetahuan internal dan internet
- Melakukan penafsiran hukum multi-lapisan (gramatikal, sistematis, teleologis, sosiologis, historis, komparatif, kritis)
- Menyelesaikan konflik norma secara sistematis
- Menerapkan hukum yang tepat berdasarkan waktu kejadian
- Menyajikan argumen berimbang dengan kontraargumentasi
- Melakukan stress test terhadap konstruksi hukum
- Mempertimbangkan implikasi praktis, ekonomi, dan strategis
- Menjaga kerahasiaan data klien
- Menghasilkan analisis yang dapat dipertanggungjawabkan secara profesional dengan sitasi lengkap
- Dapat diakses oleh pengguna dengan berbagai kemampuan
- Dapat di-deploy dengan mudah tanpa setup teknis yang rumit

---

## 3. TUJUAN PRODUK

PAUGERAN harus memungkinkan pengguna untuk:

### 3.1 Instalasi & Akses

**3.1.1** Menjalankan produk dengan satu perintah (download binary dan jalankan).

**3.1.2** Mengakses produk dari browser mana saja pada deployment cloud atau VPS (jika dipilih).

**3.1.3** Menggunakan produk tanpa keahlian teknis untuk instalasi.

**3.1.4** Tidak perlu install dependency eksternal (Node.js, Docker, database server).

### 3.2 Konfigurasi

**3.2.1** Memasukkan API key LLM melalui antarmuka web tanpa menyentuh terminal.

**3.2.2** Memilih penyedia dan model LLM dari berbagai pilihan yang didukung.

**3.2.3** Mengganti atau menghapus API key dan provider kapan saja melalui pengaturan.

### 3.3 Manajemen Perkara (Case Management)

**3.3.1** Membuat perkara baru kapan saja untuk topik hukum yang berbeda.

**3.3.2** Mengelola banyak perkara secara paralel.

**3.3.3** Membuka kembali perkara lama kapan saja tanpa batas waktu.

**3.3.4** Menghapus perkara kapan saja secara permanen.

**3.3.5** Melihat Case State dan Reasoning Mode saat ini untuk setiap perkara.

**3.3.6** Mengubah Case State secara manual atau membiarkan sistem mendeteksi secara otomatis.

**3.3.7** Beralih antar Reasoning Mode sesuai kebutuhan.

### 3.4 Dialog & Pemahaman Adaptif

**3.4.1** Menjelaskan masalah menggunakan bahasa natural dalam antarmuka chat.

**3.4.2** Mendapatkan pertanyaan klarifikasi yang relevan dan adaptif berdasarkan Reasoning Mode aktif.

**3.4.3** Membangun pemahaman masalah secara bertahap dalam satu perkara.

**3.4.4** Mengoreksi pemahaman PAUGERAN melalui dialog.

**3.4.5** Menentukan kapan proses pemahaman dianggap cukup.

**3.4.6** Mendapatkan **Fact Re-qualification Challenge** di mana PAUGERAN secara aktif menantang kualifikasi hukum dari fakta yang disampaikan pengguna.

### 3.5 Penelitian & Penalaran Multi-Mode

**3.5.1** Meminta PAUGERAN melakukan penalaran hukum dalam mode yang sesuai.

**3.5.2** Memperoleh penelitian hukum dari basis pengetahuan internal dan dari situs web resmi pemerintah serta sumber tepercaya lainnya di internet.

**3.5.3** Menyimpan peraturan, pasal, PP, dan sejenisnya yang pernah diteliti ke dalam Legal Knowledge Base untuk digunakan sebagai referensi di perkara berikutnya.

**3.5.4** Mengelola Legal Knowledge Base: menambah, memperbarui, menandai sebagai tidak berlaku, dan menghapus entri.

**3.5.5** Menggunakan Exploration Mode untuk eksplorasi isu hukum tanpa komitmen pada kesimpulan.

**3.5.6** Menggunakan Preventive Mode untuk analisis risiko dan mitigasi.

**3.5.7** Menggunakan Dispute Mode untuk pemetaan posisi para pihak dan opsi penyelesaian.

**3.5.8** Menggunakan Litigation Preparation Mode untuk membangun case theory dan struktur pembuktian.

**3.5.9** Menggunakan Adversarial Mode untuk stress test argumentasi.

**3.5.10** Menggunakan Neutral Mode untuk analisis netral dan simulasi judicial review.

**3.5.11** Mendapatkan **Multi-Layer Legal Interpretation** untuk setiap isu hukum.

**3.5.12** Mendapatkan **Norm Conflict Resolution** jika terdapat konflik antar peraturan.

**3.5.13** Mendapatkan **Temporal Law Application** yang memastikan hukum yang tepat diterapkan berdasarkan waktu kejadian.

**3.5.14** Mendapatkan **Judicial Discretion Analysis** yang mempertimbangkan kebebasan hakim dalam menilai bukti.

**3.5.15** Mendapatkan **Causation & Remoteness Analysis** untuk memastikan hubungan kausal yang tepat.

**3.5.16** Mendapatkan **Unwritten Law & Customary Law Integration** jika kasus melibatkan aspek hukum tidak tertulis.

**3.5.17** Mendapatkan **Ethical Guardrails** yang memastikan strategi hukum sesuai dengan kode etik profesi.

**3.5.18** Mendapatkan **Procedural & Formal Law Check** yang mendeteksi jebakan formil dan prosedural.

**3.5.19** Mendapatkan **Economic & Practical Reality Check** yang mempertimbangkan implikasi praktis dan ekonomi.

**3.5.20** Mendapatkan **Multi-Jurisdictional Analysis** jika kasus melibatkan beberapa yurisdiksi.

**3.5.21** Mendapatkan **Tax Implication Analysis** untuk setiap transaksi hukum.

**3.5.22** Mendapatkan **Corporate Governance & Regulatory Compliance Check** untuk kasus korporasi.

**3.5.23** Mendapatkan **Strategic Timing & Forum Analysis** untuk menentukan waktu dan forum terbaik.

### 3.6 Output & Analisis

**3.6.1** Mengetahui dasar hukum yang digunakan dengan **FULL CITATION** (teks lengkap pasal, bukan rangkuman) pada setiap bagian output.

**3.6.2** Memahami penerapan hukum terhadap fakta.

**3.6.3** Melihat argumentasi yang mendukung dan berlawanan.

**3.6.4** Mengetahui ketidakpastian dan kelemahan analisis sebagai structured metric.

**3.6.5** Melihat alternatif penafsiran.

**3.6.6** Menelusuri setiap kesimpulan menuju dasar dan sumbernya melalui Case Graph visual.

**3.6.7** Memperoleh output profesional yang sesuai dengan Reasoning Mode aktif.

**3.6.8** Mendapatkan **Legal Hierarchy Display** yang menunjukkan hierarki setiap peraturan yang dikutip.

**3.6.9** Mendapatkan **Precedential Weight Indicator** untuk setiap putusan yang dikutip.

**3.6.10** Mendapatkan **Professional Document Output** yang siap digunakan dalam konteks profesional.

### 3.7 Data & Ekspor

**3.7.1** Menyimpan seluruh data kasus secara privat di penyimpanan yang dikendalikan pengguna.

**3.7.2** Mengekspor hasil analisis dalam format PDF profesional dan DOCX profesional dengan template yang sesuai mode.

**3.7.3** Mengunggah dokumen pendukung dalam format PDF, DOCX, dan TXT ke dalam perkara.

**3.7.4** Mem-backup dan merestore seluruh data dengan mudah.

### 3.8 Kustomisasi & Aksesibilitas

**3.8.1** Mengustomisasi antarmuka pengguna meliputi tema warna, ukuran font, tata letak, dan bahasa sesuai preferensi pribadi.

**3.8.2** Menyimpan preferensi kustomisasi secara persisten.

**3.8.3** Menggunakan PAUGERAN dengan aksesibilitas tinggi melalui keyboard shortcuts, command palette, screen reader support, dan mode aksesibilitas.

### 3.9 Manajemen Tim (jika AUTH_ENABLED=true)

**3.9.1** Mengelola anggota tim melalui sistem undangan.

**3.9.2** Menyediakan API key global untuk kenyamanan tim.

**3.9.3** Memantau penggunaan tim melalui statistik agregat.

---

## 4. MASALAH YANG DISELESAIKAN

### 4.1 Masalah Sistem Hukum

**4.1.1** Fakta pengguna sering tidak lengkap.

**4.1.2** Istilah yang digunakan pengguna belum tentu merupakan istilah hukum yang tepat.

**4.1.3** Satu fakta dapat memiliki beberapa konsekuensi hukum.

**4.1.4** Satu masalah dapat melibatkan beberapa bidang hukum.

**4.1.5** Aturan hukum dapat berubah, memiliki pengecualian, dan bertentangan satu sama lain.

**4.1.6** Putusan pengadilan dapat memiliki fakta yang berbeda meskipun kasusnya tampak serupa.

**4.1.7** Suatu norma dapat memiliki beberapa interpretasi yang sah.

**4.1.8** Kekuatan suatu kesimpulan bergantung pada fakta dan bukti yang tersedia.

**4.1.9** Informasi hukum di internet memiliki tingkat keandalan yang berbeda.

**4.1.10** Tidak semua masalah hukum berakhir pada litigasi — banyak yang diselesaikan melalui negosiasi, mediasi, atau tindakan preventif.

**4.1.11** **Konflik norma dan tumpang tindih regulasi** sangat umum di Indonesia, menyebabkan kebingungan tentang hukum mana yang berlaku.

**4.1.12** **Asas-asas hukum tidak tertulis** (itikad baik, kepatutan, kebiasaan) sering diabaikan oleh sistem AI, padahal sangat penting dalam praktik hukum.

**4.1.13** **Penerapan hukum yang salah secara temporal** (menggunakan hukum baru untuk peristiwa lama) sering terjadi.

**4.1.14** **Kebebasan hakim dalam menilai bukti** (vrije bewijswaardering) sering tidak dipertimbangkan dalam analisis AI.

**4.1.15** **Hubungan kausalitas yang terlalu jauh** (remote cause) sering tidak terdeteksi, menyebabkan gugatan dengan nilai kerugian yang tidak masuk akal.

**4.1.16** **Hukum adat dan kebiasaan** yang tidak terdokumentasi sering diabaikan, padahal sangat relevan dalam banyak kasus.

**4.1.17** **Jebakan prosedural dan formil** (daluwarsa, kewenangan pengadilan, error in persona) sering menyebabkan kasus gugur meskipun secara substansi benar.

**4.1.18** **Implikasi praktis dan ekonomi** dari kemenangan hukum sering tidak dipertimbangkan (menang secara hukum tapi tidak mendapatkan hasil yang bernilai).

**4.1.19** **Aspek etika profesi** sering diabaikan, menyebabkan saran yang secara teknis legal tetapi secara etika meragukan.

**4.1.20** **Implikasi pajak** dari transaksi hukum sering tidak dipertimbangkan.

**4.1.21** **Multi-yurisdiksi dan pilihan hukum** dalam kasus internasional sering tidak ditangani dengan baik.

**4.1.22** **Corporate governance dan regulatory compliance** sering diabaikan dalam kasus korporasi.

**4.1.23** **Strategic timing dan forum shopping** sering tidak dipertimbangkan dalam strategi litigasi.

**4.1.24** **Kualifikasi hukum yang salah** dari fakta yang disampaikan pengguna sering tidak dikoreksi oleh sistem AI.

---

## 5. PRINSIP PRODUK

Prinsip-prinsip berikut bersifat mengikat dan tidak dapat dikompromikan.

### 5.1 Prinsip Penalaran Adaptif

**P-01 — Pemahaman sebelum kesimpulan**
PAUGERAN tidak boleh langsung memberikan kesimpulan hukum mendalam apabila informasi material belum memadai.

**P-02 — Dialog adaptif berdasarkan mode**
Pertanyaan lanjutan harus dihasilkan berdasarkan informasi yang telah diperoleh dan Reasoning Mode aktif. PAUGERAN tidak boleh menggunakan daftar pertanyaan statis sebagai satu-satunya mekanisme wawancara.

**P-03 — Pengguna dapat mengoreksi**
PAUGERAN harus menyajikan pemahaman sementara dan memberikan kesempatan kepada pengguna untuk memperbaikinya dalam perkara yang sama.

**P-04 — Fakta bukan asumsi**
Pernyataan pengguna harus dibedakan dari fakta yang telah diverifikasi melalui dokumen atau sumber lain.

**P-05 — Sumber adalah bagian dari penalaran**
Sumber hukum bukan sekadar daftar referensi di bagian akhir. Setiap sumber harus mempunyai hubungan dengan klaim atau bagian analisis yang menggunakannya.

**P-06 — Kesimpulan harus dapat ditelusuri melalui Case Graph**
Setiap kesimpulan material harus dapat ditelusuri melalui rantai: Kesimpulan → Argumentasi → Interpretasi → Pasal/Putusan → Fakta Material → Bukti → Dokumen Sumber.

**P-07 — Penalaran harus berimbang dan adversarial**
PAUGERAN wajib mencari argumentasi yang dapat melemahkan kesimpulannya sendiri, terutama dalam Adversarial Mode.

**P-08 — Ketidakpastian adalah fitur, bukan bug**
Ketidakpastian bukan kegagalan sistem. Apabila fakta belum cukup, sistem harus mengatakan bahwa fakta belum cukup. Apabila dua interpretasi hukum sama-sama memiliki dasar, sistem harus menampilkan keduanya.

**P-09 — Tidak memalsukan kepastian**
Jika kesimpulan tidak dapat ditentukan secara pasti, PAUGERAN harus menyatakan kondisi tersebut secara eksplisit dengan structured uncertainty metric.

**P-10 — Bahasa profesional dan mudah dipahami**
Bahasa keluaran harus memenuhi standar komunikasi hukum profesional Indonesia tanpa sengaja dibuat rumit.

**P-11 — Mode-agnostic reasoning**
PAUGERAN tidak memaksakan semua kasus melalui satu jalur penalaran. Reasoning Mode dipilih berdasarkan Case State dan tujuan pengguna.

**P-12 — Dynamic state transition**
Perkara dapat berpindah antar Case State dan Reasoning Mode secara dinamis berdasarkan fakta baru, bukti baru, perubahan posisi para pihak, atau tujuan baru pengguna.

**P-13 — Case history preservation**
Seluruh perpindahan state dan mode tetap mempertahankan case history, sumber, bukti, argumentasi, dan keputusan sebelumnya.

### 5.2 Prinsip Privasi & Data

**P-14 — Privasi dan isolasi perkara**
Data dalam satu perkara tidak boleh bocor ke perkara lain. Setiap perkara adalah entitas terisolasi. Data pengguna tidak boleh bocor ke pihak ketiga.

**P-15 — Data 100% lokal**
Seluruh data pengguna tersimpan di penyimpanan yang dikendalikan pengguna. Tidak ada replikasi ke pihak ketiga tanpa persetujuan eksplisit.

**P-16 — API key adalah milik pengguna**
API key LLM disimpan terenkripsi di penyimpanan pengguna. API key tidak pernah dikirim ke server PAUGERAN. Pengguna memiliki kontrol penuh.

### 5.3 Prinsip Produk

**P-17 — Produk harus siap pakai**
PAUGERAN harus dapat digunakan segera setelah instalasi tanpa konfigurasi tambahan yang rumit oleh pengguna akhir.

**P-18 — Chat-first experience**
Antarmuka utama adalah chat yang intuitif. Fitur kompleks seperti Case Graph visual dan laporan harus dapat diakses dari dalam chat tanpa meninggalkan konteks obrolan.

**P-19 — Instalasi satu langkah**
Pengguna harus dapat menjalankan PAUGERAN dengan satu perintah. Tidak perlu setup database terpisah, tidak perlu konfigurasi server, tidak perlu keahlian teknis, tidak perlu install dependency eksternal.

**P-20 — Autentikasi opsional**
Autentikasi multi-user adalah fitur opsional yang diaktifkan melalui `AUTH_ENABLED=true`. Secara default, PAUGERAN berjalan tanpa autentikasi untuk kemudahan penggunaan pribadi.

**P-21 — Perkara adalah entitas independen**
Setiap perkara berdiri sendiri. Perkara A tidak mengetahui keberadaan perkara B. Satu-satunya data yang bersifat global adalah API key, preferensi UI per pengguna, dan Legal Knowledge Base.

**P-22 — Kustomisasi adalah hak pengguna**
Pengguna harus dapat menyesuaikan antarmuka sesuai preferensi visual dan ergonomi mereka. Preferensi disimpan secara persisten dan diaplikasikan secara global.

### 5.4 Prinsip Teknis

**P-23 — Performa dan keamanan melalui Rust**
Mesin inti PAUGERAN dibangun di atas Rust untuk menjamin keamanan memori tanpa garbage collector, konkurensi berperforma tinggi melalui Tokio, ukuran biner yang kecil, startup time yang cepat, dan konsumsi resource yang minimal.

**P-24 — Single binary universal**
PAUGERAN didistribusikan sebagai satu binary universal yang berisi seluruh aplikasi (backend, frontend, database engine) dalam satu file executable. Binary ini dapat dijalankan langsung tanpa dependency eksternal. Frontend static files ter-embed ke binary menggunakan `rust-embed`.

**P-25 — Case Graph sebagai fondasi data**
PAUGERAN tidak menjadikan percakapan sebagai satu-satunya sumber konteks. Setiap perkara direpresentasikan sebagai Case Graph yang menghubungkan Parties, Facts, Evidence, Issues, Legal Rules, Sources, Arguments, Counterarguments, Risks, Conclusions, dan Documents.

### 5.5 Prinsip Tim & Admin

**P-26 — First user is admin**
Saat `AUTH_ENABLED=true`, user pertama yang mendaftar otomatis mendapatkan peran admin. Admin bertanggung jawab mengelola anggota tim berikutnya melalui sistem undangan.

**P-27 — Admin adalah user biasa plus hak istimewa**
Admin memiliki data, sesi, dan preferensi sendiri seperti user biasa. Hak istimewa admin terbatas pada manajemen tim dan konfigurasi sistem.

**P-28 — Privacy-preserving administration**
Admin tidak dapat melihat API key pribadi user lain, isi sesi obrolan user lain, atau data pribadi user lain. Admin hanya dapat melihat metadata statistik untuk keperluan manajemen.

**P-29 — Fallback API key hierarchy**
API key dievaluasi dengan urutan: pertama API key pribadi user, kedua API key global yang disediakan admin, ketiga error jika keduanya tidak ada.

### 5.6 Prinsip Multi-Provider

**P-30 — Multi-provider LLM agnostik**
PAUGERAN tidak mengunci pengguna pada satu penyedia atau model LLM tertentu. PAUGERAN mendukung berbagai penyedia dan model — termasuk yang tidak ternama — selama model tersebut menyediakan API yang kompatibel. Pengguna bebas memilih berdasarkan kebutuhan, ketersediaan, dan biaya.

### 5.7 Prinsip Sumber & Penelitian

**P-31 — FULL CITATION dalam setiap output (TIDAK BOLEH RANGKUMAN)**
Setiap klaim hukum, setiap kutipan, setiap referensi dalam output PAUGERAN harus disertai dengan **FULL CITATION** yang mencakup:
- **Teks lengkap** pasal/peraturan yang relevan (bukan rangkuman)
- Nomor, tahun, dan tanggal pengesahan
- Status keberlakuan (aktif/dicabut/diamendemen)
- Hierarki dalam sistem peraturan perundang-undangan
- URL sumber resmi (jika dari internet)
- Tanggal akses

**P-32 — Penelitian web yang bertanggung jawab**
PAUGERAN hanya boleh mengakses situs web yang masuk dalam daftar putih (whitelist) yang mencakup situs pemerintah resmi dan sumber hukum tepercaya. Akses ke situs di luar whitelist dilarang. Setiap akses web harus mencantumkan sumber secara eksplisit dalam output.

**P-33 — Basis pengetahuan yang dapat dibangun**
Peraturan, pasal, PP, dan sejenisnya yang pernah diteliti oleh PAUGERAN dapat disimpan ke dalam Legal Knowledge Base internal. Basis pengetahuan ini menjadi sumber referensi untuk analisis di masa depan tanpa perlu melakukan penelitian ulang dari internet, mempercepat analisis dan mengurangi ketergantungan pada koneksi internet.

### 5.8 Prinsip Aksesibilitas & Export

**P-34 — Aksesibilitas adalah standar, bukan fitur tambahan**
PAUGERAN harus dapat digunakan oleh pengguna dengan berbagai kemampuan. Keyboard navigation, screen reader support, high contrast mode, dan reduced motion harus tersedia sebagai standar.

**P-35 — Export profesional siap pakai**
Laporan yang diekspor harus siap digunakan dalam konteks profesional tanpa perlu formatting ulang. Template harus memenuhi standar dokumen hukum Indonesia dan sesuai dengan Reasoning Mode yang digunakan.

### 5.9 Prinsip Penalaran Hukum Tingkat Tinggi (SUPREME)

**P-36 — Multi-Layer Legal Interpretation (7 Lapisan Penafsiran)**
PAUGERAN wajib melakukan penafsiran hukum melalui 7 lapisan:
1. **Grammatical Interpretation**: Penafsiran berdasarkan teks pasal
2. **Systematic Interpretation**: Penafsiran dalam konteks hierarki norma dan sistem hukum
3. **Teleological Interpretation**: Penafsiran berdasarkan tujuan hukum (geest van de wet)
4. **Sociological Interpretation**: Penafsiran berdasarkan realitas sosial dan dampak
5. **Historical Interpretation**: Penafsiran berdasarkan sejarah legislasi dan maksud pembentuk UU
6. **Comparative Interpretation**: Penafsiran berdasarkan yurisprudensi dan putusan pengadilan
7. **Critical Interpretation**: Penafsiran kritis terhadap kelemahan, risiko, dan alternatif

**P-37 — Norm Conflict Resolution**
PAUGERAN wajib mendeteksi dan menyelesaikan konflik antar peraturan menggunakan asas-asas:
- *Lex superior derogat legi inferiori* (hukum yang lebih tinggi mengesampingkan yang lebih rendah)
- *Lex specialis derogat legi generali* (hukum yang lebih khusus mengesampingkan yang lebih umum)
- *Lex posterior derogat legi priori* (hukum yang lebih baru mengesampingkan yang lebih lama)

**P-38 — Temporal Law Application (Lex Temporis)**
PAUGERAN wajib menerapkan hukum yang berlaku pada saat peristiwa hukum terjadi, bukan hukum yang berlaku saat ini, kecuali ada ketentuan yang secara eksplisit menyatakan berlaku surut.

**P-39 — Judicial Discretion Modeling**
PAUGERAN wajib mempertimbangkan kebebasan hakim dalam menilai bukti (vrije bewijswaardering) dan tidak memberikan kepastian semu berdasarkan kekuatan bukti formil semata.

**P-40 — Causation & Remoteness Analysis**
PAUGERAN wajib menganalisis hubungan kausalitas antara perbuatan dan kerugian, serta mendeteksi apakah kerugian tersebut merupakan akibat langsung (proximate cause) atau akibat yang terlalu jauh (remote cause).

**P-41 — Unwritten Law & Customary Law Integration**
PAUGERAN wajib mempertimbangkan asas-asas hukum tidak tertulis, hukum adat, dan kebiasaan yang berlaku di masyarakat, terutama untuk kasus yang melibatkan aspek agraria, waris, dan bisnis keluarga.

**P-42 — Ethical Guardrails**
PAUGERAN wajib memastikan bahwa setiap strategi hukum yang disarankan sesuai dengan Kode Etik Advokat Indonesia dan tidak menyarankan taktik yang bersifat frivolous, vexatious, atau melanggar etika profesi.

**P-43 — Procedural & Formal Law Check**
PAUGERAN wajib melakukan pemeriksaan menyeluruh terhadap aspek formil dan prosedural sebelum memberikan rekomendasi litigasi, termasuk:
- Daluwarsa (statute of limitations)
- Kewenangan pengadilan (absolut dan relatif)
- Ne bis in idem
- Error in persona
- Keberadaan klausula arbitrase

**P-44 — Economic & Practical Reality Check**
PAUGERAN wajib mempertimbangkan implikasi praktis dan ekonomi dari kemenangan hukum, termasuk:
- Kemampuan eksekusi lawan (judgment proof)
- Biaya vs manfaat litigasi
- Dampak pada hubungan bisnis jangka panjang

**P-45 — Fact Re-qualification Challenge**
PAUGERAN wajib secara aktif menantang kualifikasi hukum dari fakta yang disampaikan pengguna dan mengusulkan kualifikasi yang lebih tepat berdasarkan hukum.

**P-46 — Hierarchical Legal Citation**
PAUGERAN wajib menampilkan hierarki setiap peraturan yang dikutip dalam output, menunjukkan posisi peraturan tersebut dalam sistem peraturan perundang-undangan Indonesia.

**P-47 — Precedential Weight System**
PAUGERAN wajib memberikan bobot pada setiap putusan yang dikutip berdasarkan:
- Tingkat pengadilan (PN/PT/MA/MK)
- Status yurisprudensi (tetap/belum tetap)
- Konsistensi dengan putusan lain
- Tanggal putusan (relevansi temporal)

**P-48 — Multi-Jurisdictional Analysis**
PAUGERAN wajib menangani kasus yang melibatkan beberapa yurisdiksi dengan menganalisis:
- Pilihan hukum (choice of law)
- Yurisdiksi yang berwenang
- Pengakuan dan eksekusi putusan asing

**P-49 — Tax Implication Analysis**
PAUGERAN wajib mempertimbangkan implikasi pajak dari setiap transaksi hukum yang dianalisis.

**P-50 — Corporate Governance & Regulatory Compliance**
PAUGERAN wajib memeriksa aspek corporate governance dan regulatory compliance untuk kasus yang melibatkan korporasi.

**P-51 — Strategic Timing & Forum Analysis**
PAUGERAN wajib memberikan rekomendasi mengenai waktu dan forum terbaik untuk mengajukan gugatan atau mengambil tindakan hukum.

**P-52 — Ratio Decidendi vs Obiter Dicta Distinction**
PAUGERAN wajib membedakan antara bagian putusan yang mengikat (ratio decidendi) dan bagian yang hanya berupa komentar (obiter dicta) saat mengutip putusan pengadilan.

**P-53 — Split Decision Tracker**
PAUGERAN wajib melacak dan menampilkan putusan-putusan yang saling bertentangan untuk kasus serupa, serta menjelaskan perbedaan interpretasi.

**P-54 — Legal Drafting Precision**
PAUGERAN wajib menghasilkan draf dokumen hukum (kontrak, gugatan, jawaban) yang presisi, tidak ambigu, dan sesuai dengan standar praktik hukum profesional.

**P-55 — Dynamic Burden of Proof Tracking**
PAUGERAN wajib melacak pergeseran beban pembuktian selama proses analisis dan persidangan.

**P-56 — Transitional Law Module**
PAUGERAN wajib menangani masa transisi saat UU baru disahkan dengan aturan peralihan yang kompleks.

**P-57 — Private International Law Module**
PAUGERAN wajib menangani kasus dengan unsur asing, termasuk pilihan hukum dan yurisdiksi.

**P-58 — Islamic Law Module**
PAUGERAN wajib mempertimbangkan aspek hukum Islam untuk kasus yang melibatkan pihak Muslim (waris, perkawinan, ekonomi syariah).

**P-59 — Public Policy Filter**
PAUGERAN wajib mendeteksi klausula atau tindakan yang meskipun secara tekstual sah, dapat batal demi kebijakan publik.

**P-60 — Enforcement Strategy Module**
PAUGERAN wajib memberikan strategi eksekusi putusan yang realistis dan dapat dilaksanakan.

---

## 6. AKTOR & STAKEHOLDER

### 6.1 Pengguna (User)

**6.1.1** Orang yang menggunakan PAUGERAN.

**6.1.2** Pengguna memiliki data, perkara, dan preferensi sendiri.

**6.1.3** Dalam deployment tanpa auth, semua pengguna adalah local user tunggal.

**6.1.4** Dalam deployment dengan auth aktif, pengguna adalah anggota tim yang di-invite oleh admin.

### 6.2 Administrator (Admin)

**6.2.1** User pertama yang mendaftar saat `AUTH_ENABLED=true`.

**6.2.2** Admin memiliki hak istimewa untuk mengelola anggota tim, menyediakan API key global, mengelola Legal Knowledge Base, mengonfigurasi sistem, dan mengelola lisensi.

**6.2.3** Admin tetap memiliki data pribadi sendiri yang tidak dapat diakses oleh admin lain.

### 6.3 PAUGERAN (Supreme Legal Reasoning Agent)

**6.3.1** Agen AI yang melakukan wawancara adaptif dalam perkara, pemodelan masalah, penelitian hukum (dari basis pengetahuan internal dan internet), penalaran multi-mode dengan 7 lapisan penafsiran, pengujian adversarial, dan penyusunan output profesional.

**6.3.2** PAUGERAN dapat beroperasi dalam 6 Reasoning Mode berbeda: Exploration, Preventive, Dispute/Pre-Litigation, Litigation Preparation, Adversarial/Courtroom, dan Neutral/Judicial.

### 6.4 Perkara (Case)

**6.4.1** Entitas yang menaungi satu topik analisis hukum.

**6.4.2** Setiap perkara terisolasi dari perkara lain dan memiliki:
- ID unik
- Judul (auto-generated atau manual)
- Case State (Unknown → Resolved)
- Reasoning Mode aktif
- Case Graph (Parties, Facts, Evidence, Issues, Rules, Arguments, dll)
- Daftar pesan
- Dokumen yang diunggah
- Riwayat perubahan state dan mode
- Timestamp pembuatan dan pembaruan

### 6.5 Sumber Hukum

**6.5.1** Sumber eksternal yang digunakan sebagai dasar analisis.

**6.5.2** Termasuk peraturan perundang-undangan, putusan pengadilan, doktrin, dan dokumen resmi lembaga.

**6.5.3** Sumber dapat berasal dari Legal Knowledge Base internal atau dari internet melalui penelitian web.

### 6.6 Dokumen Pengguna

**6.6.1** Dokumen yang diberikan pengguna sebagai sumber fakta atau bukti.

**6.6.2** Disimpan secara aman dan terisolasi dalam perkara.

### 6.7 Penyedia LLM

**6.7.1** Layanan eksternal yang dipanggil oleh PAUGERAN menggunakan API key milik pengguna atau global.

**6.7.2** PAUGERAN mendukung berbagai penyedia termasuk Anthropic, OpenAI, Groq, Together AI, Fireworks, OpenRouter, Mistral AI, DeepSeek, Ollama (lokal), LM Studio (lokal), vLLM (lokal), dan penyedia lain yang kompatibel dengan API OpenAI atau Anthropic.

**6.7.3** PAUGERAN tidak menyimpan, memproses, atau meneruskan API key ke pihak lain selain penyedia resmi yang dipilih pengguna.

### 6.8 Supreme Legal Knowledge Base

**6.8.1** Basis pengetahuan hukum internal yang dibangun dari peraturan, pasal, PP, putusan, yurisprudensi, SEMA, doktrin, dan hukum adat yang pernah diteliti oleh PAUGERAN dan disimpan secara eksplisit oleh pengguna atau admin.

**6.8.2** Basis ini bersifat global (dapat diakses semua perkara) dan read-only selama analisis.

**6.8.3** Basis ini memiliki fitur khusus:
- **Norm Conflict Database**: Memetakan konflik antar peraturan
- **Precedential Weight System**: Memberikan bobot pada setiap putusan
- **Judicial Pitfall Library**: Kumpulan jebakan umum di pengadilan
- **Legal Heuristics Collection**: Aturan jempol hukum dari praktisi senior
- **Temporal Law Tracker**: Melacak perubahan peraturan dari waktu ke waktu

---

# BAGIAN II — ARSITEKTUR PENALARAN HUKUM TINGKAT TINGGI

---

## 7. LEGAL REASONING LAYERS (7 LAPISAN PENALARAN)

### 7.1 Konsep Dasar

PAUGERAN tidak menggunakan pendekatan penalaran tunggal. Setiap isu hukum dianalisis melalui **7 lapisan penalaran** yang saling melengkapi, meniru cara berpikir seorang ahli hukum senior yang telah menangani ribuan kasus selama puluhan tahun.

### 7.2 Layer 1: Grammatical Interpretation (Penafsiran Gramatikal)

**7.2.1 Definisi**
Penafsiran berdasarkan makna harfiah kata-kata dalam peraturan, sesuai dengan kaidah bahasa Indonesia yang baku dan konteks penggunaan dalam peraturan perundang-undangan.

**7.2.2 Metodologi**
- Analisis morfologi kata (imbuhan, akar kata)
- Analisis sintaksis (struktur kalimat)
- Analisis semantik (makna kata dalam konteks hukum)
- Perbandingan dengan definisi dalam pasal definisi peraturan terkait
- Perbandingan dengan Kamus Besar Bahasa Indonesia (KBBI)
- Perbandingan dengan kamus hukum (Black's Law Dictionary, Kamus Hukum Indonesia)

**7.2.3 Contoh Output**
```
LAPISAN 1: PENAFSIRAN GRAMATIKAL
═══════════════════════════════════

Pasal yang Dianalisis:
"Pihak yang karena kesalahannya mengakibatkan tidak dilaksanakannya 
perjanjian, wajib mengganti kerugian."

Istilah Kunci:
1. "Kesalahan" 
   → Definisi: Perbuatan yang bertentangan dengan kepatutan 
     atau kelalaian dalam melaksanakan kewajiban
   → Sumber: Yurisprudensi MA No. 3155 K/Pdt/1984
   
2. "Mengakibatkan"
   → Definisi: Menyebabkan sebagai akibat langsung
   → Sumber: KBBI Edisi V
   
3. "Mengganti kerugian"
   → Definisi: Memberikan kompensasi atas kerugian yang diderita
   → Sumber: Pasal 1246 KUHPerdata

Makna Harfiah:
Pihak yang melakukan kesalahan (perbuatan melawan hukum atau 
kelalaian) yang secara langsung menyebabkan tidak terlaksananya 
perjanjian, berkewajiban memberikan kompensasi atas kerugian 
yang timbul.

Ambiguitas yang Terdeteksi:
⚠️ Istilah "kesalahan" dapat ditafsirkan secara luas (termasuk 
   force majeure) atau sempit (hanya kelalaian).
⚠️ Tidak jelas apakah "kerugian" mencakup kerugian immateriil.
```

### 7.3 Layer 2: Systematic Interpretation (Penafsiran Sistematis)

**7.3.1 Definisi**
Penafsiran yang menempatkan peraturan dalam konteks sistem hukum secara keseluruhan, memperhatikan hubungan dengan peraturan lain, hierarki norma, dan prinsip-prinsip hukum yang berlaku.

**7.3.2 Metodologi**
- Pemetaan hubungan antar pasal dalam satu peraturan
- Pemetaan hubungan antar peraturan dalam satu bidang hukum
- Analisis hierarki norma (UU > PP > Perpres > Permen)
- Identifikasi asas-asas hukum yang mendasari
- Deteksi konflik norma (lex specialis, lex posterior)
- Analisis konsistensi dengan konstitusi

**7.3.3 Contoh Output**
```
LAPISAN 2: PENAFSIRAN SISTEMATIS
═══════════════════════════════════

Posisi dalam Hierarki Norma:
Undang-Undang (Hierarki ke-3 setelah UUD 1945 dan TAP MPR)
├── Lebih tinggi dari: PP, Perpres, Permen, Perda
└── Lebih rendah dari: UUD 1945, TAP MPR

Peraturan Terkait dalam Sistem Hukum:
┌─────────────────────────────────────────────────────────┐
│ UU No. 13 Tahun 2003 (Ketenagakerjaan)                  │
│ ├── Pasal 158: PHK dengan izin PHI                     │
│ ├── Pasal 160: PHK tanpa izin PHI = null and void      │
│ ├── Pasal 169: PHK oleh pekerja (resign)               │
│ └── Pasal 170: PHK karena pensiun                      │
├─────────────────────────────────────────────────────────┤
│ PP No. 35 Tahun 2021 (PKWT, Alih Daya, Waktu Kerja)    │
│ ├── Pasal 45: Kompensasi PHK                           │
│ └── Pasal 46: Uang Penghargaan                         │
├─────────────────────────────────────────────────────────┤
│ UU Cipta Kerja (UU No. 6 Tahun 2023)                   │
│ └── Mengubah beberapa pasal UU 13/2003                 │
└─────────────────────────────────────────────────────────┘

Konflik Norma yang Terdeteksi:
⚠️ KONFLIK: Pasal 158 UU 13/2003 vs Pasal 81 UU Cipta Kerja
    ├── UU 13/2003: PHK memerlukan izin PHI
    ├── UU Cipta Kerja: PHK langsung ke PHI tanpa izin
    └── RESOLUSI: Lex posterior derogat legi priori
        → UU Cipta Kerja yang berlaku (lebih baru)

Asas Hukum yang Mendasari:
✓ Pacta sunt servanda (perjanjian mengikat sebagai undang-undang)
✓ Itikad baik (good faith) dalam pelaksanaan perjanjian
✓ Kepastian hukum (legal certainty)
✓ Perlindungan bagi pihak yang lemah (workers protection)

Konsistensi dengan Konstitusi:
✓ Sesuai dengan Pasal 27 ayat (2) UUD 1945 (hak atas pekerjaan)
✓ Sesuai dengan Pasal 28D ayat (1) UUD 1945 (hak atas 
  pengakuan yang sama di depan hukum)
```

### 7.4 Layer 3: Teleological Interpretation (Penafsiran Teleologis)

**7.4.1 Definisi**
Penafsiran berdasarkan tujuan hukum (geest van de wet) yang ingin dicapai oleh pembentuk undang-undang, melampaui makna harfiah teks.

**7.4.2 Metodologi**
- Analisis konsiderans (menimbang) dalam peraturan
- Analisis penjelasan umum peraturan
- Analisis risalah pembahasan UU (jika tersedia)
- Identifikasi tujuan sosial-ekonomi peraturan
- Analisis dampak kebijakan (policy impact)
- Perbandingan dengan tujuan peraturan serupa di negara lain

**7.4.3 Contoh Output**
```
LAPISAN 3: PENAFSIRAN TELEOLOGIS
═══════════════════════════════════

Tujuan Hukum (Geest van de Wet):
Berdasarkan konsiderans "Menimbang" UU No. 13 Tahun 2003:
┌─────────────────────────────────────────────────────────┐
│ a. Bahwa pembangunan nasional bertujuan untuk             │
│    mewujudkan masyarakat adil dan makmur...               │
│ b. Bahwa pembangunan ketenagakerjaan bertujuan untuk...  │
│    melindungi tenaga kerja...                            │
│ c. Bahwa untuk melindungi tenaga kerja, diperlukan...    │
│    pengaturan yang jelas...                              │
└─────────────────────────────────────────────────────────┘

→ TUJUAN UTAMA: Perlindungan tenaga kerja sebagai pihak 
  yang lebih lemah dalam hubungan kerja

→ TUJUAN KHUSUS: 
  1. Kepastian hukum dalam hubungan kerja
  2. Keadilan sosial bagi pekerja
  3. Pencegahan eksploitasi tenaga kerja

Analisis Penjelasan Umum:
"Undang-undang ini bertujuan untuk memberikan perlindungan 
yang maksimal bagi tenaga kerja, termasuk dalam hal 
pemutusan hubungan kerja, dengan memastikan bahwa PHK 
hanya dapat dilakukan dengan alasan yang sah dan melalui 
proses yang adil."

Risalah Pembahasan (jika tersedia):
Dalam pembahasan DPR, frasa "izin PHI" ditambahkan untuk 
mencegah PHK sepihak oleh pengusaha tanpa pemeriksaan 
apakah alasan PHK tersebut sah secara hukum.

Perbandingan Hukum Komparatif:
┌─────────────────────────────────────────────────────────┐
│ Negara        │ Regulasi PHK         │ Tujuan          │
├───────────────┼──────────────────────┼─────────────────┤
│ Indonesia     │ UU 13/2003           │ Perlindungan    │
│               │ (izin PHI)           │ pekerja         │
├───────────────┼──────────────────────┼─────────────────┤
│ Jerman        │ KSchG                │ Social justice  │
│               │ (social selection)   │ in employment   │
├───────────────┼──────────────────────┼─────────────────┤
│ Prancis       │ Code du Travail      │ Protectionnisme │
│               │ (autorisation        │ des travailleurs│
│               │  administrative)     │                 │
└─────────────────────────────────────────────────────────┘

Dampak Kebijakan:
✓ Positif: Mencegah PHK sewenang-wenang
✓ Positif: Memberikan kepastian hukum bagi pekerja
⚠️ Negatif: Dapat memperlambat proses PHK yang sah
⚠️ Negatif: Beban administratif bagi pengusaha
```

### 7.5 Layer 4: Sociological Interpretation (Penafsiran Sosiologis)

**7.5.1 Definisi**
Penafsiran yang mempertimbangkan realitas sosial, budaya, ekonomi, dan dampak praktis dari penerapan hukum dalam masyarakat.

**7.5.2 Metodologi**
- Analisis dampak sosial dari penerapan hukum
- Pertimbangan norma sosial dan budaya setempat
- Analisis realitas ekonomi para pihak
- Pertimbangan akses terhadap keadilan
- Analisis power dynamics antar pihak
- Pertimbangan hukum adat dan kebiasaan

**7.5.3 Contoh Output**
```
LAPISAN 4: PENAFSIRAN SOSIOLOGIS
═══════════════════════════════════

Konteks Sosial:
Lokasi: Bali, Indonesia
├── Masyarakat adat masih kuat
├── Sistem kasta (wangsa) masih berpengaruh
├── Tanah ulayat memiliki nilai sakral
└── Hukum adat waris berlaku alongside hukum negara

Norma Budaya yang Relevan:
┌─────────────────────────────────────────────────────────┐
│ 1. Sistem Waris Bali (Purusa/Pradana)                    │
│    → Anak laki-laki (purusa) mewarisi harta keluarga    │
│    → Anak perempuan (pradana) mendapat "igel-igel"      │
│    ⚠️ BERTENTANGAN dengan UU 1/1974 tentang perkawinan │
│                                                         │
│ 2. Tanah Pusaka                                        │
│    → Tidak boleh dijual ke luar keluarga                │
│    → Harus tetap dalam garis keturunan                  │
│    ⚠️ Dapat membatasi hak milik perorangan             │
└─────────────────────────────────────────────────────────┘

Realitas Ekonomi Para Pihak:
┌─────────────────────────────────────────────────────────┐
│ Penggugat (Pekerja)                                      │
│ ├── Pendapatan: Rp 5.000.000/bulan                      │
│ ├── Tanggungan: Istri + 2 anak                          │
│ ├── Aset: Tidak ada                                     │
│ └── Akses hukum: Terbatas (butuh bantuan hukum)         │
├─────────────────────────────────────────────────────────┤
│ Tergugat (Pengusaha)                                     │
│ ├── Pendapatan: Rp 500.000.000/bulan                    │
│ ├── Tim hukum: 5 orang advokat                          │
│ ├── Aset: Properti, kendaraan, deposito                 │
│ └── Akses hukum: Sangat baik                            │
└─────────────────────────────────────────────────────────┘

Dinamika Kekuasaan:
⚠️ ASIMETRI KUASA SIGNIFIKAN
   ├── Pengusaha memiliki posisi tawar yang jauh lebih kuat
   ├── Pekerja bergantung pada pengusaha untuk mata pencaharian
   ├── Risiko intimidasi atau tekanan sosial terhadap pekerja
   └── REKOMENDASI: Perlindungan ekstra bagi pekerja

Akses terhadap Keadilan:
⚠️ HAMBATAN:
   ├── Biaya perkara (Rp 5-10 juta) = 1-2 bulan gaji pekerja
   ├── Jarak ke PHI (harus ke kota besar)
   ├── Waktu proses (6-12 bulan)
   └── Kompleksitas hukum (butuh advokat)

✓ FASILITAS:
   ├── Prodeo (perkara cuma-cuma) tersedia
   ├── Posbakum di pengadilan
   └── Bantuan hukum dari LBH

Rekomendasi Sosiologis:
Dalam kasus ini, penerapan hukum harus mempertimbangkan:
1. Posisi tawar yang tidak seimbang → Perlindungan ekstra 
   bagi pekerja
2. Realitas ekonomi pekerja → Kompensasi harus realistis 
   dan dapat dieksekusi
3. Akses terhadap keadilan → Pertimbangkan mekanisme 
   alternatif (mediasi)
4. Norma budaya lokal → Jika melibatkan tanah adat, 
   konsultasikan dengan tetua adat
```

### 7.6 Layer 5: Historical Interpretation (Penafsiran Historis)

**7.6.1 Definisi**
Penafsiran yang mempertimbangkan sejarah legislasi, evolusi norma hukum, dan maksud asli pembentuk undang-undang (original intent).

**7.6.2 Metodologi**
- Analisis sejarah legislasi (legislative history)
- Pelacakan evolusi norma dari waktu ke waktu
- Analisis perubahan sosial yang mempengaruhi perubahan hukum
- Perbandingan dengan peraturan terdahulu
- Analisis risalah DPR/DPA (jika tersedia)

**7.6.3 Contoh Output**
```
LAPISAN 5: PENAFSIRAN HISTORIS
═══════════════════════════════════

Evolusi Norma:
┌─────────────────────────────────────────────────────────┐
│ Tahun │ Peraturan              │ Perubahan Utama        │
├───────┼────────────────────────┼────────────────────────┤
│ 1847  │ KUHPerdata (BW)        │ Pasal 1233: Perikatan│
│       │                        │ lahir dari UU/sepakat│
├───────┼────────────────────────┼────────────────────────┤
│ 1948  │ UU Darurat No.16/1951  │ Pengaturan hubungan  │
│       │                        │ kerja pertama        │
├───────┼────────────────────────┼────────────────────────┤
│ 1964  │ UU No.14/1964          │ Penyesuaian pasca    │
│       │                        │ kemerdekaan          │
├───────┼────────────────────────┼────────────────────────┤
│ 2003  │ UU No.13/2003          │ Reformasi menyeluruh │
│       │ (Ketenagakerjaan)      │ perlindungan pekerja │
├───────┼────────────────────────┼────────────────────────┤
│ 2020  │ UU Cipta Kerja         │ Fleksibilitas pasar  │
│       │ (UU No.6/2023)         │ kerja ditingkatkan   │
└─────────────────────────────────────────────────────────┘

Risalah Pembahasan UU No. 13 Tahun 2003:
Dari risalah DPR (2002-2003):
"Pasal 158 ini kami rumuskan dengan ketat karena 
pengalaman menunjukkan bahwa banyak pekerja di-PHK 
sepihak oleh pengusaha tanpa alasan yang sah. Dengan 
kewajiban izin PHI, kami ingin memastikan bahwa PHK 
hanya dilakukan setelah melalui pemeriksaan yang adil."

→ ORIGINAL INTENT: Mencegah PHK sepihak, melindungi 
  pekerja dari kesewenang-wenangan pengusaha

Perubahan Sosial yang Mempengaruhi:
┌─────────────────────────────────────────────────────────┐
│ 2003 (Pengesahan UU 13/2003)                            │
│ ├── Ekonomi: Pasca krisis 1998, pemulihan ekonomi       │
│ ├── Sosial: Demokratisasi, penguatan hak pekerja        │
│ └── Politik: Reformasi, kebebasan berserikat            │
├─────────────────────────────────────────────────────────┤
│ 2020 (UU Cipta Kerja)                                   │
│ ├── Ekonomi: Perlu investasi, fleksibilitas pasar kerja │
│ ├── Sosial: Gig economy, startup, digital economy       │
│ └── Politik: Omnibus law, deregulasi                    │
└─────────────────────────────────────────────────────────┘

Relevansi Historis untuk Kasus Ini:
✓ Maksud asli UU 13/2003 mendukung posisi pekerja
⚠️ UU Cipta Kerja menggeser keseimbangan ke arah 
   fleksibilitas pengusaha
→ INTERPRETASI: Harus hati-hati dalam menerapkan pasal 
  yang telah diubah oleh UU Cipta Kerja
```

### 7.7 Layer 6: Comparative Interpretation (Penafsiran Komparatif)

**7.7.1 Definisi**
Penafsiran yang membandingkan dengan yurisprudensi, putusan pengadilan, dan praktik hukum serupa, baik di Indonesia maupun di negara lain.

**7.7.2 Metodologi**
- Analisis yurisprudensi MA dan pengadilan bawahannya
- Perbandingan dengan putusan pengadilan serupa
- Analisis putusan Mahkamah Konstitusi yang relevan
- Perbandingan dengan hukum negara lain (comparative law)
- Identifikasi tren putusan (judicial trend)

**7.7.3 Contoh Output**
```
LAPISAN 6: PENAFSIRAN KOMPARATIF
═══════════════════════════════════

Yurisprudensi MA yang Relevan:
┌─────────────────────────────────────────────────────────┐
│ 1. Putusan MA No. 3155 K/Pdt/1984                      │
│    ├── Status: Yurisprudensi Tetap                      │
│    ├── Bobot: ★★★★★ (Sangat Tinggi)                     │
│    ├── Ratio Decidendi: Wanprestasi terjadi apabila     │
│    │   debitur tidak memenuhi kewajiban sesuai perjanjian│
│    └── Relevansi: Langsung relevan dengan kasus ini     │
├─────────────────────────────────────────────────────────┤
│ 2. Putusan MA No. 2721 K/Pdt/1999                      │
│    ├── Status: Yurisprudensi                            │
│    ├── Bobot: ★★★★☆ (Tinggi)                            │
│    ├── Ratio Decidendi: Ganti rugi mencakup kerugian   │
│    │   materiil dan immateriil                          │
│    └── Relevansi: Relevan untuk perhitungan ganti rugi  │
├─────────────────────────────────────────────────────────┤
│ 3. Putusan MA No. 1504 K/Pdt/2015                      │
│    ├── Status: Putusan MA (belum yurisprudensi tetap)   │
│    ├── Bobot: ★★★☆☆ (Sedang)                            │
│    ├── Ratio Decidendi: Force majeure dapat免除         │
│    │   tanggung jawab wanprestasi                       │
│    └── Relevansi: Potensi pembelaan untuk tergugat      │
└─────────────────────────────────────────────────────────┘

Tren Putusan (Judicial Trend):
┌─────────────────────────────────────────────────────────┐
│ Analisis 100 putusan MA terkait wanprestasi (2015-2025) │
├─────────────────────────────────────────────────────────┤
│ Tren 1: Peningkatan pengakuan kerugian immateriil       │
│ ├── 2015: 15% putusan mengakui kerugian immateriil      │
│ ├── 2020: 35% putusan mengakui kerugian immateriil      │
│ └── 2025: 60% putusan mengakui kerugian immateriil      │
│ → PELUANG: Gugatan kerugian immateriil semakin diterima │
├─────────────────────────────────────────────────────────┤
│ Tren 2: Pengetatan pembuktian force majeure             │
│ ├── 2015: 40% pembelaan force majeure diterima          │
│ ├── 2020: 25% pembelaan force majeure diterima          │
│ └── 2025: 15% pembelaan force majeure diterima          │
│ → RISIKO: Pembelaan force majeure semakin sulit         │
└─────────────────────────────────────────────────────────┘

Perbandingan Hukum Asing:
┌─────────────────────────────────────────────────────────┐
│ Belanda (Nederland)                                      │
│ ├── BW Artikel 6:74: Wanprestasi                       │
│ ├── Putusan HR 19 Mei 1967 (NJ 1968, 32)               │
│ └── Prinsip: Schuldnerschap = tanggung jawab mutlak    │
├─────────────────────────────────────────────────────────┤
│ Jerman (Deutschland)                                     │
│ ├── BGB § 280: Schadensersatz bei Pflichtverletzung    │
│ ├── Putusan BGH VIII ZR 26/14                          │
│ └── Prinsip: Verschulden (kesalahan) diperlukan        │
├─────────────────────────────────────────────────────────┤
│ Singapura (Common Law)                                   │
│ ├── Contract Act (berbasis English law)                │
│ ├── Putusan SGCA 2014: "The law does not require       │
│ │   perfection, but it does require compliance"         │
│ └── Prinsip: Strict liability dalam kontrak komersial  │
└─────────────────────────────────────────────────────────┘

Ratio Decidendi yang Diekstrak:
Dari analisis yurisprudensi, ratio decidendi yang konsisten:
1. Wanprestasi = tidak memenuhi kewajiban sesuai perjanjian
2. Ganti rugi = kerugian riil + keuntungan yang seharusnya 
   diperoleh + kerugian immateriil (tren meningkat)
3. Force majeure = peristiwa di luar kendali, tidak dapat 
   diduga, dan tidak dapat diatasi (pembuktian ketat)
4. Beban pembuktian = pada pihak yang mengklaim wanprestasi
```

### 7.8 Layer 7: Critical Interpretation (Penafsiran Kritis)

**7.8.1 Definisi**
Penafsiran kritis yang secara aktif mencari kelemahan, risiko, dan alternatif interpretasi. Lapisan ini berfungsi sebagai "devil's advocate" yang menguji kekuatan argumen.

**7.8.2 Metodologi**
- Identifikasi kelemahan dalam interpretasi sebelumnya
- Analisis risiko dari setiap interpretasi
- Pencarian alternatif interpretasi yang mungkin
- Stress test terhadap argumen
- Devil's advocate approach
- Pre-mortem analysis

**7.8.3 Contoh Output**
```
LAPISAN 7: PENAFSIRAN KRITIS
═══════════════════════════════════

Kelemahan dalam Interpretasi Sebelumnya:
┌─────────────────────────────────────────────────────────┐
│ 1. Kelemahan Gramatikal                                  │
│    ├── Istilah "kesalahan" ambigu                        │
│    ├── Tidak jelas apakah mencakup force majeure         │
│    └── RISIKO: ⚠️ TINGGI - Dapat diperdebatkan         │
├─────────────────────────────────────────────────────────┤
│ 2. Kelemahan Sistematis                                  │
│    ├── Konflik dengan UU Cipta Kerja belum diselesaikan  │
│    ├── Hierarki norma tidak jelas untuk kasus ini        │
│    └── RISIKO: ⚠️ SEDANG - Perlu klarifikasi MA        │
├─────────────────────────────────────────────────────────┤
│ 3. Kelemahan Teleologis                                  │
│    ├── Tujuan perlindungan pekerja vs fleksibilitas      │
│    │   pengusaha belum seimbang                          │
│    └── RISIKO: ⚠️ TINGGI - Tergantung kebijakan MA     │
└─────────────────────────────────────────────────────────┘

Analisis Risiko:
┌─────────────────────────────────────────────────────────┐
│ Risiko 1: Putusan MA Berubah                             │
│ ├── Probabilitas: 25%                                    │
│ ├── Dampak: TINGGI (mengubah dasar hukum)               │
│ └── Mitigasi: Siapkan argumen alternatif                 │
├─────────────────────────────────────────────────────────┤
│ Risiko 2: Lawan Mengajukan Judicial Review               │
│ ├── Probabilitas: 10%                                    │
│ ├── Dampak: SANGAT TINGGI (menunda proses)              │
│ └── Mitigasi: Pastikan konstitusionalitas argumen        │
├─────────────────────────────────────────────────────────┤
│ Risiko 3: Hakim Menafsirkan Berbeda                      │
│ ├── Probabilitas: 40%                                    │
│ ├── Dampak: SEDANG (dapat mempengaruhi hasil)           │
│ └── Mitigasi: Siapkan yurisprudensi pendukung            │
└─────────────────────────────────────────────────────────┘

Alternatif Interpretasi:
┌─────────────────────────────────────────────────────────┐
│ Alternatif A: Interpretasi Restriktif                    │
│ ├── "Kesalahan" = hanya kelalaian, bukan force majeure  │
│ ├── Keuntungan: Kepastian hukum tinggi                  │
│ ├── Kerugian: Tidak fleksibel untuk kasus kompleks      │
│ └── Probabilitas Diterima Hakim: 60%                    │
├─────────────────────────────────────────────────────────┤
│ Alternatif B: Interpretasi Ekstensif                     │
│ ├── "Kesalahan" = termasuk force majeure                │
│ ├── Keuntungan: Fleksibel, mencakup berbagai situasi    │
│ ├── Kerugian: Ketidakpastian hukum                      │
│ └── Probabilitas Diterima Hakim: 30%                    │
├─────────────────────────────────────────────────────────┤
│ Alternatif C: Interpretasi Teleologis                    │
│ ├── "Kesalahan" = dinilai berdasarkan tujuan hukum      │
│ ├── Keuntungan: Adil, kontekstual                       │
│ ├── Kerugian: Subjektivitas tinggi                      │
│ └── Probabilitas Diterima Hakim: 50%                    │
└─────────────────────────────────────────────────────────┘

Devil's Advocate Analysis:
"Jika saya adalah advokat lawan, saya akan menyerang dengan:"
┌─────────────────────────────────────────────────────────┐
│ Serangan 1: Ambiguitas Istilah                           │
│ ├── Argumen: "Kesalahan" tidak didefinisikan dalam UU   │
│ ├── Dasar: Prinsip nullum crimen sine lege              │
│ └── Kekuatan: ★★★★☆                                    │
├─────────────────────────────────────────────────────────┤
│ Serangan 2: Perubahan Regulasi                           │
│ ├── Argumen: UU Cipta Kerja mengubah keseimbangan       │
│ ├── Dasar: Lex posterior derogat legi priori            │
│ └── Kekuatan: ★★★★★                                    │
├─────────────────────────────────────────────────────────┤
│ Serangan 3: Force Majeure                                │
│ ├── Argumen: Pandemi = force majeure                    │
│ ├── Dasar: Pasal 1244-1245 KUHPerdata                   │
│ └── Kekuatan: ★★★☆☆                                    │
└─────────────────────────────────────────────────────────┘

Pre-Mortem Analysis:
"Skenario di mana kita KALAH di pengadilan:"
┌─────────────────────────────────────────────────────────┐
│ Skenario 1: Hakim Menerima Argumen Force Majeure        │
│ ├── Penyebab: Pandemi dianggap force majeure yang valid │
│ ├── Probabilitas: 30%                                    │
│ └── Pencegahan: Buktikan bahwa tergugat masih bisa      │
│   memenuhi kewajiban dengan upaya reasonable            │
├─────────────────────────────────────────────────────────┤
│ Skenario 2: Hakim Mengutamakan UU Cipta Kerja           │
│ ├── Penyebab: Fleksibilitas pasar kerja diutamakan      │
│ ├── Probabilitas: 20%                                    │
│ └── Pencegahan: Tunjukkan bahwa kasus ini tidak         │
│   melibatkan fleksibilitas pasar kerja                  │
├─────────────────────────────────────────────────────────┤
│ Skenario 3: Pembuktian Tidak Memenuhi Standar           │
│ ├── Penyebab: Bukti tidak cukup meyakinkan hakim        │
│ ├── Probabilitas: 25%                                    │
│ └── Pencegahan: Perkuat pembuktian dengan saksi dan     │
│   dokumen tambahan                                      │
└─────────────────────────────────────────────────────────┘

Rekomendasi Kritis:
1. ✓ Gunakan Alternatif A (interpretasi restriktif) sebagai 
   argumen utama
2. ✓ Siapkan Alternatif C (teleologis) sebagai cadangan
3. ✓ Antisipasi serangan force majeure dengan bukti konkret
4. ✓ Perkuat dengan yurisprudensi MA terbaru
5. ⚠️ Hindari mengandalkan satu dasar hukum saja
```

### 7.9 Integrasi 7 Lapisan Penalaran

**7.9.1 Output Sintesis Final**
```
SINTESIS 7 LAPISAN PENAFSIRAN
═══════════════════════════════════

Konsensus Antar Lapisan:
✓ Lapisan 1, 2, 3 sepakat: Wanprestasi telah terjadi
✓ Lapisan 4, 5, 6 sepakat: Perlindungan pekerja diutamakan
⚠️ Lapisan 7 memperingatkan: Risiko force majeure

Konflik Antar Lapisan:
⚠️ Lapisan 2 vs Lapisan 5:
   ├── Sistematis: UU 13/2003 yang berlaku
   ├── Historis: UU Cipta Kerja mengubah beberapa pasal
   └── RESOLUSI: Lex posterior → UU Cipta Kerja

Bobot Setiap Lapisan (berdasarkan konteks kasus):
┌─────────────────────────────────────────────────────────┐
│ Lapisan                  │ Bobot │ Alasan               │
├──────────────────────────┼───────┼──────────────────────┤
│ 1. Gramatikal            │ 15%   │ Dasar interpretasi   │
│ 2. Sistematis            │ 20%   │ Hierarki norma       │
│ 3. Teleologis            │ 15%   │ Tujuan hukum         │
│ 4. Sosiologis            │ 10%   │ Konteks sosial       │
│ 5. Historis              │ 10%   │ Evolusi norma        │
│ 6. Komparatif            │ 20%   │ Yurisprudensi        │
│ 7. Kritis                │ 10%   │ Validasi argumen     │
└─────────────────────────────────────────────────────────┘

Interpretasi Final:
Berdasarkan sintesis 7 lapisan penafsiran:

1. SECARA GRAMATIKAL: Pasal 1243 KUHPerdata mewajibkan 
   debitur yang lalai memenuhi kewajiban untuk mengganti 
   kerugian.

2. SECARA SISTEMATIS: Dalam hierarki norma, UU 13/2003 
   (sebagaimana diubah UU Cipta Kerja) merupakan lex 
   specialis terhadap KUHPerdata.

3. SECARA TELEOLOGIS: Tujuan hukum adalah melindungi 
   pekerja dari PHK sepihak, namun juga memberikan 
   fleksibilitas bagi pengusaha dalam situasi tertentu.

4. SECARA SOSIOLOGIS: Dalam konteks kasus ini, posisi 
   tawar pekerja lebih lemah, sehingga diperlukan 
   perlindungan ekstra.

5. SECARA HISTORIS: Evolusi norma menunjukkan pergeseran 
   dari perlindungan ketat (UU 13/2003) ke fleksibilitas 
   (UU Cipta Kerja).

6. SECARA KOMPARATIF: Yurisprudensi MA konsisten dalam 
   mengakui wanprestasi apabila debitur lalai memenuhi 
   kewajiban tanpa alasan yang sah.

7. SECARA KRITIS: Risiko utama adalah pembelaan force 
   majeure oleh tergugat, namun tren putusan menunjukkan 
   pembuktian force majeure semakin ketat.

KESIMPULAN FINAL:
Berdasarkan sintesis 7 lapisan penafsiran, wanprestasi 
telah terjadi. Tergugat tidak dapat mengandalkan force 
majeure karena pandemi COVID-19 tidak secara langsung 
menghalangi pelaksanaan kewajiban (masih dapat dilakukan 
secara online/remote). Oleh karena itu, tergugat wajib 
mengganti kerugian sesuai Pasal 1246 KUHPerdata.

TINGKAT KEPASTIAN: 0.78 (Kuat)
RISIKO UTAMA: Pembelaan force majeure (probabilitas 25%)
REKOMENDASI: Perkuat pembuktian bahwa kewajiban dapat 
dilaksanakan secara remote selama pandemi.
```

---

## 8. NORMATIVE CONFLICT RESOLUTION ENGINE

### 8.1 Konsep

PAUGERAN wajib mendeteksi dan menyelesaikan konflik antar peraturan secara sistematis menggunakan asas-asas hukum yang telah diakui secara universal.

### 8.2 Asas-Asas yang Digunakan

**8.2.1 Lex Superior Derogat Legi Inferiori**
Hukum yang lebih tinggi mengesampingkan hukum yang lebih rendah.

**Hierarki Peraturan di Indonesia:**
1. UUD 1945
2. Ketetapan MPR
3. Undang-Undang / Perppu
4. Peraturan Pemerintah
5. Peraturan Presiden
6. Peraturan Daerah Provinsi
7. Peraturan Daerah Kabupaten/Kota

**8.2.2 Lex Specialis Derogat Legi Generali**
Hukum yang lebih khusus mengesampingkan hukum yang lebih umum.

**8.2.3 Lex Posterior Derogat Legi Priori**
Hukum yang lebih baru mengesampingkan hukum yang lebih lama.

### 8.3 Implementasi Teknis

```rust
pub struct NormConflictResolver {
    hierarchy_db: Arc<NormHierarchyDB>,
    conflict_detector: Arc<ConflictDetector>,
}

impl NormConflictResolver {
    pub async fn resolve(
        &self,
        provision_a: &LegalProvision,
        provision_b: &LegalProvision,
    ) -> Result<ConflictResolution> {
        // 1. Detect conflict type
        let conflict_type = self.conflict_detector.detect(provision_a, provision_b).await?;
        
        // 2. Apply resolution principles
        let resolution = match conflict_type {
            ConflictType::Hierarchical => {
                self.apply_lex_superior(provision_a, provision_b).await?
            }
            ConflictType::SpecialGeneral => {
                self.apply_lex_specialis(provision_a, provision_b).await?
            }
            ConflictType::Temporal => {
                self.apply_lex_posterior(provision_a, provision_b).await?
            }
            ConflictType::Complex => {
                self.apply_complex_resolution(provision_a, provision_b).await?
            }
        };
        
        Ok(resolution)
    }
}
```

### 8.4 Output Format

```
RESOLUSI KONFLIK NORMA
═══════════════════════════════════

Konflik yang Terdeteksi:
┌─────────────────────────────────────────────────────────┐
│ Peraturan A: Pasal 158 UU No. 13 Tahun 2003             │
│ ├── Isi: PHK memerlukan izin PHI                        │
│ ├── Tanggal: 2003-03-25                                 │
│ └── Hierarki: Undang-Undang                             │
├─────────────────────────────────────────────────────────┤
│ Peraturan B: Pasal 81 UU No. 6 Tahun 2023 (Cipta Kerja) │
│ ├── Isi: PHK langsung ke PHI tanpa izin                 │
│ ├── Tanggal: 2023-01-16                                 │
│ └── Hierarki: Undang-Undang                             │
└─────────────────────────────────────────────────────────┘

Jenis Konflik: LEX POSTERIOR DEROGAT LEGI PRIORI
(Peraturan yang lebih baru mengesampingkan yang lebih lama)

Analisis:
✓ Kedua peraturan memiliki hierarki yang sama (UU)
✓ Peraturan B (2023) lebih baru dari Peraturan A (2003)
✓ Tidak ada ketentuan peralihan yang mempertahankan 
  Peraturan A
✓ Penjelasan UU Cipta Kerja secara eksplisit menyatakan 
  perubahan prosedur PHK

Resolusi:
→ Peraturan B (UU Cipta Kerja) yang BERLAKU
→ Peraturan A (UU 13/2003 Pasal 158) TIDAK BERLAKU 
  untuk kasus ini

Rekomendasi:
Gunakan prosedur PHK sesuai UU Cipta Kerja: langsung 
ajukan ke PHI tanpa memerlukan izin sebelumnya.

Catatan:
⚠️ Untuk kasus yang terjadi SEBELUM berlakunya UU Cipta 
  Kerja (sebelum 2020), gunakan UU 13/2003 (lex temporis)
```

---

## 9. TEMPORAL LAW APPLICATION SYSTEM

### 9.1 Konsep

PAUGERAN wajib menerapkan hukum yang berlaku pada saat peristiwa hukum terjadi, bukan hukum yang berlaku saat ini, kecuali ada ketentuan yang secara eksplisit menyatakan berlaku surut.

### 9.2 Metodologi

**9.2.1 Identifikasi Tanggal Kejadian Kritis**
- Tanggal perbuatan hukum dilakukan
- Tanggal perjanjian dibuat
- Tanggal wanprestasi terjadi
- Tanggal kerugian diderita

**9.2.2 Pelacakan Peraturan yang Berlaku**
- Cari peraturan yang berlaku pada tanggal kejadian
- Periksa apakah peraturan tersebut masih berlaku
- Periksa apakah ada perubahan/amandemen
- Periksa apakah ada aturan peralihan

**9.2.3 Penerapan Hukum yang Tepat**
- Gunakan hukum yang berlaku pada saat kejadian
- Jangan gunakan hukum baru untuk peristiwa lama (kecuali retroaktif eksplisit)
- Pertimbangkan aturan peralihan

### 9.3 Implementasi Teknis

```rust
pub struct TemporalLawApplier {
    regulation_tracker: Arc<RegulationTracker>,
    transition_rule_db: Arc<TransitionRuleDB>,
}

impl TemporalLawApplier {
    pub async fn apply(
        &self,
        event_date: DateTime<Utc>,
        legal_issue: &LegalIssue,
    ) -> Result<TemporalApplication> {
        // 1. Find regulations in effect on event date
        let applicable_regulations = self.regulation_tracker
            .find_by_date(event_date, legal_issue).await?;
        
        // 2. Check for transition rules
        let transition_rules = self.transition_rule_db
            .find_applicable(event_date, legal_issue).await?;
        
        // 3. Apply temporal principles
        let application = self.apply_temporal_principles(
            event_date,
            applicable_regulations,
            transition_rules,
        ).await?;
        
        Ok(application)
    }
}
```

### 9.4 Output Format

```
PENERAPAN HUKUM BERDASARKAN WAKTU (LEX TEMPORIS)
═══════════════════════════════════════════════════

Tanggal Kejadian Kritis: 15 Maret 2018

Peraturan yang Berlaku pada Tanggal Tersebut:
┌─────────────────────────────────────────────────────────┐
│ ✓ UU No. 13 Tahun 2003 (Ketenagakerjaan)               │
│   ├── Berlaku sejak: 2003-03-25                         │
│   ├── Status pada 2018-03-15: BERLAKU                   │
│   └── Relevansi: Langsung relevan                       │
├─────────────────────────────────────────────────────────┤
│ ✗ UU No. 6 Tahun 2023 (Cipta Kerja)                    │
│   ├── Berlaku sejak: 2023-01-16                         │
│   ├── Status pada 2018-03-15: BELUM BERLAKU             │
│   └── Relevansi: TIDAK RELEVAN (peristiwa sebelum       │
│       UU ini disahkan)                                  │
├─────────────────────────────────────────────────────────┤
│ ✓ PP No. 1 Tahun 2022                                  │
│   ├── Berlaku sejak: 2022-01-01                         │
│   ├── Status pada 2018-03-15: BELUM BERLAKU             │
│   └── Relevansi: TIDAK RELEVAN                          │
└─────────────────────────────────────────────────────────┘

Aturan Peralihan yang Relevan:
Tidak ada aturan peralihan khusus untuk kasus ini.

Hukum yang Diterapkan:
→ UU No. 13 Tahun 2003 (Ketenagakerjaan)
→ Pasal 158, 160, 169

Catatan Penting:
⚠️ Meskipun UU Cipta Kerja telah mengubah beberapa pasal 
  UU 13/2003, perubahan tersebut TIDAK BERLAKU SURUT.
⚠️ Untuk peristiwa yang terjadi pada 2018, gunakan UU 
  13/2003 versi asli (sebelum diubah UU Cipta Kerja).
```

---

## 10. JUDICIAL DISCRETION MODELING

### 10.1 Konsep
PAUGERAN wajib memodelkan dan mempertimbangkan kebebasan hakim dalam menilai bukti (*vrije bewijswaardering*) sesuai Pasal 1865 HIR / 163 RBg, serta tidak memberikan kepastian semu berdasarkan kekuatan bukti formil semata.

### 10.2 Metodologi
- Analisis kekuatan pembuktian (Alat bukti surat > Saksi > Persangkaan > Pengakuan > Sumpah).
- Evaluasi kredibilitas dan kepentingan saksi.
- Identifikasi celah dalam rantai pembuktian.
- Simulasi penilaian hakim berdasarkan tren yurisprudensi terkini.

### 10.3 Output Format
```
ANALISIS KELELUASAAN HAKIM (VRIJE BEWIJSWAARDERING)
════════════════════════════════════════════════════
Bukti Utama: Surat Perjanjian Bawah Tangan
⚠️ Risiko: Pihak lawan dapat menyangkal keaslian tanda tangan.
   → Jika disangkal, beban pembuktian beralih ke Penggugat 
     untuk membuktikan keaslian (Pasal 1875 KUHPerdata).
   → Rekomendasi: Siapkan saksi atau bukti tambahan (misal: 
     riwayat transfer bank yang konsisten dengan perjanjian).

Tren Penilaian Hakim (PHI):
Dalam 80% kasus serupa, hakim cenderung memberikan bobot 
lebih tinggi pada bukti objektif (transfer bank) dibandingkan 
kesaksian lisan semata.
```

---

## 11. CAUSATION & REMOTENESS ANALYSIS

### 11.1 Konsep
PAUGERAN wajib menganalisis hubungan kausalitas (*causal link*) antara perbuatan dan kerugian, serta mendeteksi apakah kerugian tersebut merupakan akibat langsung (*proximate cause*) atau akibat yang terlalu jauh (*remote cause*).

### 11.2 Metodologi
- Uji "But-For" (Sine qua non): Apakah kerugian terjadi *jika tidak* ada perbuatan tersebut?
- Uji Foreseeability: Apakah kerugian tersebut dapat diduga secara wajar pada saat perjanjian dibuat/perbuatan dilakukan?
- Intervening Cause: Apakah ada peristiwa baru yang memutus rantai kausalitas?

### 11.3 Output Format
```
ANALISIS KAUSALITAS & REMOTENESS
════════════════════════════════════════════════════
Rantai Kausalitas:
Perbuatan (Keterlambatan Pengiriman) 
  → Akibat Langsung (Proyek terlambat 1 minggu) [✓ Proximate Cause]
  → Akibat Lanjutan (Klien membatalkan kontrak senilai Rp 10 M) [⚠️ Remote Cause?]

Evaluasi Foreseeability:
⚠️ Apakah kerugian Rp 10 M dapat diduga secara wajar saat kontrak dibuat?
  → Jika TIDAK: Gugatan ganti rugi sebesar Rp 10 M berisiko tinggi 
     ditolak atau dikurangi oleh pengadilan (Pasal 1247 KUHPerdata).
  → Rekomendasi: Fokus gugatan pada denda keterlambatan yang 
     tercantum eksplisit dalam kontrak.
```

---

## 12. UNWRITTEN LAW & CUSTOMARY LAW INTEGRATION

### 12.1 Konsep
PAUGERAN wajib mempertimbangkan asas-asas hukum tidak tertulis, hukum adat, dan kebiasaan (*customary law*) yang berlaku, terutama untuk kasus agraria, waris, dan bisnis keluarga.

### 12.2 Metodologi
- Deteksi kata kunci lokasi/suku/budaya dalam fakta.
- Pencarian di modul Hukum Adat (misal: Hukum Waris Bali, Tanah Ulayat Minangkabau).
- Penyeimbangan antara Hukum Negara (UU) dan Hukum Adat (dengan prinsip Hukum Negara sebagai *lex superior*, namun Hukum Adat sebagai *living law* yang diakui Pasal 18B UUD 1945).

### 12.3 Output Format
```
INTEGRASI HUKUM ADAT & TIDAK TERTULIS
════════════════════════════════════════════════════
Konteks: Sengketa Waris di Yogyakarta
✓ Asas Hukum Adat: Sistem Kewarisan Bilateral (mayoritas) atau 
  Parental, di mana anak laki-laki dan perempuan mendapat bagian.
⚠️ Konflik dengan KUHPerdata: KUHPerdata menganut sistem 
  individual, namun dalam praktik, Pengadilan Negeri di Yogyakarta 
  sering mempertimbangkan hukum adat setempat sebagai hukum yang 
  hidup (living law).
→ Rekomendasi: Siapkan bukti penerapan hukum adat dalam keluarga 
  tersebut (misal: musyawarah keluarga sebelumnya).
```

---

## 13. ETHICAL GUARDRAILS SYSTEM

### 13.1 Konsep
PAUGERAN wajib memastikan bahwa setiap strategi hukum yang disarankan sesuai dengan Kode Etik Advokat Indonesia (PERADI) dan tidak menyarankan taktik yang bersifat *frivolous* (mengada-ada), *vexatious* (mengganggu), atau melanggar etika profesi.

### 13.2 Metodologi
- Filter kata kunci taktik ilegal (misal: "memalsukan bukti", "menyuap", "mengulur waktu tanpa dasar").
- Validasi strategi terhadap Pasal-pasal Kode Etik Advokat (misal: kewajiban menjaga rahasia klien, larangan konflik kepentingan).

### 13.3 Output Format
```
PERINGATAN ETIKA PROFESI (ETHICAL GUARDRAIL)
════════════════════════════════════════════════════
⛔ STRATEGI DITOLAK: "Ajukan gugatan hanya untuk menekan lawan 
   agar setuju mediasi, tanpa niat benar-benar berperkara."
→ Alasan: Melanggar Kode Etik Advokat Pasal X (Penyalahgunaan 
   proses hukum / Abuse of Process).
→ Alternatif yang Diizinkan: Kirimkan Somasi yang tegas dengan 
   tenggat waktu yang wajar sebagai prasyarat mediasi.
```

---

## 14. PROCEDURAL & FORMAL LAW CHECKER

### 14.1 Konsep
PAUGERAN wajib melakukan pemeriksaan menyeluruh terhadap aspek formil dan prosedural sebelum memberikan rekomendasi litigasi.

### 14.2 Metodologi (Mandatory Checks)
1. **Daluwarsa (Statute of Limitations):** Hitung selisih waktu kejadian dengan hari ini vs batas waktu gugat (misal: 5 tahun untuk perdata umum).
2. **Kewenangan Pengadilan:** Absolut (Pengadilan Negeri vs Agama vs TUN) dan Relatif (domisili tergugat).
3. **Ne Bis In Idem:** Cek apakah perkara yang sama pernah diputus.
4. **Error in Persona:** Apakah pihak yang digugat adalah pihak yang tepat.
5. **Klausula Penyelesaian Sengketa:** Cek keberadaan klausula arbitrase atau mediasi wajib.

### 14.3 Output Format
```
PEMERIKSAAN PROSEDURAL & FORMIL (MANDATORY)
════════════════════════════════════════════════════
✓ Daluwarsa: Aman. Kejadian 2023, batas gugat 5 tahun.
✓ Kewenangan Absolut: Pengadilan Negeri (Sengketa Perdata).
⚠️ Kewenangan Relatif: PERINGATAN. Tergugat berdomisili di 
  Surabaya, namun Anda berencana menggugat di Jakarta. 
  → Risiko: Gugatan dapat dinyatakan NO (Niet Ontvankelijke 
  Verklaard) kecuali ada pilihan domisili dalam kontrak.
✓ Ne Bis In Idem: Tidak ditemukan putusan sebelumnya.
⚠️ Klausula Sengketa: Kontrak Pasal 12 menyebutkan "segala 
  perselisihan diselesaikan melalui Arbitrase BANI".
  → RISIKO FATAL: Pengadilan Negeri tidak berwenang mengadili. 
  Gugatan akan ditolak.
```

---

## 15. ECONOMIC & PRACTICAL REALITY CHECK

### 15.1 Konsep
PAUGERAN wajib mempertimbangkan implikasi praktis dan ekonomi dari kemenangan hukum, termasuk kemampuan eksekusi lawan (*judgment proof*), biaya vs manfaat, dan dampak hubungan bisnis.

### 15.2 Output Format
```
CEK REALITAS EKONOMI & PRAKTIS
════════════════════════════════════════════════════
Analisis Biaya vs Manfaat:
- Estimasi Biaya Perkara: Rp 50.000.000
- Nilai Gugatan: Rp 20.000.000
→ REKOMENDASI: Tidak ekonomis untuk litigasi. Pertimbangkan 
  negosiasi atau small claim court.

Analisis Kemampuan Eksekusi (Judgment Proof):
⚠️ Tergugat adalah PT yang sedang dalam proses PKPU berdasarkan 
  berita terkini.
→ Risiko: Meskipun Anda menang, eksekusi sita jaminan akan 
  terhambat oleh moratorium PKPU.
→ Strategi Alternatif: Tawarkan restrukturisasi utang atau 
  penjaminan oleh direksi secara pribadi.
```

---

## 16-19. MODUL LANJUTAN (RINGKASAN SPESIFIKASI)

**16. Multi-Jurisdictional & Private International Law:** Menganalisis *choice of law* dan *choice of forum* untuk kontrak internasional. Mendeteksi apakah hukum Indonesia atau hukum asing yang berlaku, serta apakah putusan asing dapat dieksekusi di Indonesia (berdasarkan asas timbal balik).

**17. Tax Implication Module:** Secara otomatis menyoroti potensi konsekuensi pajak (PPh, PPN, BPHTB) dari setiap transaksi atau penyelesaian sengketa yang dianalisis.

**18. Corporate Governance & Regulatory Compliance:** Untuk kasus korporasi, memeriksa kepatuhan terhadap RUPS, kewenangan direksi/komisaris (Pasal 97-98 UU PT), dan regulasi sektoral (OJK, KPPU).

**19. Strategic Timing & Forum Analysis:** Memberikan rekomendasi kapan waktu terbaik untuk bertindak (misal: sebelum lawan membalik aset) dan di forum mana (Pengadilan vs Arbitrase vs Mediasi) peluang sukses paling tinggi.

---

# BAGIAN III — SPESIFIKASI FUNGSIONAL LENGKAP

---

## 20-23. CASE STATE, REASONING MODES, & GRAPH

*(Mengacu pada definisi di Bagian I & II, dengan penekanan pada implementasi)*
- **Case State Machine:** Transisi state harus dicatat di `case_state_history`.
- **Reasoning Modes:** 6 mode (Exploration, Preventive, Dispute, Litigation Prep, Adversarial, Neutral). Setiap mode mengaktifkan subset spesifik dari 7 Lapisan Penalaran dan Modul Lanjutan (16-19).
- **Case Graph:** Struktur data wajib menggunakan `petgraph` di Rust. Setiap node harus memiliki `metadata` JSON yang menyimpan tingkat kepastian dan sumber.

---

## 24. LEGAL KNOWLEDGE BASE (SUPREME EDITION)

### 24.1 Spesifikasi Data
Setiap entri di Knowledge Base wajib memiliki:
- `full_text`: Teks lengkap pasal/putusan (TIDAK BOLEH RANGKUMAN).
- `hierarchy_level`: Integer (1=UUD, 2=UU, 3=PP, dst).
- `precedential_weight`: Float (0.0 - 1.0) untuk putusan.
- `effective_date` & `revoked_date`: Untuk validasi temporal.
- `tags`: Array string untuk pencarian semantik (misal: ["wanprestasi", "force majeure"]).

### 24.2 Fitur Wajib
- **Auto-Update Checker:** Membandingkan tanggal `revoked_date` entri lokal dengan sumber web resmi. Jika dicabut, tampilkan peringatan merah di UI.

---

## 25. PENELITIAN WEB

### 25.1 Whitelist Domain (Strict)
Hanya domain berikut yang diizinkan untuk scraping:
- `*.go.id` (Khususnya `jdih.setkab.go.id`, `mahkamahagung.go.id`, `putusan.mahkamahagung.go.id`)
- `jdih.kemenkeu.go.id`, `jdih.kumham.go.id`
- Domain yang secara eksplisit ditambahkan oleh Admin melalui UI.

### 25.2 Protokol Scraping
- Respect `robots.txt`.
- Delay 1 detik per request.
- Ekstrak metadata: Judul, Nomor, Tahun, Tanggal Pengundangan.
- Simpan URL dan Tanggal Akses ke dalam `Source` node di Case Graph.

---

## 26. MULTI-PROVIDER LLM

### 26.1 Spesifikasi Router
Router harus memilih model berdasarkan `Reasoning Mode`:
- **Exploration/Preventive:** Model cepat & murah (e.g., Claude Haiku, GPT-4o-mini).
- **Litigation/Adversarial/Neutral:** Model reasoning tinggi (e.g., Claude 3.5 Sonnet/Opus, GPT-4o).
- **Fallback:** Jika provider utama gagal (HTTP 429/500), otomatis coba provider cadangan yang dikonfigurasi pengguna.

---

## 27-29. AUTENTIKASI, KUSTOMISASI, AKSESIBILITAS

- **Autentikasi:** Opsional (`AUTH_ENABLED=false` default). Jika `true`, gunakan JWT + Argon2. First user = Admin.
- **Kustomisasi:** Tema (Light, Dark, Sepia, High Contrast), Font Size, Bahasa (ID/EN). Disimpan di `ui_preferences`.
- **Aksesibilitas:** Wajib WCAG 2.1 AA. Keyboard navigation penuh, ARIA labels, reduced motion support.

---

# BAGIAN IV — SPESIFIKASI OUTPUT PROFESIONAL

---

## 30. MANDATORY CITATION FORMAT (TIDAK BOLEH RANGKUMAN)

Setiap kali PAUGERAN mengutip hukum, format WAJIB:
```
[Sumber: {Jenis Peraturan} Nomor {Nomor} Tahun {Tahun} tentang {Judul}, Pasal {Pasal}, diundangkan pada {Tanggal}. Status: {Aktif/Dicabut}. Teks: "{Teks Lengkap Pasal}"]
```
**Larangan Keras:** Mengutip "Pasal 1338 KUHPerdata menyatakan bahwa perjanjian sah mengikat seperti UU" tanpa menyertakan teks lengkap dan metadata di atas.

---

## 31. LEGAL HIERARCHY DISPLAY

Output wajib menyertakan visualisasi atau teks yang menunjukkan posisi peraturan:
```
Hirarki Norma:
[3] Undang-Undang (UU No. 13 Tahun 2003)
    └── [4] Peraturan Pelaksana (PP No. 35 Tahun 2021)
```

---

## 32-34. MODE-SPECIFIC PROFESSIONAL OUTPUTS & EXPORT

Setiap mode menghasilkan template dokumen yang berbeda (PDF/DOCX):
- **Exploration:** Legal Possibility Assessment (Fokus pada skenario).
- **Preventive:** Legal Risk & Mitigation Matrix (Fokus pada klausul dan checklist).
- **Dispute:** Position Mapping & Settlement Strategy (Fokus pada perbandingan posisi).
- **Litigation Prep:** Case Theory & Evidence Chart (Fokus pada unsur-unsur dan pembuktian).
- **Adversarial:** Pre-Mortem & Vulnerability Report (Fokus pada serangan lawan dan mitigasi).
- **Neutral:** Judicial Simulation Report (Fokus pada penimbangan bukti secara adil).

Semua export wajib menyertakan *footer*: *"Dibuat dengan bantuan PAUGERAN AI. Analisis ini bukan nasihat hukum final dan wajib diverifikasi oleh Advokat berlisensi. Tanggal generate: [Timestamp]."*

---

# BAGIAN V — SPESIFIKASI TEKNIS

---

## 35-37. ARSITEKTUR & STACK

- **Single Binary:** Rust (Axum + Tokio) meng-embed frontend SolidJS (via `rust-embed`) dan menjalankan SQLite.
- **Graph Engine:** `petgraph` crate untuk Case Graph in-memory processing.
- **Database:** SQLite dengan ekstensi `sqlite-vec` untuk pencarian semantik Knowledge Base.
- **Ukuran Binary Target:** < 50 MB. Startup time: < 3 detik.

---

## 38. ENHANCED CASE GRAPH DATABASE SCHEMA

```sql
-- Tabel Nodes
CREATE TABLE case_nodes (
    id TEXT PRIMARY KEY,
    case_id TEXT NOT NULL,
    node_type TEXT NOT NULL CHECK (node_type IN ('case', 'party', 'fact', 'evidence', 'issue', 'rule', 'source', 'argument', 'counterargument', 'risk', 'conclusion', 'document')),
    content TEXT NOT NULL, -- Teks lengkap, bukan rangkuman
    metadata JSON NOT NULL, -- Menyimpan: hierarchy_level, precedential_weight, certainty_score, full_citation_text
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabel Edges
CREATE TABLE case_edges (
    id TEXT PRIMARY KEY,
    case_id TEXT NOT NULL,
    source_node_id TEXT NOT NULL,
    target_node_id TEXT NOT NULL,
    edge_type TEXT NOT NULL, -- 'supports', 'challenges', 'governed_by', 'derived_from', 'uncertain_because'
    metadata JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes untuk performa traversal
CREATE INDEX idx_case_nodes_case_type ON case_nodes(case_id, node_type);
CREATE INDEX idx_case_edges_source_target ON case_edges(source_node_id, target_node_id);
```

---

## 39-41. API, FRONTEND, & BUILD

- **API:** RESTful dengan SSE (`text/event-stream`) untuk streaming output agen. Endpoint wajib memvalidasi `case_id` terhadap `user_id` (isolasi ketat).
- **Frontend:** SolidJS + Vite + Tailwind CSS. Cytoscape.js untuk visualisasi Case Graph.
- **Build:** `cargo build --release` menghasilkan binary tunggal. Frontend di-build terlebih dahulu via `build.rs`.

---

# BAGIAN VI — SPESIFIKASI PERILAKU AGEN (SUPREME)

---

## 42-47. PERILAKU PER MODE

- **Exploration:** Wajib mengajukan minimal 2 pertanyaan klarifikasi sebelum memberikan kemungkinan isu hukum.
- **Preventive:** Wajib menyertakan "Checklist Tindakan Preventif" di akhir output.
- **Dispute:** Wajib menampilkan tabel perbandingan "Posisi Kita vs Posisi Lawan".
- **Litigation Prep:** Wajib menjalankan **Procedural & Formal Law Checker** (Pasal 14) sebelum menyusun Case Theory.
- **Adversarial:** Wajib menjalankan **Devil's Advocate Analysis** (Pasal 7.8) dan menyoroti minimal 3 kelemahan fatal argumen pengguna.
- **Neutral:** Wajib menimbang bukti kedua belah pihak secara simetris sebelum memberikan simulasi putusan.

---

## 48-51. UNCERTAINTY, BAHASA, & KETERLACAKAN

- **Uncertainty Metric:** Setiap kesimpulan harus memiliki `certainty_score` (0.0 - 1.0) dan daftar "Faktor yang Dapat Mengubah Kesimpulan".
- **Bahasa:** Bahasa Indonesia baku, istilah hukum tepat (misal: "wanprestasi", bukan "ingkar janji").
- **Keterlacakan:** Setiap node `conclusion` di Case Graph wajib memiliki edge `supported_by` ke `argument`, yang memiliki edge `relies_on` ke `rule`, yang memiliki edge `sourced_from` ke `source` (dengan `full_citation_text`). Jika rantai ini putus, output harus ditandai ⚠️ **KETERLACAKAN TIDAK LENGKAP**.

---

# BAGIAN VII — SPESIFIKASI ANTARMUKA

---

## 52-56. UI/UX

- **Layout:** Sidebar (Daftar Perkara) + Main Chat Area + Right Drawer (Case Graph / Export).
- **Command Palette:** `Ctrl+K` untuk akses cepat ke semua fitur (Ganti Mode, Export, Cari Kasus).
- **Responsivitas:** Desktop (3 kolom), Tablet (Sidebar collapsible), Mobile (Sidebar sebagai drawer).
- **Indikator Mode:** Badge berwarna di header chat menunjukkan Reasoning Mode aktif (misal: Ungu untuk Adversarial, Hijau untuk Preventive).

---

# BAGIAN VIII — SPESIFIKASI DEPLOYMENT & OPERASIONAL

---

## 57-61. DEPLOYMENT & BACKUP

- **Single Binary Deployment:** User download `.exe`/bin, jalankan, browser terbuka di `localhost:3000`.
- **Data Directory:** `~/.local/share/paugeran/` (Linux), `%APPDATA%\paugeran\` (Windows).
- **Backup:** Fitur "Export Full Case Data" (JSON + Dokumen) dan "Backup Database" (SQLite file copy).
- **Monitoring:** Logging terstruktur (JSON) ke file lokal. Endpoint `/health` untuk cek status.

---

# BAGIAN IX — SPESIFIKASI KEAMANAN & COMPLIANCE

---

## 62-65. KEAMANAN

- **Enkripsi:** API Key dan Lisensi dienkripsi dengan AES-256-GCM. Kunci enkripsi disimpan di file `.secret` dengan permission `600`.
- **Isolasi:** Query SQL wajib menggunakan parameterized queries (`sqlx`) dan memfilter berdasarkan `case_id` dan `user_id`.
- **PII Redaction:** Sebelum dikirim ke LLM, nama, NIK, nomor rekening harus di-replace dengan placeholder (misal: `[PERSON_1]`), lalu di-deanonymize saat menampilkan hasil ke user.
- **Audit Trail:** Semua aksi kritis (hapus kasus, ganti mode, export) dicatat di `audit_logs` dengan timestamp dan IP.

---

# BAGIAN X — KRITERIA PENERIMAAN & TESTING

---

## 66-69. KRITERIA & TESTING

### 66. Kriteria Keberhasilan (Supreme)
- 100% output hukum menyertakan **Full Citation** (bukan rangkuman).
- 0% halusinasi pasal/putusan dalam 100 kasus uji (divalidasi oleh ahli hukum).
- Semua 7 Lapisan Penalaran terlihat dalam output "Neutral" atau "Adversarial" mode.
- Procedural Checker berhasil mendeteksi jebakan daluwarsa/arbitrase dalam 100% kasus uji yang dirancang untuk itu.

### 67. Kriteria Penerimaan (Strict)
- [ ] Binary berjalan tanpa dependency eksternal.
- [ ] Case Graph dapat divisualisasikan dan di-traverse.
- [ ] PII Redaction berfungsi sebelum payload dikirim ke LLM.
- [ ] Export PDF/DOCX menghasilkan dokumen dengan format profesional dan footer disclaimer.
- [ ] Isolasi data antar case terverifikasi melalui penetration test.

### 68-69. Testing Strategy & Quality Gates
- **Unit Test:** >80% coverage untuk logika Rust (Graph, Routing, Validation).
- **Integration Test:** Mock LLM responses untuk menguji alur 7 lapisan penalaran.
- **E2E Test:** Simulasi user journey dari pembuatan kasus hingga export laporan.
- **Quality Gate:** Tidak ada `unwrap()` di production code, `cargo clippy` clean, `cargo audit` clean.

---

# BAGIAN XI — TATA KELOLA & DOKUMEN TURUNAN

---

## 70. LARANGAN PRODUK

PAUGERAN **DILARANG KERAS** untuk:
1. Mengarang pasal, nomor peraturan, atau tanggal putusan.
2. Memberikan rangkuman pasal tanpa menyertakan teks lengkap dan metadata sumber.
3. Mengabaikan peringatan daluwarsa atau klausula arbitrase.
4. Menyimpan API key atau data pengguna dalam plain text.
5. Mengizinkan akses data satu kasus ke kasus lain (pelanggaran isolasi).
6. Merekomendasikan taktik yang melanggar Kode Etik Advokat.
7. Menyatakan kepastian 100% tanpa menyertakan structured uncertainty metric.

---

## 71. DOKUMEN TURUNAN

Dokumen ini adalah **Single Source of Truth**. Dokumen berikut **WAJIB** dibuat dan harus merujuk ke bagian spesifik dari dokumen ini menggunakan format `[CB §X.Y]`:

1. **SPEC-ARCH:** Arsitektur Teknis & Diagram Alur Data.
2. **SPEC-API:** OpenAPI 3.1 Specification (YAML).
3. **SPEC-DB:** Skema Database & Migration Scripts.
4. **SPEC-GRAPH:** Detail Implementasi `petgraph` & Traversal Algorithms.
5. **TEST-PLAN:** Rencana Pengujian Unit, Integrasi, E2E, dan Keamanan.
6. **USER-GUIDE:** Panduan Pengguna Akhir (Non-teknis).
7. **LEGAL-DISCLAIMER:** Syarat & Ketentuan serta Kebijakan Privasi (UU PDP compliant).

---

## 72. PERUBAHAN & VERSI

- Setiap perubahan pada dokumen ini memerlukan **Change Request** tertulis.
- Harus disetujui oleh Product Owner dan Lead Legal Advisor.
- Versi dokumen turunan harus selalu sinkron dengan versi Master Specification ini.

---

## 73. PENUTUP

Dokumen **PAUGERAN — MASTER SPECIFICATION v1.0 FINAL** ini adalah kontrak mutlak yang mengatur seluruh aspek pengembangan, perilaku, dan output sistem. 

PAUGERAN bukan sekadar alat otomatisasi. Ia adalah **Supreme Legal Reasoning Engine** yang dirancang untuk menegakkan standar tertinggi dalam penalaran hukum: ketelitian, keterlacakan penuh, kejujuran intelektual, dan kepatuhan pada etika profesi. 

Setiap baris kode, setiap desain antarmuka, dan setiap algoritma yang dibangun harus tunduk pada prinsip-prinsip yang tercantum di sini. Tidak ada kompromi untuk kualitas, keamanan, dan akurasi hukum.

---

**Dokumen:** PAUGERAN — MASTER SPECIFICATION v1.0 FINAL  
**Status:** FINAL — SUMBER KEBENARAN MUTLAK  
**Disusun Oleh:** Tim Produk & Legal Advisory PAUGERAN  
**Disetujui Oleh:** [Stakeholder / Lead Counsel]  
**Tanggal Efektif:** 28 Agustus 2026  

---
*"Memahami masalah. Menelusuri hukum. Menguji alasan. Dengan standar tertinggi."*
