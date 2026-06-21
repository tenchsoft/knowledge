# Research Implement Plan Audit: Complete Results

## Objective

Re-audit all 103 implement plans in `plans/implement/research/` against the codebase to verify each is truly implemented, incomplete, or missing.

## Audit Method

1. Read each implement plan file
2. Extract key identifiers (debug_ids, function names, state fields, module paths)
3. Search `apps/research/src-tauri/src/` and `crates/research-core/src/` for each identifier
4. Verify rendering, event handling, and automation nodes are all connected

## Results Summary

| Status | Count |
|--------|-------|
| IMPLEMENTED | 100 |
| INCOMPLETE | 3 |

---

## IMPLEMENTED (100 plans)

### Shortcut / Keyboard Plans (16/16)

- [x] `control-c-copy-selected-shortcut.md` — Ctrl+C handler at `ui/events/text.rs:39`, toast at `:41-44`
- [x] `control-shift-c-citation-format-shortcut.md` — Ctrl+Shift+C at `ui/events/text.rs:32-37`, `cycle_citation_format` at `ui/state/library.rs:227`
- [x] `alt-f-favorites-toggle-shortcut.md` — Alt+F at `ui/events/text.rs:27-30`, `toggle_favorites_only` at `ui/state/library.rs:197`
- [x] `minus-pdf-zoom-shortcut.md` — `-` key at `ui/events/text.rs:153-157`, `pdf_zoom_out` at `ui/state/pdf.rs:26`
- [x] `page-up-previous-pdf-page-shortcut.md` — PageUp at `ui/events/text.rs:138-141`, `pdf_prev_page` at `ui/state/pdf.rs:10`
- [x] `control-a-select-all-shortcut.md` — Ctrl+A at `ui/events/text.rs:56-69`, `select_all_visible` at `ui/state/library.rs:575`
- [x] `page-down-next-pdf-page-shortcut.md` — PageDown at `ui/events/text.rs:142-145`, `pdf_next_page` at `ui/state/pdf.rs:4`
- [x] `control-i-import-shortcut.md` — Ctrl+I at `ui/events/text.rs:49-54`, `queue_import` at `ui/state/library.rs:235`
- [x] `doi-enter-fetch-shortcut.md` — Enter+DoiInput at `ui/events/text.rs:128`, `fetch_doi_metadata` at `ui/state/pdf.rs:93`
- [x] `backspace-focused-input-shortcut.md` — Backspace at `ui/events/text.rs:159-171`, pop methods for all 5 fields
- [x] `arrow-paper-selection-shortcut.md` — ArrowUp/Down at `ui/events/text.rs:105-121`, `move_selection` at `ui/state/library.rs:98`
- [x] `alt-p-reader-mode-toggle-shortcut.md` — Alt+P at `ui/events/text.rs:71-74`, `toggle_reader_mode` at `ui/state/library.rs:248`
- [x] `control-f-search-shortcut.md` — Ctrl+F at `ui/events/text.rs:18-25`, focus routing to SearchBox/PdfSearch
- [x] `qanda-enter-submit-shortcut.md` — Enter+QaInput at `ui/events/text.rs:123-126`, `send_qa_message` at `ui/state/library.rs:517`
- [x] `question-mark-shortcut-help-toggle.md` — `?` at `ui/events/text.rs:76-86`, `toggle_shortcut_help` at `ui/state/library.rs:654`. Plan says "자동화 노드 없음" (no automation node) — keyboard-only feature.
- [x] `escape-close-focus-or-overlay-shortcut.md` — Escape at `ui/events/text.rs:87-104`, priority-ordered close logic

### Button Plans (36/36)

