# PAUGERAN — SPESIFIKASI ARSITEKTUR REPOSITORI v1.0
## Complete End-to-End Repository Architecture

**Dokumen:** Repository Architecture Specification  
**Referensi Utama:** MASTER SPECIFICATION v1.0 FINAL [CB]  
**Status:** FINAL — SIAP IMPLEMENTASI  
**Sifat:** Kontrak teknis yang mengatur seluruh struktur, konvensi, dan organisasi kode proyek PAUGERAN  
**Kedudukan:** Dokumen turunan tingkat 1 dari Master Specification [CB §71.1]

---

## 📋 DAFTAR ISI

1. Prinsip Arsitektur Repositori
2. Struktur Direktori Lengkap
3. Konfigurasi Root Project
4. Backend Rust — Struktur & Konvensi
5. Frontend SolidJS — Struktur & Konvensi
6. Shared Packages — Struktur & Konvensi
7. Infrastructure — Struktur & Konvensi
8. Documentation — Struktur & Konvensi
9. Build System & Pipeline
10. Testing Architecture
11. CI/CD Pipeline
12. Konvensi Penamaan & Coding Standards
13. Dependency Management
14. Data & Storage Architecture
15. Cross-Platform Build Matrix
16. Release & Distribution
17. Penutup

---

## 1. PRINSIP ARSITEKTUR REPOSITORI

### 1.1 Prinsip Dasar

**RA-01 — Single Source of Truth**
Repositori ini adalah satu-satunya sumber kebenaran untuk seluruh kode, konfigurasi, dan dokumentasi proyek PAUGERAN. Tidak ada kode produksi yang hidup di luar repositori ini.

**RA-02 — Monorepo dengan Workspace Isolation**
Proyek menggunakan monorepo dengan workspace yang terisolasi secara jelas: backend Rust, frontend SolidJS, shared packages, dan infrastructure. Setiap workspace memiliki tanggung jawab yang tidak tumpang tindih.

**RA-03 — Single Binary Output**
Seluruh kode harus dapat di-compile menjadi satu binary executable. Frontend di-embed ke dalam binary Rust melalui `rust-embed`. Tidak ada artefak deployment terpisah.

**RA-04 — Convention Over Configuration**
Struktur direktori, penamaan file, dan organisasi kode mengikuti konvensi yang telah ditetapkan. Setiap developer harus dapat menemukan file yang dibutuhkan tanpa bertanya.

**RA-05 — Test Co-location**
Test ditempatkan sedekat mungkin dengan kode yang diuji. Unit test di dalam module yang sama, integration test di direktori terpisah per workspace.

**RA-06 — Documentation as Code**
Dokumentasi hidup di dalam repositori, ditulis dalam Markdown, dan di-version bersama kode. Tidak ada dokumentasi yang hidup di luar repositori (kecuali yang di-generate otomatis).

**RA-07 — Reproducible Builds**
Setiap build harus dapat direproduksi secara identik di mesin manapun. Semua dependency di-lock, semua konfigurasi di-version.

**RA-08 — Security by Default**
Tidak ada secrets, credentials, atau data sensitif yang boleh masuk ke repositori. Semua secrets dikelola melalui environment variables atau secret manager.

---

## 2. STRUKTUR DIREKTORI LENGKAP

