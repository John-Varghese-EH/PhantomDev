# Graph Report - .  (2026-04-29)

## Corpus Check
- Corpus is ~4,446 words - fits in a single context window. You may not need a graph.

## Summary
- 111 nodes · 172 edges · 7 communities detected
- Extraction: 83% EXTRACTED · 17% INFERRED · 0% AMBIGUOUS · INFERRED: 29 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Core Types|Core Types]]
- [[_COMMUNITY_Detection Engine|Detection Engine]]
- [[_COMMUNITY_Project Concepts|Project Concepts]]
- [[_COMMUNITY_CLI & TUI|CLI & TUI]]
- [[_COMMUNITY_Jitter Engine|Jitter Engine]]
- [[_COMMUNITY_Humanizer Engine|Humanizer Engine]]
- [[_COMMUNITY_Configuration|Configuration]]

## God Nodes (most connected - your core abstractions)
1. `PhantomDev` - 8 edges
2. `PhantomDetector` - 7 edges
3. `main()` - 7 edges
4. `cmd_scan()` - 7 edges
5. `cmd_humanize()` - 7 edges
6. `PhantomJitter` - 6 edges
7. `cmd_score()` - 6 edges
8. `read_code_block()` - 6 edges
9. `Config` - 5 edges
10. `PhantomHumanizer` - 5 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `cmd_config()`  [EXTRACTED]
  crates/cli/src/main.rs → crates/cli/src/main.rs  _Bridges community 3 → community 6_

## Hyperedges (group relationships)
- **PhantomDev Crates** — core, detector, humanizer, jitter, cli, tui [EXTRACTED 1.00]
- **PhantomDev Features** — ai_detection, code_humanization, temporal_obfuscation, stealth_scoring, tui_dashboard [EXTRACTED 1.00]

## Communities

### Community 0 - "Core Types"
Cohesion: 0.09
Nodes (20): ApiConfig, CodeBlock, CommentStyle, CommitFormat, CommitMessage, DetectionConfig, DetectionResult, Detector (+12 more)

### Community 1 - "Detection Engine"
Cohesion: 0.17
Nodes (9): CloudClient, CloudDetector, DetectorConfig, is_emoji(), LocalModel, PhantomDetector, RobertaDetector, test_detector_creation() (+1 more)

### Community 2 - "Project Concepts"
Cohesion: 0.14
Nodes (21): AI Detection, Anthropic API, Candle ML Framework, CLI Crate, Code Humanization, Core Crate, Detector Crate, Entropy Injection (+13 more)

### Community 3 - "CLI & TUI"
Cohesion: 0.22
Nodes (12): Language, PhantomTui, Cli, cmd_dashboard(), cmd_humanize(), cmd_init(), cmd_scan(), cmd_score() (+4 more)

### Community 4 - "Jitter Engine"
Cohesion: 0.27
Nodes (5): JitterConfig, PhantomJitter, test_apply_jitter(), test_jitter_creation(), test_jitter_enabled()

### Community 5 - "Humanizer Engine"
Cohesion: 0.32
Nodes (2): HumanizerConfig, PhantomHumanizer

### Community 6 - "Configuration"
Cohesion: 0.6
Nodes (2): Config, cmd_config()

## Knowledge Gaps
- **28 isolated node(s):** `CodeBlock`, `CommitMessage`, `StyleProfile`, `NamingConvention`, `CommentStyle` (+23 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `Humanizer Engine`** (8 nodes): `lib.rs`, `HumanizerConfig`, `.default()`, `PhantomHumanizer`, `.humanize()`, `.learn_style()`, `.new()`, `.with_config()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Configuration`** (6 nodes): `Config`, `.default()`, `.load()`, `.load_or_default()`, `.save()`, `cmd_config()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `cmd_scan()` connect `CLI & TUI` to `Core Types`, `Detection Engine`?**
  _High betweenness centrality (0.187) - this node is a cross-community bridge._
- **Why does `cmd_humanize()` connect `CLI & TUI` to `Humanizer Engine`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `cmd_scan()` (e.g. with `.new()` and `.detect()`) actually correct?**
  _`cmd_scan()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `cmd_humanize()` (e.g. with `.new()` and `.learn_style()`) actually correct?**
  _`cmd_humanize()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CodeBlock`, `CommitMessage`, `StyleProfile` to the rest of the system?**
  _28 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core Types` be split into smaller, more focused modules?**
  _Cohesion score 0.09 - nodes in this community are weakly interconnected._
- **Should `Project Concepts` be split into smaller, more focused modules?**
  _Cohesion score 0.14 - nodes in this community are weakly interconnected._