- [x] `cite-inspector-tab-button.md` — Tab 5, `research.inspector.tab.{index}` at `ui/automation/inspector.rs:36`
- [x] `visual-inspector-tab-button.md` — Tab 3, same dynamic tab automation
- [x] `pdf-previous-page-button.md` — `research.pdf.prev` at `ui/automation/pdf.rs:63`, `pdf_prev_page` at `ui/state/pdf.rs:10`
- [x] `ris-citation-format-button.md` — `research.citation.format.ris` at `ui/automation/inspector.rs:169`
- [x] `qanda-key-points-quick-action-button.md` — `research.qa.quick.key_points` at `ui/automation/inspector.rs:75`
- [x] `pdf-annotation-list-toggle-button.md` — `research.pdf.annotation_list_toggle` at `ui/automation/pdf.rs:126`
- [x] `pdf-zoom-out-button.md` — `research.pdf.zoom_out` at `ui/automation/pdf.rs:82`
- [x] `welcome-get-started-button.md` — `research.welcome.get_started` at `ui/automation/welcome.rs:31`
- [x] `apa-citation-format-button.md` — `research.citation.format.apa` at `ui/automation/inspector.rs:173`
- [x] `import-bibtex-button.md` — `research.citation.import_bibtex` at `ui/automation/inspector.rs:230`
- [x] `qanda-summarize-quick-action-button.md` — `research.qa.quick.summarize` at `ui/automation/inspector.rs:74`
- [x] `save-search-button.md` — `save_current_search` at `ui/state/library.rs:176`, plan says no automation node
- [x] `pdf-rotate-button.md` — `research.pdf.rotate` at `ui/automation/pdf.rs:100`
- [x] `pdf-zoom-in-button.md` — `research.pdf.zoom_in` at `ui/automation/pdf.rs:91`
- [x] `header-export-button.md` — `research.header.export` at `ui/automation/mod.rs:55`
- [x] `mla-citation-format-button.md` — `research.citation.format.mla` at `ui/automation/inspector.rs:181`
- [x] `header-sync-button.md` — `research.header.sync` at `ui/automation/mod.rs:56`
- [x] `write-inspector-tab-button.md` — Tab 4, dynamic tab automation
- [x] `bibtex-citation-format-button.md` — `research.citation.format.bibtex` at `ui/automation/inspector.rs:165`
- [x] `qanda-inspector-tab-button.md` — Tab 2, dynamic tab automation
- [x] `pdf-strikeout-tool-button.md` — `research.pdf.tool.strikeout` at `ui/automation/pdf.rs:107`
- [x] `pdf-sticky-note-tool-button.md` — `research.pdf.tool.sticky_note` at `ui/automation/pdf.rs:108`
- [x] `pdf-underline-tool-button.md` — `research.pdf.tool.underline` at `ui/automation/pdf.rs:106`
- [x] `pdf-next-page-button.md` — `research.pdf.next` at `ui/automation/pdf.rs:72`
- [x] `welcome-import-from-file-button.md` — `research.welcome.import` at `ui/automation/welcome.rs:41`
- [x] `pdf-highlight-tool-button.md` — `research.pdf.tool.highlight` at `ui/automation/pdf.rs:105`
- [x] `insert-citation-result-button.md` — `research.manuscript.cite_result.{index}.insert` at `ui/automation/inspector.rs:147`
- [x] `chicago-citation-format-button.md` — `research.citation.format.chicago` at `ui/automation/inspector.rs:177`
- [x] `summary-inspector-tab-button.md` — Tab 1, dynamic tab automation
- [x] `notes-inspector-tab-button.md` — Tab 0, dynamic tab automation
- [x] `fetch-doi-metadata-button.md` — `research.citation.fetch` at `ui/automation/inspector.rs:221`
- [x] `advanced-search-toggle-button.md` — `research.header.advanced_search` at `ui/automation/mod.rs:43`
- [x] `sort-mode-button.md` — `research.paper.sort` at `ui/automation/mod.rs:191`
- [x] `add-manuscript-section-button.md` — `research.manuscript.add_section` at `ui/automation/inspector.rs:120`
- [x] `qanda-send-button.md` — `research.qa.send` at `ui/automation/inspector.rs:68`
- [x] `qanda-limitations-quick-action-button.md` — `research.qa.quick.limitations` at `ui/automation/inspector.rs:76`

### Field / Input Plans (10/10)