```
paugeran/
│
├── 📄 README.md                              # Project overview & quick start
├── 📄 CONTRIBUTING.md                        # Contribution guidelines
├── 📄 CHANGELOG.md                           # Version history
├── 📄 LICENSE                                # MIT License
├── 📄 CODE_OF_CONDUCT.md                     # Community guidelines
├── 📄 SECURITY.md                            # Security policy & reporting
│
├── 📄 Cargo.toml                             # Rust workspace root [§3.1]
├── 📄 Cargo.lock                             # Rust dependency lock (COMMITTED)
├── 📄 package.json                           # Node.js workspace root [§3.2]
├── 📄 pnpm-lock.yaml                         # pnpm dependency lock (COMMITTED)
├── 📄 pnpm-workspace.yaml                    # pnpm workspace config [§3.3]
├── 📄 turbo.json                             # Turborepo config (optional) [§3.4]
│
├── 📄 .gitignore                             # Git ignore rules [§3.5]
├── 📄 .gitattributes                         # Git attributes (LFS, line endings)
├── 📄 .editorconfig                          # Editor consistency [§3.6]
├── 📄 .nvmrc                                 # Node.js version pin (20.x)
├── 📄 .rustfmt.toml                          # Rust formatting config [§12.1]
├── 📄 clippy.toml                            # Rust clippy config [§12.2]
├── 📄 .env.example                           # Environment variables template
├── 📄 .env.test                              # Test environment variables
│
├── 📁 .github/                               # GitHub configurations [§11]
│   ├── 📁 workflows/
│   │   ├── 📄 ci.yml                         # Continuous Integration
│   │   ├── 📄 release.yml                    # Release automation
│   │   ├── 📄 security-audit.yml             # Security scanning
│   │   ├── 📄 docs.yml                       # Documentation deployment
│   │   └── 📄 nightly-build.yml              # Nightly builds
│   ├── 📁 ISSUE_TEMPLATE/
│   │   ├── 📄 bug-report.md
│   │   ├── 📄 feature-request.md
│   │   └── 📄 security-vulnerability.md
│   ├── 📄 PULL_REQUEST_TEMPLATE.md
│   ├── 📄 CODEOWNERS
│   └── 📄 dependabot.yml
│
├── 📁 .vscode/                               # VS Code workspace settings
│   ├── 📄 settings.json
│   ├── 📄 extensions.json
│   ├── 📄 launch.json                        # Debug configurations
│   └── 📄 tasks.json                         # Build tasks
│
│
│ ══════════════════════════════════════════════
│  BACKEND — RUST (apps/server)
│ ══════════════════════════════════════════════
│
├── 📁 apps/
│   ├── 📁 server/                            # Rust Backend [§4]
│   │   ├── 📄 Cargo.toml                     # Server dependencies [§4.1]
│   │   ├── 📄 build.rs                       # Build script (embed frontend) [§4.2]
│   │   ├── 📄 README.md                      # Backend documentation
│   │   │
│   │   ├── 📁 src/
│   │   │   ├── 📄 main.rs                    # Entry point [§4.3]
│   │   │   ├── 📄 lib.rs                     # Library exports [§4.4]
│   │   │   ├── 📄 error.rs                   # Global error types [§4.5]
│   │   │   │
│   │   │   ├── 📁 config/                    # Configuration [§4.6]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 app_config.rs          # App configuration struct
│   │   │   │   ├── 📄 data_dir.rs            # Platform-specific data dir
│   │   │   │   ├── 📄 env_loader.rs          # Environment variable loader
│   │   │   │   └── 📄 defaults.rs            # Default values
│   │   │   │
│   │   │   ├── 📁 http/                      # HTTP Server Layer [§4.7]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 app.rs                 # Axum app builder
│   │   │   │   ├── 📄 router.rs              # Route definitions
│   │   │   │   ├── 📄 state.rs               # Application state
│   │   │   │   ├── 📄 sse.rs                 # Server-Sent Events
│   │   │   │   ├── 📁 middleware/
│   │   │   │   │   ├── 📄 mod.rs
│   │   │   │   │   ├── 📄 auth.rs            # JWT auth (optional)
│   │   │   │   │   ├── 📄 cors.rs            # CORS configuration
│   │   │   │   │   ├── 📄 logging.rs         # Request logging
│   │   │   │   │   ├── 📄 rate_limit.rs      # Rate limiting
│   │   │   │   │   └── 📄 error_handler.rs   # Global error handler
│   │   │   │   └── 📁 handlers/
│   │   │   │       ├── 📄 mod.rs
│   │   │   │       ├── 📄 health.rs          # GET /health [CB §57]
│   │   │   │       ├── 📄 setup.rs           # Setup wizard [CB §3.2]
│   │   │   │       ├── 📄 cases.rs           # Case CRUD [CB §3.3]
│   │   │   │       ├── 📄 messages.rs        # Chat + SSE [CB §3.4]
│   │   │   │       ├── 📄 documents.rs       # Document upload [CB §3.7]
│   │   │   │       ├── 📄 knowledge.rs       # Knowledge Base [CB §24]
│   │   │   │       ├── 📄 providers.rs       # LLM providers [CB §26]
│   │   │   │       ├── 📄 preferences.rs     # UI preferences [CB §28]
│   │   │   │       ├── 📄 export.rs          # PDF/DOCX export [CB §34]
│   │   │   │       ├── 📄 case_graph.rs      # Graph visualization [CB §22]
│   │   │   │       ├── 📄 reasoning.rs       # Reasoning trigger [CB §21]
│   │   │   │       └── 📁 admin/
│   │   │   │           ├── 📄 mod.rs
│   │   │   │           ├── 📄 users.rs       # User management [CB §27]
│   │   │   │           ├── 📄 invitations.rs # Invitation system
│   │   │   │           ├── 📄 global_providers.rs
│   │   │   │           ├── 📄 global_knowledge.rs
│   │   │   │           └── 📄 system_config.rs
│   │   │   │
│   │   │   ├── 📁 engine/                    # Supreme Adaptive Graph Engine [§4.8]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 adaptive_engine.rs     # Main engine coordinator
│   │   │   │   ├── 📄 mode_router.rs         # Reasoning mode selection [CB §21]
│   │   │   │   ├── 📄 state_machine.rs       # Case state transitions [CB §20]
│   │   │   │   ├── 📄 event_streamer.rs      # SSE event streaming
│   │   │   │   ├── 📄 cancellation.rs        # CancellationToken
│   │   │   │   ├── 📄 context.rs             # ExecutionContext
│   │   │   │   │
│   │   │   │   ├── 📁 modes/                 # 6 Reasoning Modes [CB §42-47]
│   │   │   │   │   ├── 📄 mod.rs
│   │   │   │   │   ├── 📄 exploration.rs     # Exploration Mode [CB §42]
│   │   │   │   │   ├── 📄 preventive.rs      # Preventive Mode [CB §43]
│   │   │   │   │   ├── 📄 dispute.rs         # Dispute Mode [CB §44]
│   │   │   │   │   ├── 📄 litigation_prep.rs # Litigation Prep [CB §45]
│   │   │   │   │   ├── 📄 adversarial.rs     # Adversarial Mode [CB §46]
│   │   │   │   │   └── 📄 neutral.rs         # Neutral Mode [CB §47]
│   │   │   │   │
│   │   │   │   ├── 📁 layers/                # 7 Legal Reasoning Layers [CB §7]
│   │   │   │   │   ├── 📄 mod.rs
│   │   │   │   │   ├── 📄 grammatical.rs     # Layer 1: Grammatical [CB §7.2]
│   │   │   │   │   ├── 📄 systematic.rs      # Layer 2: Systematic [CB §7.3]
│   │   │   │   │   ├── 📄 teleological.rs    # Layer 3: Teleological [CB §7.4]
│   │   │   │   │   ├── 📄 sociological.rs    # Layer 4: Sociological [CB §7.5]
│   │   │   │   │   ├── 📄 historical.rs      # Layer 5: Historical [CB §7.6]
│   │   │   │   │   ├── 📄 comparative.rs     # Layer 6: Comparative [CB §7.7]
│   │   │   │   │   ├── 📄 critical.rs        # Layer 7: Critical [CB §7.8]
│   │   │   │   │   └── 📄 synthesis.rs       # 7-Layer Synthesis [CB §7.9]
│   │   │   │   │
│   │   │   │   ├── 📁 nodes/                 # Case Graph Node Executors [CB §22]
│   │   │   │   │   ├── 📄 mod.rs
│   │   │   │   │   ├── 📄 node_trait.rs      # Node trait definition
│   │   │   │   │   ├── 📄 fact_extractor.rs  # Extract facts from input
│   │   │   │   │   ├── 📄 issue_identifier.rs # Identify legal issues
│   │   │   │   │   ├── 📄 rule_retriever.rs  # Retrieve legal rules
│   │   │   │   │   ├── 📄 argument_builder.rs # Build arguments
│   │   │   │   │   ├── 📄 counterargument.rs # Generate counterarguments
│   │   │   │   │   ├── 📄 risk_assessor.rs   # Assess risks
│   │   │   │   │   ├── 📄 conclusion_maker.rs # Draw conclusions
│   │   │   │   │   ├── 📄 citation_validator.rs # Validate citations [CB §30]
│   │   │   │   │   └── 📄 requalification.rs # Fact re-qualification [CB P-45]
│   │   │   │   │
│   │   │   │   └── 📁 modules/               # Advanced Legal Modules [CB §8-19]
│   │   │   │       ├── 📄 mod.rs
│   │   │   │       ├── 📄 norm_conflict.rs   # Norm Conflict Resolution [CB §8]
│   │   │   │       ├── 📄 temporal_law.rs    # Temporal Law Application [CB §9]
│   │   │   │       ├── 📄 judicial_discretion.rs # Judicial Discretion [CB §10]
│   │   │   │       ├── 📄 causation.rs       # Causation & Remoteness [CB §11]
│   │   │   │       ├── 📄 customary_law.rs   # Unwritten & Customary Law [CB §12]
│   │   │   │       ├── 📄 ethical_guardrails.rs # Ethical Guardrails [CB §13]
│   │   │   │       ├── 📄 procedural_check.rs # Procedural & Formal Check [CB §14]
│   │   │   │       ├── 📄 economic_check.rs  # Economic & Practical Check [CB §15]
│   │   │   │       ├── 📄 multi_jurisdiction.rs # Multi-Jurisdictional [CB §16]
│   │   │   │       ├── 📄 tax_implication.rs # Tax Implications [CB §17]
│   │   │   │       ├── 📄 corporate_governance.rs # Corporate Governance [CB §18]
│   │   │   │       ├── 📄 strategic_timing.rs # Strategic Timing & Forum [CB §19]
│   │   │   │       ├── 📄 islamic_law.rs     # Islamic Law Module [CB P-58]
│   │   │   │       ├── 📄 public_policy.rs   # Public Policy Filter [CB P-59]
│   │   │   │       └── 📄 enforcement.rs     # Enforcement Strategy [CB P-60]
│   │   │   │
│   │   │   ├── 📁 case_graph/                # Case Graph Management [CB §22]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 graph_manager.rs       # In-memory graph operations
│   │   │   │   ├── 📄 node_types.rs          # Node type definitions
│   │   │   │   ├── 📄 edge_types.rs          # Edge type definitions
│   │   │   │   ├── 📄 traversal.rs           # Graph traversal algorithms
│   │   │   │   ├── 📄 persistence.rs         # Save/load from SQLite
│   │   │   │   ├── 📄 visualization.rs       # Prepare data for frontend
│   │   │   │   └── 📄 validation.rs          # Graph integrity validation
│   │   │   │
│   │   │   ├── 📁 llm/                       # Multi-Provider LLM [CB §26]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 provider_trait.rs      # LlmProvider trait
│   │   │   │   ├── 📄 router.rs              # Mode-aware model routing
│   │   │   │   ├── 📄 fallback.rs            # Fallback mechanism
│   │   │   │   ├── 📄 token_counter.rs       # Token counting & budget
│   │   │   │   └── 📁 providers/
│   │   │   │       ├── 📄 mod.rs
│   │   │   │       ├── 📄 anthropic.rs       # Anthropic Claude
│   │   │   │       ├── 📄 openai.rs          # OpenAI GPT
│   │   │   │       ├── 📄 openai_compat.rs   # Generic OpenAI-compatible
│   │   │   │       └── 📄 ollama.rs          # Local Ollama
│   │   │   │
│   │   │   ├── 📁 knowledge_base/            # Supreme Legal KB [CB §24]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 kb_manager.rs          # KB CRUD operations
│   │   │   │   ├── 📄 document_parser.rs     # Parse PDF/DOCX regulations
│   │   │   │   ├── 📄 article_splitter.rs    # Split into articles
│   │   │   │   ├── 📄 embedding_generator.rs # Vector embeddings
│   │   │   │   ├── 📄 semantic_search.rs     # Semantic search (sqlite-vec)
│   │   │   │   ├── 📄 keyword_search.rs      # Keyword search
│   │   │   │   ├── 📄 hybrid_search.rs       # Hybrid search
│   │   │   │   ├── 📄 metadata_extractor.rs  # Extract regulation metadata
│   │   │   │   ├── 📄 norm_conflict_db.rs    # Norm conflict mapping
│   │   │   │   ├── 📄 precedent_weight.rs    # Precedential weight system
│   │   │   │   ├── 📄 temporal_tracker.rs    # Temporal law tracking
│   │   │   │   └── 📄 update_checker.rs      # Auto-update checker
│   │   │   │
│   │   │   ├── 📁 web_research/              # Web Research [CB §25]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 researcher.rs          # Main research coordinator
│   │   │   │   ├── 📄 whitelist.rs           # Domain whitelist management
│   │   │   │   ├── 📄 http_client.rs         # Reqwest-based HTTP client
│   │   │   │   ├── 📄 html_parser.rs         # HTML parsing (scraper)
│   │   │   │   ├── 📄 content_extractor.rs   # Extract relevant content
│   │   │   │   ├── 📄 rate_limiter.rs        # Rate limiting
│   │   │   │   └── 📄 robots_checker.rs      # robots.txt compliance
│   │   │   │
│   │   │   ├── 📁 database/                  # Database Layer [CB §38]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 pool.rs                # SQLite connection pool
│   │   │   │   ├── 📄 migrations.rs          # Migration runner
│   │   │   │   ├── 📁 repositories/
│   │   │   │   │   ├── 📄 mod.rs
│   │   │   │   │   ├── 📄 case_repo.rs       # Case CRUD
│   │   │   │   │   ├── 📄 message_repo.rs    # Message CRUD
│   │   │   │   │   ├── 📄 document_repo.rs   # Document CRUD
│   │   │   │   │   ├── 📄 knowledge_repo.rs  # Knowledge Base CRUD
│   │   │   │   │   ├── 📄 provider_repo.rs   # LLM provider CRUD
│   │   │   │   │   ├── 📄 preference_repo.rs # UI preferences CRUD
│   │   │   │   │   ├── 📄 user_repo.rs       # User CRUD
│   │   │   │   │   ├── 📄 audit_repo.rs      # Audit log CRUD
│   │   │   │   │   └── 📄 graph_repo.rs      # Case Graph persistence
│   │   │   │   └── 📁 models/
│   │   │   │       ├── 📄 mod.rs
│   │   │   │       ├── 📄 case.rs            # Case model
│   │   │   │       ├── 📄 message.rs         # Message model
│   │   │   │       ├── 📄 document.rs        # Document model
│   │   │   │       ├── 📄 knowledge.rs       # Knowledge Base models
│   │   │   │       ├── 📄 provider.rs        # LLM provider model
│   │   │   │       ├── 📄 preference.rs      # UI preference model
│   │   │   │       ├── 📄 user.rs            # User model
│   │   │   │       ├── 📄 audit.rs           # Audit log model
│   │   │   │       └── 📄 graph.rs           # Graph node/edge models
│   │   │   │
│   │   │   ├── 📁 export/                    # Document Export [CB §34]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 pdf_generator.rs       # PDF generation (printpdf)
│   │   │   │   ├── 📄 docx_generator.rs      # DOCX generation (docx-rs)
│   │   │   │   ├── 📄 formatter.rs           # Content formatting
│   │   │   │   ├── 📄 citation_renderer.rs   # Full citation rendering
│   │   │   │   ├── 📄 hierarchy_renderer.rs  # Legal hierarchy display
│   │   │   │   ├── 📄 graph_renderer.rs      # Case graph to image
│   │   │   │   └── 📁 templates/
│   │   │   │       ├── 📄 mod.rs
│   │   │   │       ├── 📄 exploration.rs     # Exploration template
│   │   │   │       ├── 📄 preventive.rs      # Preventive template
│   │   │   │       ├── 📄 dispute.rs         # Dispute template
│   │   │   │       ├── 📄 litigation.rs      # Litigation template
│   │   │   │       ├── 📄 adversarial.rs     # Adversarial template
│   │   │   │       └── 📄 neutral.rs         # Neutral template
│   │   │   │
│   │   │   ├── 📁 crypto/                    # Encryption & Security [CB §62-63]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   ├── 📄 aes_gcm.rs             # AES-256-GCM encryption
│   │   │   │   ├── 📄 key_manager.rs         # Encryption key management
│   │   │   │   ├── 📄 password_hash.rs       # Argon2 password hashing
│   │   │   │   ├── 📄 jwt.rs                 # JWT token management
│   │   │   │   └── 📄 pii_redactor.rs        # PII redaction [CB P-15]
│   │   │   │
│   │   │   ├── 📁 frontend/                  # Embedded Frontend [CB §35]
│   │   │   │   ├── 📄 mod.rs
│   │   │   │   └── 📄 assets.rs              # rust-embed configuration
│   │   │   │
│   │   │   └── 📁 utils/                     # Utility Functions
│   │   │       ├── 📄 mod.rs
│   │   │       ├── 📄 text_processing.rs     # Text processing
│   │   │       ├── 📄 date_utils.rs          # Date/time utilities
│   │   │       ├── 📄 validation.rs          # Input validation
│   │   │       ├── 📄 id_generator.rs        # UUID generation
│   │   │       └── 📄 logger.rs              # Structured logging
│   │   │
│   │   ├── 📁 migrations/                    # SQLx Migrations [CB §38]
│   │   │   ├── 📄 0001_initial_schema.sql
│   │   │   ├── 📄 0002_case_graph.sql
│   │   │   ├── 📄 0003_knowledge_base.sql
│   │   │   ├── 📄 0004_users_auth.sql
│   │   │   ├── 📄 0005_audit_logs.sql
│   │   │   └── 📄 0006_embeddings.sql
│   │   │
│   │   ├── 📁 tests/                         # Backend Tests [§10]
│   │   │   ├── 📁 unit/
│   │   │   │   ├── 📄 test_case_graph.rs
│   │   │   │   ├── 📄 test_mode_router.rs
│   │   │   │   ├── 📄 test_state_machine.rs
│   │   │   │   ├── 📄 test_norm_conflict.rs
│   │   │   │   ├── 📄 test_temporal_law.rs
│   │   │   │   ├── 📄 test_causation.rs
│   │   │   │   ├── 📄 test_citation_validator.rs
│   │   │   │   ├── 📄 test_pii_redactor.rs
│   │   │   │   ├── 📄 test_aes_gcm.rs
│   │   │   │   └── 📄 test_llm_router.rs
│   │   │   ├── 📁 integration/
│   │   │   │   ├── 📄 test_api_cases.rs
│   │   │   │   ├── 📄 test_api_messages.rs
│   │   │   │   ├── 📄 test_api_export.rs
│   │   │   │   ├── 📄 test_agent_flow.rs
│   │   │   │   ├── 📄 test_knowledge_base.rs
│   │   │   │   └── 📄 test_isolation.rs
│   │   │   └── 📁 fixtures/
│   │   │       ├── 📄 sample_case.json
│   │   │       ├── 📄 sample_regulation.pdf
│   │   │       ├── 📄 sample_contract.docx
│   │   │       └── 📄 sample_knowledge.json
│   │   │
│   │   └── 📁 benches/                       # Benchmarks
│   │       ├── 📄 bench_graph_traversal.rs
│   │       ├── 📄 bench_semantic_search.rs
│   │       └── 📄 bench_pdf_generation.rs
│   │
│   │
│   │ ══════════════════════════════════════════
│   │  FRONTEND — SOLIDJS (apps/web)
│   │ ══════════════════════════════════════════
│   │
│   └── 📁 web/                               # SolidJS Frontend [§5]
│       ├── 📄 package.json                   # Frontend dependencies [§5.1]
│       ├── 📄 tsconfig.json                  # TypeScript config [§5.2]
│       ├── 📄 tsconfig.node.json             # Node TypeScript config
│       ├── 📄 vite.config.ts                 # Vite config [§5.3]
│       ├── 📄 tailwind.config.js             # Tailwind CSS config [§5.4]
│       ├── 📄 postcss.config.js              # PostCSS config
│       ├── 📄 index.html                     # HTML entry point
│       ├── 📄 README.md                      # Frontend documentation
│       │
│       ├── 📁 public/                        # Static assets
│       │   ├── 📄 favicon.ico
│       │   ├── 📄 favicon.svg
│       │   ├── 📄 manifest.json
│       │   └── 📁 icons/
│       │       ├── 📄 icon-16.png
│       │       ├── 📄 icon-32.png
│       │       ├── 📄 icon-128.png
│       │       └── 📄 icon-256.png
│       │
│       └── 📁 src/
│           ├── 📄 main.tsx                   # Entry point
│           ├── 📄 App.tsx                    # Root component
│           │
│           ├── 📁 components/                # UI Components [CB §53]
│           │   ├── 📁 chat/
│           │   │   ├── 📄 ChatWindow.tsx     # Main chat interface
│           │   │   ├── 📄 MessageBubble.tsx  # Individual message
│           │   │   ├── 📄 CaseSidebar.tsx    # Case list sidebar
│           │   │   ├── 📄 InputArea.tsx      # Message input
│           │   │   ├── 📄 ModeIndicator.tsx  # Current mode badge
│           │   │   ├── 📄 StateBadge.tsx     # Case state badge
│           │   │   ├── 📄 StreamingText.tsx  # Streaming text renderer
│           │   │   ├── 📄 CitationTooltip.tsx # Inline citation tooltip
│           │   │   └── 📄 PhaseIndicator.tsx # Agent phase indicator
│           │   │
│           │   ├── 📁 case_graph/
│           │   │   ├── 📄 CaseGraphViewer.tsx # Cytoscape.js viewer
│           │   │   ├── 📄 NodeDetail.tsx     # Node detail panel
│           │   │   ├── 📄 GraphControls.tsx  # Zoom, pan, filter
│           │   │   ├── 📄 GraphLegend.tsx    # Legend for node types
│           │   │   └── 📄 GraphExport.tsx    # Export graph as PNG/SVG
│           │   │
│           │   ├── 📁 reasoning/
│           │   │   ├── 📄 LayerDisplay.tsx   # 7-layer interpretation
│           │   │   ├── 📄 NormConflictCard.tsx # Norm conflict display
│           │   │   ├── 📄 TemporalLawCard.tsx # Temporal law display
│           │   │   ├── 📄 UncertaintyMeter.tsx # Uncertainty metric
│           │   │   ├── 📄 HierarchyTree.tsx  # Legal hierarchy tree
│           │   │   └── 📄 PrecedentWeight.tsx # Precedent weight stars
│           │   │
│           │   ├── 📁 settings/
│           │   │   ├── 📄 SettingsPanel.tsx  # Settings container
│           │   │   ├── 📄 ThemeSelector.tsx  # Theme selection
│           │   │   ├── 📄 FontSelector.tsx   # Font settings
│           │   │   ├── 📄 LanguageSelector.tsx # Language (ID/EN)
│           │   │   ├── 📄 AccessibilitySettings.tsx
│           │   │   ├── 📄 LLMProviderManager.tsx
│           │   │   ├── 📄 KnowledgeBaseManager.tsx
│           │   │   ├── 📄 WebResearchSettings.tsx
│           │   │   └── 📄 DataManager.tsx    # Backup/restore
│           │   │
│           │   ├── 📁 knowledge/
│           │   │   ├── 📄 KnowledgeBaseList.tsx
│           │   │   ├── 📄 KnowledgeBaseDetail.tsx
│           │   │   ├── 📄 KnowledgeBaseImport.tsx
│           │   │   └── 📄 KnowledgeBaseSearch.tsx
│           │   │
│           │   ├── 📁 export/
│           │   │   ├── 📄 ExportDialog.tsx   # Export options
│           │   │   ├── 📄 ExportPreview.tsx  # Preview before export
│           │   │   └── 📄 TemplateSelector.tsx
│           │   │
│           │   ├── 📁 admin/                 # Admin panel [CB §27]
│           │   │   ├── 📄 AdminPanel.tsx
│           │   │   ├── 📄 UserManagement.tsx
│           │   │   ├── 📄 InvitationManager.tsx
│           │   │   ├── 📄 GlobalProviderManager.tsx
│           │   │   ├── 📄 GlobalKnowledgeBase.tsx
│           │   │   └── 📄 SystemConfig.tsx
│           │   │
│           │   ├── 📁 common/
│           │   │   ├── 📄 CommandPalette.tsx # Ctrl+K [CB §56]
│           │   │   ├── 📄 Toast.tsx          # Toast notifications
│           │   │   ├── 📄 Modal.tsx          # Modal dialog
│           │   │   ├── 📄 Drawer.tsx         # Side drawer
│           │   │   ├── 📄 Breadcrumb.tsx     # Breadcrumb nav
│           │   │   ├── 📄 LoadingSpinner.tsx
│           │   │   ├── 📄 ConfirmDialog.tsx
│           │   │   └── 📄 EmptyState.tsx
│           │   │
│           │   └── 📁 ui/                    # Base UI primitives
│           │       ├── 📄 Button.tsx
│           │       ├── 📄 Input.tsx
│           │       ├── 📄 Textarea.tsx
│           │       ├── 📄 Select.tsx
│           │       ├── 📄 Checkbox.tsx
│           │       ├── 📄 Radio.tsx
│           │       ├── 📄 Switch.tsx
│           │       ├── 📄 Slider.tsx
│           │       ├── 📄 Tabs.tsx
│           │       ├── 📄 Accordion.tsx
│           │       ├── 📄 Tooltip.tsx
│           │       ├── 📄 Dropdown.tsx
│           │       ├── 📄 Avatar.tsx
│           │       ├── 📄 Badge.tsx
│           │       ├── 📄 Card.tsx
│           │       ├── 📄 Alert.tsx
│           │       ├── 📄 Skeleton.tsx
│           │       ├── 📄 Progress.tsx
│           │       └── 📄 Separator.tsx
│           │
│           ├── 📁 pages/                     # Page Components [CB §52]
│           │   ├── 📄 ChatPage.tsx           # Main chat page
│           │   ├── 📄 SettingsPage.tsx       # Settings page
│           │   ├── 📄 KnowledgeBasePage.tsx  # KB page
│           │   ├── 📄 AdminPage.tsx          # Admin page
│           │   └── 📄 SetupPage.tsx          # Setup wizard
│           │
│           ├── 📁 lib/                       # Utility Libraries
│           │   ├── 📄 api.ts                 # API client wrapper
│           │   ├── 📄 sse.ts                 # SSE client
│           │   ├── 📄 preferences.ts         # Preferences management
│           │   ├── 📄 auth.ts                # Auth utilities
│           │   ├── 📄 types.ts               # TypeScript types
│           │   ├── 📄 constants.ts           # Constants
│           │   └── 📄 utils.ts               # General utilities
│           │
│           ├── 📁 hooks/                     # SolidJS Hooks
│           │   ├── 📄 useCases.ts            # Case management
│           │   ├── 📄 useMessages.ts         # Message handling
│           │   ├── 📄 usePreferences.ts      # Preferences
│           │   ├── 📄 useKnowledgeBase.ts    # Knowledge Base
│           │   ├── 📄 useCommandPalette.ts   # Command palette
│           │   ├── 📄 useCaseGraph.ts        # Case Graph
│           │   ├── 📄 useStreaming.ts        # SSE streaming
│           │   └── 📄 useAuth.ts             # Authentication
│           │
│           ├── 📁 stores/                    # Global State (SolidJS)
│           │   ├── 📄 caseStore.ts           # Current case state
│           │   ├── 📄 messageStore.ts        # Messages state
│           │   ├── 📄 preferenceStore.ts     # UI preferences
│           │   ├── 📄 authStore.ts           # Auth state
│           │   └── 📄 modeStore.ts           # Reasoning mode state
│           │
│           ├── 📁 locales/                   # i18n [CB §28]
│           │   ├── 📄 id.json                # Bahasa Indonesia
│           │   └── 📄 en.json                # English
│           │
│           └── 📁 styles/                    # Global Styles [CB §54]
│               ├── 📄 globals.css            # Global CSS
│               └── 📁 themes/
│                   ├── 📄 light.css          # Light theme
│                   ├── 📄 dark.css           # Dark theme
│                   ├── 📄 sepia.css          # Sepia theme
│                   └── 📄 high-contrast.css  # High contrast (a11y)
│
│
│ ══════════════════════════════════════════════
│  SHARED PACKAGES
│ ══════════════════════════════════════════════
│
├── 📁 packages/
│   ├── 📁 shared/                            # Shared Types [§6.1]
│   │   ├── 📄 package.json
│   │   ├── 📄 tsconfig.json
│   │   └── 📁 src/
│   │       ├── 📄 index.ts                   # Barrel export
│   │       ├── 📁 types/
│   │       │   ├── 📄 case.ts                # Case types [CB §20]
│   │       │   ├── 📄 message.ts             # Message types
│   │       │   ├── 📄 case_graph.ts          # Case Graph types [CB §22]
│   │       │   ├── 📄 knowledge.ts           # Knowledge Base types [CB §24]
│   │       │   ├── 📄 provider.ts            # LLM provider types [CB §26]
│   │       │   ├── 📄 reasoning.ts           # Reasoning mode types [CB §21]
│   │       │   ├── 📄 interpretation.ts      # 7-layer types [CB §7]
│   │       │   ├── 📄 export.ts              # Export types [CB §34]
│   │       │   └── 📄 api.ts                 # API request/response
│   │       ├── 📁 validators/
│   │       │   ├── 📄 case.validator.ts
│   │       │   ├── 📄 message.validator.ts
│   │       │   └── 📄 citation.validator.ts
│   │       └── 📁 constants/
│   │           ├── 📄 case_states.ts         # Case state enum
│   │           ├── 📄 reasoning_modes.ts     # Reasoning mode enum
│   │           ├── 📄 node_types.ts          # Graph node types
│   │           ├── 📄 edge_types.ts          # Graph edge types
│   │           └── 📄 legal_hierarchy.ts     # Legal hierarchy levels
│   │
│   └── 📁 config/                            # Shared Configs [§6.2]
│       ├── 📄 package.json
│       ├── 📁 eslint/
│       │   └── 📄 eslint-config-paugeran.js
│       ├── 📁 prettier/
│       │   └── 📄 prettier-config-paugeran.js
│       └── 📁 typescript/
│           └── 📄 tsconfig.base.json
│
│
│ ══════════════════════════════════════════════
│  INFRASTRUCTURE
│ ══════════════════════════════════════════════
│
├── 📁 infra/
│   ├── 📁 docker/                            # Docker configs [§7.1]
│   │   ├── 📄 Dockerfile                     # Multi-stage build
│   │   ├── 📄 Dockerfile.dev                 # Development Dockerfile
│   │   ├── 📄 docker-compose.yml             # Dev compose
│   │   ├── 📄 docker-compose.prod.yml        # Production compose
│   │   └── 📄 .dockerignore
│   │
│   ├── 📁 railway/                           # Railway deployment [§7.2]
│   │   └── 📄 railway.json
│   │
│   ├── 📁 scripts/                           # Build & Deploy Scripts [§7.3]
│   │   ├── 📄 build.sh                       # Build single binary
│   │   ├── 📄 build-all-platforms.sh         # Cross-platform build
│   │   ├── 📄 release.sh                     # Release automation
│   │   ├── 📄 setup-dev.sh                   # Development setup
│   │   └── 📄 install.sh                     # User install script
│   │
│   └── 📁 legal_data/                        # Seed Legal Data [§7.4]
│       ├── 📄 seed_knowledge_base.json       # Initial KB entries
│       ├── 📄 seed_norm_conflicts.json       # Known norm conflicts
│       ├── 📄 seed_precedents.json           # Key precedents
│       ├── 📄 seed_daluwarsa_rules.json      # Statute of limitations
│       └── 📄 seed_customary_law.json        # Customary law entries
│
│
│ ══════════════════════════════════════════════
│  DOCUMENTATION
│ ══════════════════════════════════════════════
│
├── 📁 docs/
│   ├── 📁 prd/
│   │   └── 📄 master-specification.md        # Master Spec [CB]
│   │
│   ├── 📁 architecture/
│   │   ├── 📄 repository-architecture.md     # This document
│   │   ├── 📄 system-design.md               # System design overview
│   │   ├── 📄 case-graph.md                  # Case Graph design
│   │   ├── 📄 reasoning-engine.md            # Reasoning engine design
│   │   ├── 📄 legal-modules.md               # Legal modules design
│   │   └── 📁 diagrams/
│   │       ├── 📄 architecture-overview.png
│   │       ├── 📄 case-graph-example.png
│   │       ├── 📄 reasoning-flow.png
│   │       ├── 📄 mode-switching.png
│   │       └── 📄 data-flow.png
│   │
│   ├── 📁 api/
│   │   └── 📄 openapi.yaml                   # OpenAPI 3.1 spec [CB §39]
│   │
│   ├── 📁 deployment/
│   │   ├── 📄 single-binary.md               # Single binary guide
│   │   ├── 📄 docker.md                      # Docker deployment
│   │   ├── 📄 railway.md                     # Railway deployment
│   │   └── 📄 vps.md                         # VPS deployment
│   │
│   ├── 📁 development/
│   │   ├── 📄 getting-started.md             # Developer onboarding
│   │   ├── 📄 coding-standards.md            # Coding standards
│   │   ├── 📄 testing-guide.md               # Testing guide
│   │   ├── 📄 adding-reasoning-mode.md       # How to add new mode
│   │   ├── 📄 adding-legal-module.md         # How to add new module
│   │   └── 📄 adding-llm-provider.md         # How to add new provider
│   │
│   └── 📁 legal/
│       ├── 📄 terms-of-service.md            # ToS draft
│       ├── 📄 privacy-policy.md              # Privacy policy draft
│       └── 📄 disclaimer.md                  # Legal disclaimer
│
│
│ ══════════════════════════════════════════════
│  SCRIPTS & TOOLS
│ ══════════════════════════════════════════════
│
└── 📁 scripts/                               # Development Scripts [§8]
    ├── 📄 dev.sh                             # Start development
    ├── 📄 test.sh                            # Run all tests
    ├── 📄 format.sh                          # Format all code
    ├── 📄 lint.sh                            # Lint all code
    ├── 📄 clean.sh                           # Clean build artifacts
    └── 📄 seed-db.sh                         # Seed database with legal data
```