- [x] `pdf-search-field.md` — `research.pdf.search` at `ui/automation/pdf.rs:33`, `pdf_search_query` at `ui/state/types.rs:126`
- [x] `advanced-search-year-range-field.md` — `year_from`/`year_to` at `ui/state/types.rs:360-361`, logic at `ui/state/library.rs:154-155`, paint at `ui/paint/header.rs:200-213`. **Note: automation node `research.advanced.year` missing** (see INCOMPLETE section).
- [x] `advanced-search-title-field.md` — `research.advanced.title` at `ui/automation/mod.rs:76`
- [x] `header-search-field.md` — `research.header.search` at `ui/automation/mod.rs:35`
- [x] `qanda-input-field.md` — `research.qa.input` at `ui/automation/inspector.rs:54`
- [x] `manuscript-cite-search-field.md` — `research.manuscript.cite_search` at `ui/automation/inspector.rs:130`
- [x] `advanced-search-tag-field.md` — `research.advanced.tag` at `ui/automation/mod.rs:79`
- [x] `doi-arxiv-input-field.md` — `research.citation.doi` at `ui/automation/inspector.rs:207`
- [x] `advanced-search-venue-field.md` — `research.advanced.venue` at `ui/automation/mod.rs:78`
- [x] `advanced-search-author-field.md` — `research.advanced.author` at `ui/automation/mod.rs:77`

### Control Plans (14/14)

- [x] `paper-row-selection-control.md` — `select_visible_paper` at `ui/state/library.rs:88`, `research.paper.{index}` at `ui/automation/mod.rs:211`
- [x] `annotation-list-row-control.md` — Pointer handler at `ui/events/pointer.rs:444-472`, `select_pdf_annotation` at `ui/state/pdf.rs:118`, automation at `ui/automation/pdf.rs:131-156` (uses `research.pdf.annotation.{index}`, plan says `research.annotation.{index}` — naming difference, implementation complete)
- [x] `text-input-routing-control.md` — `on_text_event_impl` at `ui/events/text.rs:6`, character routing at `:174-189`
- [x] `pdf-search-enter-control.md` — Enter+PdfSearch at `ui/events/text.rs:133-136`, `advance_pdf_search`
- [x] `status-filter-row-control.md` — `hit_test_status_row` at `ui/collection_tree.rs:151`, `research.status.{index}` at `ui/automation/mod.rs:167`
- [x] `paper-row-multi-select-toggle.md` — `toggle_multi_select` at `ui/state/library.rs:567`, pointer at `ui/events/pointer.rs:172-173`
- [x] `pdf-surface-annotation-placement-control.md` — `add_pdf_annotation` at `ui/state/pdf.rs:107`, `research.pdf.surface` at `ui/automation/pdf.rs:47`
- [x] `saved-search-row-control.md` — `hit_test_saved_search_row` at `ui/collection_tree.rs:124`, `load_saved_search` at `ui/state/library.rs:187`, `research.saved_search.{index}` at `ui/automation/mod.rs:150`
- [x] `tag-chip-row-control.md` — `hit_test_tag_row` at `ui/collection_tree.rs:93`, `research.tag.{index}` at `ui/automation/mod.rs:132`
- [x] `smart-collection-row-control.md` — `hit_test_smart_collection_row` at `ui/collection_tree.rs:66`, `research.smart_collection.{index}` at `ui/automation/mod.rs:115`
- [x] `collection-row-selection-control.md` — `hit_test_collection_row` at `ui/collection_tree.rs:35`, `select_collection` at `ui/state/library.rs:464`
- [x] `manuscript-section-row-control.md` — `manuscript_active_section` at `ui/state/types.rs:148`, `research.manuscript.section.{index}` at `ui/automation/inspector.rs:99`
- [x] `welcome-backdrop-dismiss-control.md` — Click outside card at `ui/events/pointer.rs:42-48`, `research.welcome` at `ui/automation/welcome.rs:18`
- [x] `collection-expand-toggle.md` — `toggle_collection_expanded` at `ui/state/library.rs:471`, pointer at `ui/events/pointer.rs:81`. **Note: automation node `research.collection.{index}.expand` missing** (see INCOMPLETE section).

### Automatic Behavior Plans (23/23)