---

## 3. KONFIGURASI ROOT PROJECT

### 3.1 Root `Cargo.toml` (Rust Workspace)

```toml
[workspace]
members = ["apps/server"]
resolver = "2"

[workspace.package]
version = "1.0.0"
edition = "2021"
rust-version = "1.75"
authors = ["PAUGERAN Team"]
license = "MIT"
repository = "https://github.com/paugeran/paugeran"
description = "Supreme Legal Reasoning Engine for Indonesian Law"

[workspace.dependencies]
# === Web Framework ===
axum = { version = "0.7", features = ["ws", "multipart", "macros"] }
tokio = { version = "1.35", features = ["full"] }
tower = { version = "0.4" }
tower-http = { version = "0.5", features = ["cors", "trace", "fs", "compression-gzip"] }

# === Database ===
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "json", "chrono", "uuid"] }

# === Graph Processing ===
petgraph = { version = "0.6", features = ["serde-1"] }

# === Serialization ===
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0" }

# === HTTP Client ===
reqwest = { version = "0.12", features = ["json", "stream", "rustls-tls"], default-features = false }

# === HTML Parsing ===
scraper = "0.19"

# === Error Handling ===
thiserror = "1.0"
anyhow = "1.0"

# === Logging & Tracing ===
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json", "chrono"] }

# === Encryption & Security ===
aes-gcm = "0.10"
argon2 = "0.5"
jsonwebtoken = "9.2"

# === Utilities ===
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
rand = "0.8"
base64 = "0.22"
regex = "1.10"
url = "2.5"

# === Async ===
async-trait = "0.1"
tokio-stream = { version = "0.1", features = ["sync"] }
futures = "0.3"

# === Document Processing ===
lopdf = "0.32"
docx-rs = "0.4"
printpdf = "0.7"

# === Frontend Embedding ===
rust-embed = { version = "8.3", features = ["compression"] }
mime_guess = "2.0"

# === System ===
open = "5.0"
dirs = "5.0"

# === Testing ===
mockall = "0.12"
wiremock = "0.6"
tempfile = "3.10"
```

### 3.2 Root `package.json`

```json
{
  "name": "paugeran",
  "version": "1.0.0",
  "private": true,
  "description": "Supreme Legal Reasoning Engine for Indonesian Law",
  "scripts": {
    "dev": "pnpm --filter @paugeran/web dev",
    "build": "pnpm --filter @paugeran/web build",
    "build:server": "cd apps/server && cargo build --release",
    "build:all": "pnpm build && pnpm build:server",
    "test": "pnpm --filter @paugeran/web test",
    "test:server": "cd apps/server && cargo test",
    "test:all": "pnpm test && pnpm test:server",
    "lint": "pnpm --filter @paugeran/web lint",
    "lint:server": "cd apps/server && cargo clippy -- -D warnings",
    "lint:all": "pnpm lint && pnpm lint:server",
    "format": "prettier --write \"apps/web/**/*.{ts,tsx,css}\" \"packages/**/*.ts\"",
    "format:server": "cd apps/server && cargo fmt",
    "format:all": "pnpm format && pnpm format:server",
    "clean": "pnpm --filter @paugeran/web exec rm -rf dist && cd apps/server && cargo clean",
    "typecheck": "pnpm --filter @paugeran/web exec tsc --noEmit"
  },
  "devDependencies": {
    "prettier": "^3.2.0",
    "@paugeran/eslint-config": "workspace:*",
    "@paugeran/typescript-config": "workspace:*"
  },
  "packageManager": "pnpm@9.1.0",
  "engines": {
    "node": ">=20.0.0"
  }
}
```