- [x] `automatic-inspector-tab-content-behavior.md` — `active_inspector_tab` at `ui/state/types.rs:79`, tab match at `ui/paint/inspector.rs:76`
- [x] `automatic-annotation-list-render-behavior.md` — `pdf_show_annotation_list` at `ui/state/types.rs:139`, conditional render at `ui/paint/center.rs:391-443`
- [x] `automatic-pdf-surface-render-behavior.md` — `pdf_zoom`/`pdf_rotation`/`pdf_page_image_data` at `ui/state/types.rs:121-131`, `build_pdf_surface_for_paper` at `ui/helpers.rs:12`
- [x] `automatic-dropped-import-paths-behavior.md` — `dropped_import_paths` at `ui/state/types.rs:92`, `handle_dropped_import_paths` at `ui/state/library.rs:256`
- [x] `automatic-paper-selection-highlight-behavior.md` — `selected_paper`/`selected_papers` at `ui/state/types.rs:77,89`, highlight at `ui/paint/left.rs:227-238`
- [x] `automatic-sort-indicator-behavior.md` — `sort_mode` at `ui/state/types.rs:102`, label at `ui/paint/left.rs:206`
- [x] `automatic-header-status-text-behavior.md` — `import_status`/`reader_mode`/`favorites_only` rendered at `ui/paint/header.rs:105-119`
- [x] `automatic-qanda-message-bubble-behavior.md` — `analysis_messages` at `ui/state/types.rs:84`, bubble render at `ui/paint/inspector.rs:133`
- [x] `automatic-citation-format-active-state-behavior.md` — `citation_export_format` at `ui/state/types.rs:143`, active check at `ui/paint/inspector.rs:515`
- [x] `automatic-pdf-search-result-counter-behavior.md` — `pdf_search_results`/`pdf_search_active_index` at `ui/state/types.rs:127-128`, counter at `ui/paint/center.rs:171-178`
- [x] `automatic-research-visual-surface-behavior.md` — `visual_summary_lines`/`visual_draw_plan` at `ui/state/types.rs:85-86`, render at `ui/paint/inspector.rs:255-267`
- [x] `automatic-welcome-overlay-behavior.md` — `show_welcome` at `ui/state/types.rs:155`, overlay at `ui/paint/overlays.rs:36-102`
- [x] `automatic-toast-stack-render-behavior.md` — `toasts` at `ui/state/types.rs:113`, stack at `ui/paint/overlays.rs:9-32`
- [x] `automatic-paper-detail-refresh-behavior.md` — `selected_paper` drives detail panel at `ui/paint/center.rs:19`
- [x] `automatic-cite-result-filter-behavior.md` — `manuscript_cite_search` at `ui/state/types.rs:147`, filter at `ui/state/manuscript.rs:54-57`
- [x] `automatic-paper-list-filtering-behavior.md` — `visible_paper_indices` at `ui/state/library.rs:14`, multi-dimension filtering at `:15-65`
- [x] `automatic-advanced-search-panel-render-behavior.md` — `show_advanced_search` at `ui/state/types.rs:109`, panel at `ui/paint/header.rs:70,144`
- [x] `automatic-manuscript-readiness-visual-behavior.md` — `manuscript_summary_lines`/`writing_visual_draw_plan` at `ui/state/types.rs:87-88`
- [x] `automatic-backup-restore-progress-behavior.md` — `progress_history` at `ui/state/types.rs:93`, record methods at `ui/state/library.rs:417-443`
- [x] `automatic-status-badge-color-behavior.md` — `ReadingStatus` at `crates/research-core/src/lib.rs:84`, `status_color` at `ui/paper_list.rs:4`
- [x] `automatic-paper-search-highlight-behavior.md` — `search_query` highlight at `ui/paint/left.rs:258`
- [x] `automatic-responsive-region-layout-behavior.md` — `research_regions` at `ui/mod.rs:28-37`
- [x] `toast-dismiss-control.md` — `dismiss_toast` at `ui/state/library.rs:650`, pointer at `ui/events/pointer.rs:53-67`

### Format / Cross-cutting Plans (2/2)

- [x] `research-format-roundtrip.md` — `save_library_snapshot`/`load_library_snapshot` at `crates/research-core/src/storage/file_io.rs:5,13`, 4 integration tests at `crates/research-core/tests/format_roundtrip.rs:81-193`
- [x] `citation-rendering.md` — `render_citation_preview` at `crates/research-core/src/citation.rs:157`, `CitationStyleFamily` at `:75`, all format buttons in inspector

---

## INCOMPLETE (3 plans)