### 3.3 `pnpm-workspace.yaml`

```yaml
packages:
  - 'apps/*'
  - 'packages/*'
```

### 3.4 `turbo.json` (Optional)

```json
{
  "$schema": "https://turbo.build/schema.json",
  "globalDependencies": [".env", ".env.local"],
  "tasks": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**"]
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "test": {
      "dependsOn": ["build"]
    },
    "lint": {},
    "typecheck": {}
  }
}
```

### 3.5 `.gitignore`

```gitignore
# === Rust ===
/target/
**/*.rs.bk
*.pdb

# === Node.js ===
node_modules/
dist/
.next/
.turbo/

# === Environment ===
.env
.env.local
.env.*.local
!.env.example
!.env.test

# === IDE ===
.vscode/
.idea/
*.swp
*.swo
*~

# === OS ===
.DS_Store
Thumbs.db
Desktop.ini

# === Data ===
*.db
*.sqlite
*.sqlite3
/data/

# === Secrets ===
*.pem
*.key
*.secret
.secrets/

# === Logs ===
*.log
logs/

# === Build ===
*.o
*.a
*.so
*.dylib
*.dll
*.exe

# === Coverage ===
coverage/
*.lcov
tarpaulin-report.html

# === Temp ===
*.tmp
*.temp
.cache/
```