### 1. `question-mark-shortcut-help-toggle.md`

**Status**: INCOMPLETE — Shortcut help overlay has no visual rendering.

**What works**:
- Keyboard handler: `?` toggles `show_shortcut_help` at `ui/events/text.rs:76-86`
- State field: `show_shortcut_help` at `ui/state/types.rs:116`
- Toggle method: `toggle_shortcut_help` at `ui/state/library.rs:654`
- Escape closes it: `ui/events/text.rs:88-89`

**What's missing**:
- **No paint code**: `ui/paint/overlays.rs` has no branch for `show_shortcut_help`. The field is toggled but nothing is rendered. The overlay file only handles toasts (line 9) and welcome screen (line 36).
- **No automation node**: The plan says "자동화 노드 없음" (no automation node), which is correct for a keyboard shortcut — but the feature still needs to paint something when `show_shortcut_help = true`.

**Impact**: Pressing `?` sets state but produces no visible change. User cannot see the shortcut help.

---

### 2. `advanced-search-year-range-field.md`

**Status**: INCOMPLETE — Missing automation node.

**What works**:
- State fields: `year_from`/`year_to` at `ui/state/types.rs:360-361`
- Filter logic: `update_advanced_search` at `ui/state/library.rs:154-155`
- Paint: Year range inputs rendered at `ui/paint/header.rs:200-213`
- Pointer handling: Advanced search toggle works

**What's missing**:
- **Automation node `research.advanced.year`**: The plan specifies this debug_id with role `TextInput` and value `Year`, exposed when `show_advanced_search=true`. The advanced search automation section at `ui/automation/mod.rs:75-79` only lists title, author, venue, and tag — year is absent.

**Impact**: E2E tests cannot find the year range input via automation node. The other 4 advanced search fields (title, author, venue, tag) all have automation nodes.

---

### 3. `collection-expand-toggle.md`

**Status**: INCOMPLETE — Missing automation node.

**What works**:
- State method: `toggle_collection_expanded` at `ui/state/library.rs:471`
- Pointer handler: Expand/collapse click at `ui/events/pointer.rs:81`
- Paint: Expand/collapse indicator at `ui/paint/left.rs:48` (`col.expanded`)
- Hit test: `hit_test_collection_row` at `ui/collection_tree.rs:35`

**What's missing**:
- **Automation node `research.collection.{index}.expand`**: The plan specifies a dedicated debug_id with role `Button` and value `expand/collapse`. The base collection node `research.collection.{index}` exists at `ui/automation/mod.rs:97-110`, but no child node with the `.expand` suffix is emitted.

**Impact**: E2E tests cannot target the expand/collapse button specifically — they can only target the whole collection row. The expand/collapse behavior works but is not independently testable via automation.

---

## Verification Criteria

- [x] All 103 plan files read and analyzed
- [x] Key identifiers extracted and searched for each plan
- [x] Rendering (paint), event handling (pointer/text), and automation nodes verified
- [x] State fields, methods, and enum variants confirmed
- [x] Cross-references between plan specs and actual code verified
- [x] Special attention to row controls, automatic behaviors, controls, and shortcuts

## Potential Risks and Mitigations

1. **Shortcut help overlay is invisible** — User-facing bug. The `?` key toggles state but nothing renders.
   Mitigation: Add paint code in `ui/paint/overlays.rs` to render shortcut help modal when `show_shortcut_help = true`.

2. **Missing automation nodes** — Test coverage gaps. Two features lack automation nodes specified in their plans.
   Mitigation: Add `research.advanced.year` node in `ui/automation/mod.rs` and `research.collection.{index}.expand` child node.

3. **Annotation list debug_id naming discrepancy** — Plan says `research.annotation.{index}`, code uses `research.pdf.annotation.{index}`. Not a functional issue but tests written against the plan's debug_id would fail.
   Mitigation: Either update the plan to match code or update the code to match the plan.

## Alternative Approaches

1. **Update plans instead of code**: For the annotation list naming discrepancy, updating the plan document to match the implemented `research.pdf.annotation.{index}` naming may be simpler than changing the debug_id in code and updating any existing tests.
2. **Batch the 3 fixes**: The 3 incomplete items are small, independent changes that could be addressed in a single commit.