### 3.6 `.editorconfig`

```ini
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space
indent_size = 4

[*.{ts,tsx,js,jsx,json,css,html,yml,yaml}]
indent_size = 2

[*.rs]
indent_size = 4

[*.md]
trim_trailing_whitespace = false

[Makefile]
indent_style = tab
```

---

## 4. BACKEND RUST — STRUKTUR & KONVENSI

### 4.1 `apps/server/Cargo.toml`

```toml
[package]
name = "paugeran"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
description = "PAUGERAN Supreme Legal Reasoning Engine"

[[bin]]
name = "paugeran"
path = "src/main.rs"

[dependencies]
# All from workspace
axum.workspace = true
tokio.workspace = true
tower.workspace = true
tower-http.workspace = true
sqlx.workspace = true
petgraph.workspace = true
serde.workspace = true
serde_json.workspace = true
reqwest.workspace = true
scraper.workspace = true
thiserror.workspace = true
anyhow.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
aes-gcm.workspace = true
argon2.workspace = true
jsonwebtoken.workspace = true
uuid.workspace = true
chrono.workspace = true
rand.workspace = true
base64.workspace = true
regex.workspace = true
url.workspace = true
async-trait.workspace = true
tokio-stream.workspace = true
futures.workspace = true
lopdf.workspace = true
docx-rs.workspace = true
printpdf.workspace = true
rust-embed.workspace = true
mime_guess.workspace = true
open.workspace = true
dirs.workspace = true

[dev-dependencies]
mockall.workspace = true
wiremock.workspace = true
tempfile.workspace = true

[features]
default = []
auth = []  # Enable multi-user authentication

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

### 4.2 `apps/server/build.rs`

```rust
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../web/src");
    println!("cargo:rerun-if-changed=../web/package.json");
    println!("cargo:rerun-if-changed=../web/vite.config.ts");
    
    // Build frontend before compiling backend
    let status = Command::new("pnpm")
        .args(["--filter", "@paugeran/web", "build"])
        .current_dir("../..")
        .status()
        .expect("Failed to build frontend. Ensure pnpm is installed.");
    
    if !status.success() {
        panic!("Frontend build failed. Check apps/web for errors.");
    }
    
    println!("cargo:warning=Frontend built successfully, embedding into binary...");
}
```

### 4.3 `apps/server/src/main.rs`

```rust
//! PAUGERAN — Supreme Legal Reasoning Engine
//! 
//! Entry point for the single binary application.
//! Initializes configuration, database, and HTTP server.

use paugeran::config::AppConfig;
use paugeran::http::app::create_app;
use paugeran::database::pool::create_pool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "paugeran=info,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
    
    tracing::info!("🚀 PAUGERAN v{} starting...", env!("CARGO_PKG_VERSION"));
    
    // Load configuration
    let config = AppConfig::from_env();
    tracing::info!("📂 Data directory: {}", config.data_dir.display());
    
    // Initialize database
    let pool = create_pool(&config).await?;
    tracing::info!("✅ Database initialized");
    
    // Create application
    let app = create_app(config.clone(), pool).await?;
    
    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let url = format!("http://localhost:{}", config.port);
    
    tracing::info!("✅ Server running at {}", url);
    println!("🚀 PAUGERAN ready at {}", url);
    
    // Auto-open browser
    if let Err(e) = open::that(&url) {
        tracing::warn!("Could not open browser: {}. Open {} manually.", e, url);
    }
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

### 4.4 `apps/server/src/lib.rs`

```rust
//! PAUGERAN Library
//! 
//! Re-exports all modules for use in tests and benchmarks.

pub mod config;
pub mod error;
pub mod http;
pub mod engine;
pub mod case_graph;
pub mod llm;
pub mod knowledge_base;
pub mod web_research;
pub mod database;
pub mod export;
pub mod crypto;
pub mod frontend;
pub mod utils;
```

### 4.5 `apps/server/src/error.rs`

```rust
//! Global error types for PAUGERAN
//! 
//! All errors implement `IntoResponse` for Axum integration.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("LLM error: {0}")]
    Llm(String),
    
    #[error("No LLM provider configured")]
    NoLlmProvider,
    
    #[error("Case isolation violation")]
    IsolationViolation,
    
    #[error("Citation validation failed: {0}")]
    CitationValidation(String),
    
    #[error("Procedural check failed: {0}")]
    ProceduralCheck(String),
    
    #[error("Export error: {0}")]
    Export(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code) = match &self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::Llm(_) => (StatusCode::BAD_GATEWAY, "LLM_ERROR"),
            AppError::NoLlmProvider => (StatusCode::SERVICE_UNAVAILABLE, "NO_LLM_PROVIDER"),
            AppError::IsolationViolation => (StatusCode::FORBIDDEN, "ISOLATION_VIOLATION"),
            AppError::CitationValidation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "CITATION_ERROR"),
            AppError::ProceduralCheck(_) => (StatusCode::UNPROCESSABLE_ENTITY, "PROCEDURAL_ERROR"),
            AppError::Export(_) => (StatusCode::INTERNAL_SERVER_ERROR, "EXPORT_ERROR"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };
        
        let body = json!({
            "error": {
                "code": code,
                "message": self.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });
        
        (status, Json(body)).into_response()
    }
}
```

### 4.6 Konvensi Module Backend

**Setiap module di `src/` harus mengikuti struktur:**
```
src/<module>/
├── mod.rs          # Re-exports dan module-level documentation
├── <submodule>.rs  # Implementasi
└── ...
```

**Setiap `mod.rs` harus berisi:**
```rust
//! Module-level documentation
//! 
//! [CB §X.Y] — Reference to Master Specification

pub mod submodule;
pub use submodule::PublicType;
```

**Setiap file implementasi harus berisi:**
```rust
//! File-level documentation
//! 
//! Implements [CB §X.Y] — Feature name

use crate::error::AppError;

/// Public function documentation
/// 
/// # Arguments
/// * `param` - Description
/// 
/// # Errors
/// Returns `AppError::X` when...
pub async fn function_name(param: &str) -> Result<ReturnType, AppError> {
    // Implementation
}
```

---

## 5. FRONTEND SOLIDJS — STRUKTUR & KONVENSI

### 5.1 `apps/web/package.json`

```json
{
  "name": "@paugeran/web",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "test": "vitest run",
    "test:watch": "vitest",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "solid-js": "^1.8.0",
    "@solidjs/router": "^0.13.0",
    "@tanstack/solid-query": "^5.0.0",
    "cytoscape": "^3.28.0",
    "cytoscape-popper": "^2.0.0",
    "@tiptap/core": "^2.4.0",
    "@tiptap/starter-kit": "^2.4.0",
    "marked": "^12.0.0",
    "date-fns": "^3.6.0",
    "clsx": "^2.1.0",
    "@solid-primitives/i18n": "^2.1.0",
    "@solid-primitives/keyboard": "^1.2.0",
    "@solid-primitives/storage": "^3.0.0"
  },
  "devDependencies": {
    "typescript": "^5.4.0",
    "vite": "^5.2.0",
    "vite-plugin-solid": "^2.10.0",
    "@types/cytoscape": "^3.21.0",
    "tailwindcss": "^3.4.0",
    "postcss": "^8.4.0",
    "autoprefixer": "^10.4.0",
    "eslint": "^8.57.0",
    "@typescript-eslint/eslint-plugin": "^7.0.0",
    "@typescript-eslint/parser": "^7.0.0",
    "eslint-plugin-solid": "^0.14.0",
    "vitest": "^1.6.0",
    "@solidjs/testing-library": "^0.8.0"
  }
}
```

### 5.2 `apps/web/tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true,
    "jsx": "preserve",
    "jsxImportSource": "solid-js",
    "types": ["vite/client"],
    "noEmit": true,
    "isolatedModules": true,
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "paths": {
      "@/*": ["./src/*"],
      "@paugeran/shared": ["../../packages/shared/src"]
    }
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### 5.3 `apps/web/vite.config.ts`

```typescript
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import path from 'path';

export default defineConfig({
  plugins: [solidPlugin()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@paugeran/shared': path.resolve(__dirname, '../../packages/shared/src'),
    },
    conditions: ['development', 'browser'],
  },
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
      },
    },
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    sourcemap: false,
    minify: 'terser',
  },
});
```

### 5.4 `apps/web/tailwind.config.js`

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
        },
        legal: {
          gold: '#b8860b',
          navy: '#1e3a5f',
          cream: '#faf8f0',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        serif: ['Georgia', 'Times New Roman', 'serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
};
```

### 5.5 Konvensi Komponen Frontend

**Setiap komponen harus mengikuti struktur:**
```tsx
/**
 * ComponentName — Brief description
 * 
 * [CB §X.Y] — Reference to Master Specification
 * 
 * @example
 * <ComponentName prop1="value" />
 */

import { Component, createSignal } from 'solid-js';

interface ComponentNameProps {
  prop1: string;
  onAction?: () => void;
}

const ComponentName: Component<ComponentNameProps> = (props) => {
  // Implementation
  return (
    <div class="component-name">
      {/* ... */}
    </div>
  );
};

export default ComponentName;
```

---

## 6. SHARED PACKAGES — STRUKTUR & KONVENSI

### 6.1 `packages/shared/`

Berisi TypeScript types dan validators yang digunakan oleh frontend dan dapat di-generate dari Rust types.

**Konvensi:**
- Setiap type file harus mirror Rust struct di `apps/server/src/database/models/`
- Gunakan `zod` atau manual validators untuk runtime validation
- Export semua types dari `index.ts`

### 6.2 `packages/config/`

Berisi shared ESLint, Prettier, dan TypeScript configurations.

---

## 7. INFRASTRUCTURE — STRUKTUR & KONVENSI

### 7.1 `infra/docker/Dockerfile`

```dockerfile
# Stage 1: Build Frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY apps/web/package.json apps/web/
COPY packages/ packages/
RUN corepack enable && pnpm install --frozen-lockfile
COPY apps/web/ apps/web/
RUN pnpm --filter @paugeran/web build

# Stage 2: Build Backend
FROM rust:1.75-bookworm AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY apps/server/ apps/server/
COPY --from=frontend-builder /app/apps/web/dist apps/web/dist
RUN cargo build --release --bin paugeran

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=backend-builder /app/target/release/paugeran /usr/local/bin/paugeran
RUN mkdir -p /data && chmod 777 /data
VOLUME ["/data"]
ENV DATA_DIR=/data PORT=3000
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=5s --retries=3 CMD curl -f http://localhost:3000/health || exit 1
RUN useradd -m -u 1000 paugeran && chown -R paugeran:paugeran /data
USER paugeran
CMD ["paugeran"]
```

---

## 8-17. RINGKASAN BAGIAN LANJUTAN

### 8. Build System & Pipeline
- **Development:** `pnpm dev` (frontend) + `cargo run` (backend) terpisah
- **Production:** `cargo build --release` menghasilkan single binary dengan embedded frontend
- **Cross-platform:** Gunakan `cross` crate atau GitHub Actions matrix

### 9. Testing Architecture
- **Unit Tests:** Co-located di setiap Rust module (`#[cfg(test)]`)
- **Integration Tests:** `apps/server/tests/` dengan test database terpisah
- **E2E Tests:** Frontend menggunakan `@solidjs/testing-library`
- **Coverage Target:** >80% untuk Rust, >70% untuk TypeScript

### 10. CI/CD Pipeline
- **CI:** Lint → Typecheck → Test → Build (setiap PR)
- **Release:** Tag `v*` → Build all platforms → Upload to GitHub Releases
- **Security:** `cargo audit` + `pnpm audit` (weekly)

### 11. Konvensi Penamaan
- **Rust:** snake_case (files, functions), PascalCase (structs, enums), SCREAMING_SNAKE_CASE (constants)
- **TypeScript:** PascalCase (components, types), camelCase (functions, variables), kebab-case (CSS classes)
- **Database:** snake_case (tables, columns)
- **API:** kebab-case (URL paths), camelCase (JSON fields)

### 12. Dependency Management
- **Rust:** Semua dependency di `Cargo.toml` workspace, version pinned
- **Node.js:** Semua dependency di `package.json`, lock file committed
- **Audit:** `cargo audit` dan `pnpm audit` dijalankan di CI

### 13. Data & Storage Architecture
- **Database:** SQLite di `{data_dir}/paugeran.db`
- **Documents:** `{data_dir}/documents/{user_id}/{case_id}/`
- **Exports:** `{data_dir}/exports/{user_id}/`
- **Secrets:** `{data_dir}/.secret` (permission 600)
- **Logs:** `{data_dir}/logs/`

### 14. Cross-Platform Build Matrix

| Platform | Target | Binary Name |
|----------|--------|-------------|
| Linux x86_64 | `x86_64-unknown-linux-gnu` | `paugeran-linux` |
| macOS Intel | `x86_64-apple-darwin` | `paugeran-macos-intel` |
| macOS ARM | `aarch64-apple-darwin` | `paugeran-macos-arm` |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `paugeran-windows.exe` |

### 15. Release & Distribution
- **GitHub Releases:** Binary per platform + checksums
- **Docker Hub:** `paugeran/paugeran:latest`
- **Install Script:** `curl -fsSL https://get.paugeran.com | bash`

---

## 17. PENUTUP

Dokumen **Spesifikasi Arsitektur Repositori** ini adalah panduan teknis yang mengatur seluruh organisasi kode proyek PAUGERAN. Setiap developer, baik manusia maupun AI agent, **wajib** mengikuti struktur, konvensi, dan prinsip yang ditetapkan di sini.

**Hubungan dengan dokumen lain:**
- Dokumen ini merujuk ke **MASTER SPECIFICATION v1.0 FINAL** [CB] sebagai sumber kebenaran fungsional
- Dokumen ini menjadi acuan untuk **SPEC-ARCH** (System Design) dan **SPEC-DB** (Database Schema)
- Setiap perubahan pada struktur repositori harus di-update di dokumen ini terlebih dahulu

---

**Dokumen:** PAUGERAN — Repository Architecture Specification  
**Versi:** 1.0  
**Referensi:** MASTER SPECIFICATION v1.0 FINAL [CB]  
**Status:** FINAL — SIAP IMPLEMENTASI  
**Tanggal:** 28 Agustus 2026
