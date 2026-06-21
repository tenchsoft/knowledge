use super::builders::{all_research_ui_commands, context_commands_for_target};
use super::*;
use tench_research_core::ReadingStatus;

impl ResearchState {
    pub fn selected_paper(&self) -> Option<&Paper> {
        self.papers.get(self.selected_paper)
    }

    pub fn selected_paper_mut(&mut self) -> Option<&mut Paper> {
        self.papers.get_mut(self.selected_paper)
    }

    pub fn visible_paper_indices(&self) -> Vec<usize> {
        let query = self.search_query.trim().to_lowercase();
        self.papers
            .iter()
            .enumerate()
            .filter(|(_, paper)| !self.favorites_only || paper.favorite)
            .filter(|(_, paper)| {
                if let Some(collection_id) = &self.selected_collection {
                    if collection_id != "all-papers" {
                        return paper.collection_ids.iter().any(|id| id == collection_id);
                    }
                }
                true
            })
            .filter(|(_, paper)| {
                query.is_empty()
                    || paper.title.to_lowercase().contains(&query)
                    || paper.authors.to_lowercase().contains(&query)
                    || paper.venue.to_lowercase().contains(&query)
                    || status_label(&paper.status).to_lowercase().contains(&query)
                    || paper
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query))
            })
            .filter(|(_, paper)| {
                if let Some(adv) = &self.advanced_search {
                    let mut pass = true;
                    if !adv.title_query.is_empty() {
                        pass &= paper
                            .title
                            .to_lowercase()
                            .contains(&adv.title_query.to_lowercase());
                    }
                    if !adv.author_query.is_empty() {
                        pass &= paper
                            .authors
                            .to_lowercase()
                            .contains(&adv.author_query.to_lowercase());
                    }
                    if !adv.venue_query.is_empty() {
                        pass &= paper
                            .venue
                            .to_lowercase()
                            .contains(&adv.venue_query.to_lowercase());
                    }
                    if !adv.tag_query.is_empty() {
                        pass &= paper
                            .tags
                            .iter()
                            .any(|t| t.to_lowercase().contains(&adv.tag_query.to_lowercase()));
                    }
                    if let Some(from) = adv.year_from {
                        pass &= paper.year >= from;
                    }
                    if let Some(to) = adv.year_to {
                        pass &= paper.year <= to;
                    }
                    pass
                } else {
                    true
                }
            })
            .map(|(index, _)| index)
            .collect()
    }

    pub fn visible_papers(&self) -> Vec<(usize, &Paper)> {
        self.visible_paper_indices()
            .into_iter()
            .filter_map(|index| self.papers.get(index).map(|paper| (index, paper)))
            .collect()
    }

    pub fn select_visible_paper(&mut self, row_index: usize) -> bool {
        let Some(paper_index) = self.visible_paper_indices().get(row_index).copied() else {
            return false;
        };
        self.selected_paper = paper_index;
        self.selected_papers.clear();
        self.selected_papers.insert(paper_index);
        true
    }

    pub fn move_selection(&mut self, delta: isize) -> bool {
        let visible = self.visible_paper_indices();
        if visible.is_empty() {
            return false;
        }

        let current_row = visible
            .iter()
            .position(|index| *index == self.selected_paper)
            .unwrap_or(0);
        let next_row = if delta.is_negative() {
            current_row.saturating_sub(delta.unsigned_abs())
        } else {
            (current_row + delta as usize).min(visible.len() - 1)
        };
        self.selected_paper = visible[next_row];
        self.selected_papers.clear();
        self.selected_papers.insert(self.selected_paper);
        true
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn push_search_text(&mut self, text: &str) {
        self.search_query.push_str(text);
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn pop_search_text(&mut self) {
        self.search_query.pop();
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn toggle_advanced_search(&mut self) {
        self.show_advanced_search = !self.show_advanced_search;
        if self.show_advanced_search && self.advanced_search.is_none() {
            self.advanced_search = Some(AdvancedSearchState::default());
        }
    }

    pub fn update_advanced_search(&mut self, field: &str, value: &str) {
        if let Some(ref mut adv) = self.advanced_search {
            match field {
                "title" => adv.title_query = value.to_string(),
                "author" => adv.author_query = value.to_string(),
                "venue" => adv.venue_query = value.to_string(),
                "tag" => adv.tag_query = value.to_string(),
                "year_from" => adv.year_from = value.parse().ok(),
                "year_to" => adv.year_to = value.parse().ok(),
                _ => {}
            }
        }
    }

    pub fn apply_advanced_search(&mut self) {
        // The visible_paper_indices already handles advanced_search filtering
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn clear_advanced_search(&mut self) {
        self.advanced_search = None;
        self.show_advanced_search = false;
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn save_current_search(&mut self, name: String) {
        let id = format!("search-{}", self.saved_searches.len() + 1);
        self.saved_searches.push(SavedSearch {
            id,
            name,
            query: self.search_query.clone(),
            advanced: self.advanced_search.clone(),
        });
        self.add_toast("Search saved", ToastKind::Success);
    }

    pub fn load_saved_search(&mut self, search_id: &str) {
        if let Some(saved) = self.saved_searches.iter().find(|s| s.id == search_id) {
            self.search_query = saved.query.clone();
            self.advanced_search = saved.advanced.clone();
            if !self.visible_paper_indices().contains(&self.selected_paper) {
                let _ = self.select_visible_paper(0);
            }
        }
    }

    pub fn toggle_favorites_only(&mut self) {
        self.favorites_only = !self.favorites_only;
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn toggle_selected_favorite(&mut self) -> bool {
        let Some(paper) = self.selected_paper_mut() else {
            return false;
        };
        paper.favorite = !paper.favorite;
        true
    }

    pub fn append_selected_note(&mut self, note: &str) -> bool {
        let note = note.trim();
        if note.is_empty() {
            return false;
        }
        let Some(paper) = self.selected_paper_mut() else {
            return false;
        };
        if !paper.notes.is_empty() {
            paper.notes.push('\n');
        }
        paper.notes.push_str(note);
        true
    }

    pub fn cycle_citation_format(&mut self) {
        self.citation_format = match self.citation_format {
            CitationFormat::BibTex => CitationFormat::Apa,
            CitationFormat::Apa => CitationFormat::Mla,
            CitationFormat::Mla => CitationFormat::BibTex,
        };
    }

    pub fn queue_import(&mut self) {
        self.reader_mode = ReaderMode::Importing;
        self.import_status = ImportStatus::Queued;
        self.progress_history.push(ResearchProgressEvent {
            id: format!("import-{}", self.progress_history.len() + 1),
            kind: ResearchProgressKind::Import,
            label: "import_queued".to_string(),
            status: ResearchProgressStatus::Queued,
            completed: 0,
            total: 1,
        });
    }

    pub fn toggle_reader_mode(&mut self) {
        self.reader_mode = if self.reader_mode == ReaderMode::Pdf {
            ReaderMode::Detail
        } else {
            ReaderMode::Pdf
        };
    }

    pub fn handle_dropped_import_paths<I>(&mut self, paths: I) -> usize
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let mut added = 0usize;
        for path in paths {
            let path = path.into();
            if path.trim().is_empty() || self.dropped_import_paths.contains(&path) {
                continue;
            }
            self.dropped_import_paths.push(path);
            added += 1;
        }
        if added > 0 {
            self.reader_mode = ReaderMode::Importing;
            self.import_status = ImportStatus::Queued;
            self.progress_history.push(ResearchProgressEvent {
                id: format!("drop-import-{}", self.progress_history.len() + 1),
                kind: ResearchProgressKind::Import,
                label: format!("dropped_import_paths:{added}"),
                status: ResearchProgressStatus::Queued,
                completed: 0,
                total: added.try_into().unwrap_or(u32::MAX),
            });
        }
        added
    }

    pub fn open_context_menu(&mut self, target: ResearchContextTarget) {
        let commands = context_commands_for_target(&target);
        self.context_menu = Some(ResearchContextMenu { target, commands });
    }

    pub fn close_context_menu(&mut self) {
        self.context_menu = None;
    }

    pub fn open_command_palette(&mut self) {
        self.command_palette.open = true;
        self.command_palette.commands = all_research_ui_commands();
    }

    pub fn update_command_palette_query(&mut self, query: impl Into<String>) {
        self.command_palette.query = query.into();
        let normalized = self.command_palette.query.trim().to_ascii_lowercase();
        self.command_palette.commands = all_research_ui_commands()
            .into_iter()
            .filter(|command| {
                normalized.is_empty() || command.label().contains(normalized.as_str())
            })
            .collect();
    }

    pub fn close_command_palette(&mut self) {
        self.command_palette.open = false;
    }

    pub fn toggle_visible_paper_selection(&mut self, row_index: usize) -> bool {
        let Some(paper_index) = self.visible_paper_indices().get(row_index).copied() else {
            return false;
        };
        if !self.selected_papers.insert(paper_index) {
            self.selected_papers.remove(&paper_index);
        }
        if self.selected_papers.is_empty() || self.selected_papers.contains(&paper_index) {
            self.selected_paper = paper_index;
        }
        true
    }

    pub fn select_visible_paper_range(&mut self, start_row: usize, end_row: usize) -> usize {
        let visible = self.visible_paper_indices();
        if visible.is_empty() {
            return 0;
        }
        let start = start_row.min(end_row).min(visible.len() - 1);
        let end = start_row.max(end_row).min(visible.len() - 1);
        self.selected_papers.clear();
        for index in &visible[start..=end] {
            self.selected_papers.insert(*index);
        }
        if let Some(last) = visible.get(end).copied() {
            self.selected_paper = last;
        }
        self.selected_papers.len()
    }

    pub fn batch_set_selected_status(&mut self, status: ReadingStatus) -> usize {
        let targets = self.batch_target_indices();
        for index in &targets {
            if let Some(paper) = self.papers.get_mut(*index) {
                paper.status = status.clone();
            }
        }
        targets.len()
    }

    pub fn batch_add_selected_tag(&mut self, tag: impl Into<String>) -> usize {
        let tag = tag.into();
        let tag = tag.trim();
        if tag.is_empty() {
            return 0;
        }
        let targets = self.batch_target_indices();
        for index in &targets {
            if let Some(paper) = self.papers.get_mut(*index) {
                if !paper.tags.iter().any(|existing| existing == tag) {
                    paper.tags.push(tag.to_string());
                }
            }
        }
        targets.len()
    }

    pub fn record_duplicate_candidates(&mut self, candidate_count: usize) {
        self.duplicate_merge = Some(ResearchDuplicateMergeUi {
            candidate_count,
            selected_pair: None,
            last_action: Some("scan_complete".to_string()),
        });
    }

    pub fn select_duplicate_merge_pair(
        &mut self,
        left_id: impl Into<String>,
        right_id: impl Into<String>,
    ) {
        let merge = self
            .duplicate_merge
            .get_or_insert(ResearchDuplicateMergeUi {
                candidate_count: 0,
                selected_pair: None,
                last_action: None,
            });
        merge.selected_pair = Some((left_id.into(), right_id.into()));
        merge.last_action = Some("pair_selected".to_string());
    }

    pub fn record_missing_attachment_scan(
        &mut self,
        missing_count: usize,
        unresolved_count: usize,
    ) {
        self.missing_attachment_repair = ResearchAttachmentRepairUi {
            missing_count,
            unresolved_count,
            last_scan_label: Some(format!(
                "missing:{missing_count}/unresolved:{unresolved_count}"
            )),
        };
        self.progress_history.push(ResearchProgressEvent {
            id: format!("repair-scan-{}", self.progress_history.len() + 1),
            kind: ResearchProgressKind::Repair,
            label: "missing_attachment_scan".to_string(),
            status: ResearchProgressStatus::Complete,
            completed: missing_count.saturating_sub(unresolved_count) as u32,
            total: missing_count.try_into().unwrap_or(u32::MAX),
        });
    }

    pub fn record_library_backup(&mut self, path: impl Into<String>) {
        let path = path.into();
        self.backup_restore.last_backup_path = Some(path.clone());
        self.progress_history.push(ResearchProgressEvent {
            id: format!("backup-{}", self.progress_history.len() + 1),
            kind: ResearchProgressKind::Backup,
            label: path,
            status: ResearchProgressStatus::Complete,
            completed: 1,
            total: 1,
        });
    }

    pub fn record_library_restore(&mut self, label: impl Into<String>) {
        let label = label.into();
        self.backup_restore.last_restore_label = Some(label.clone());
        self.progress_history.push(ResearchProgressEvent {
            id: format!("restore-{}", self.progress_history.len() + 1),
            kind: ResearchProgressKind::Restore,
            label,
            status: ResearchProgressStatus::Complete,
            completed: 1,
            total: 1,
        });
    }

    pub fn record_export_progress(
        &mut self,
        label: impl Into<String>,
        status: ResearchProgressStatus,
        completed: u32,
        total: u32,
    ) {
        self.progress_history.push(ResearchProgressEvent {
            id: format!("export-{}", self.progress_history.len() + 1),
            kind: ResearchProgressKind::Export,
            label: label.into(),
            status,
            completed,
            total,
        });
    }

    pub fn record_i18n_coverage(&mut self, missing_keys: Vec<String>) {
        self.i18n_missing_keys = missing_keys;
    }

    pub fn select_collection(&mut self, collection_id: Option<String>) {
        self.selected_collection = collection_id;
        if !self.visible_paper_indices().contains(&self.selected_paper) {
            let _ = self.select_visible_paper(0);
        }
    }

    pub fn toggle_collection_expanded(&mut self, collection_id: &str) {
        if let Some(col) = self.collections.iter_mut().find(|c| c.id == collection_id) {
            col.expanded = !col.expanded;
        }
    }

    pub fn add_collection(&mut self, name: String) {
        let id = format!("collection-{}", self.collections.len());
        self.collections.push(Collection {
            id: id.clone(),
            name,
            count: 0,
            expanded: true,
            parent_id: None,
        });
    }

    pub fn rename_collection(&mut self, collection_id: &str, new_name: String) {
        if let Some(col) = self.collections.iter_mut().find(|c| c.id == collection_id) {
            col.name = new_name;
        }
    }

    pub fn delete_collection(&mut self, collection_id: &str) {
        self.collections.retain(|c| c.id != collection_id);
        if self.selected_collection.as_deref() == Some(collection_id) {
            self.selected_collection = None;
        }
    }

    pub fn set_focus(&mut self, focus: FocusTarget) {
        self.focus = focus;
    }

    pub fn set_sort_mode(&mut self, mode: SortMode) {
        self.sort_mode = mode;
    }

    pub fn push_qa_input(&mut self, text: &str) {
        self.qa_input.push_str(text);
    }

    pub fn pop_qa_input(&mut self) {
        self.qa_input.pop();
    }

    pub fn send_qa_message(&mut self) {
        let question = self.qa_input.trim().to_string();
        if question.is_empty() {
            return;
        }
        self.analysis_messages
            .push(("user".to_string(), question.clone()));
        self.qa_input.clear();
        // TODO: connect to analysis backend via Tauri command
        self.analysis_messages
            .push(("assistant".to_string(), format!("Processing: {question}")));
    }

    pub fn add_tag_to_selected(&mut self, tag: String) -> bool {
        let tag = tag.trim().to_string();
        if tag.is_empty() {
            return false;
        }
        let Some(paper) = self.selected_paper_mut() else {
            return false;
        };
        if !paper.tags.iter().any(|t| t == &tag) {
            paper.tags.push(tag);
            return true;
        }
        false
    }

    pub fn remove_tag_from_selected(&mut self, tag: &str) -> bool {
        let Some(paper) = self.selected_paper_mut() else {
            return false;
        };
        let before = paper.tags.len();
        paper.tags.retain(|t| t != tag);
        paper.tags.len() != before
    }

    pub fn cycle_sort_mode(&mut self) {
        self.sort_mode = match self.sort_mode {
            SortMode::TitleAsc => SortMode::TitleDesc,
            SortMode::TitleDesc => SortMode::YearDesc,
            SortMode::YearDesc => SortMode::YearAsc,
            SortMode::YearAsc => SortMode::AuthorAsc,
            SortMode::AuthorAsc => SortMode::AuthorDesc,
            SortMode::AuthorDesc => SortMode::DateAddedDesc,
            SortMode::DateAddedDesc => SortMode::DateAddedAsc,
            SortMode::DateAddedAsc => SortMode::TitleAsc,
        };
    }

    pub fn toggle_multi_select(&mut self, paper_index: usize) {
        if self.selected_papers.contains(&paper_index) {
            self.selected_papers.remove(&paper_index);
        } else {
            self.selected_papers.insert(paper_index);
        }
    }

    pub fn select_all_visible(&mut self) {
        self.selected_papers = self.visible_paper_indices().into_iter().collect();
    }

    pub fn clear_multi_select(&mut self) {
        self.selected_papers.clear();
        if let Some(&idx) = self.visible_paper_indices().first() {
            self.selected_paper = idx;
            self.selected_papers.insert(idx);
        }
    }

    pub fn batch_add_tag(&mut self, tag: String) {
        let count = self.selected_papers.len();
        for idx in &self.selected_papers {
            if let Some(paper) = self.papers.get_mut(*idx) {
                if !paper.tags.contains(&tag) {
                    paper.tags.push(tag.clone());
                }
            }
        }
        self.add_toast(
            format!("Added tag '{tag}' to {count} paper(s)"),
            ToastKind::Success,
        );
    }

    pub fn add_smart_collection(&mut self, name: String, rule: SmartCollectionRule) {
        let id = format!("smart-{}", self.smart_collections.len() + 1);
        let count = self.count_smart_collection(&rule);
        self.smart_collections.push(SmartCollection {
            id,
            name,
            rule,
            count,
        });
    }

    fn count_smart_collection(&self, rule: &SmartCollectionRule) -> usize {
        match rule {
            SmartCollectionRule::RecentlyAdded { .. } => {
                // Placeholder: count all papers (real impl would check dates)
                self.papers.len()
            }
            SmartCollectionRule::Tagged { tag } => {
                self.papers.iter().filter(|p| p.tags.contains(tag)).count()
            }
            SmartCollectionRule::Unread => self
                .papers
                .iter()
                .filter(|p| p.status == ReadingStatus::Unread)
                .count(),
            SmartCollectionRule::Favorites => self.papers.iter().filter(|p| p.favorite).count(),
            SmartCollectionRule::ByAuthor { author } => self
                .papers
                .iter()
                .filter(|p| p.authors.contains(author))
                .count(),
            SmartCollectionRule::YearRange { from, to } => self
                .papers
                .iter()
                .filter(|p| p.year >= *from && p.year <= *to)
                .count(),
        }
    }

    pub fn add_toast(&mut self, message: impl Into<String>, kind: ToastKind) {
        let id = format!("toast-{}", self.toasts.len());
        self.toasts.push(ToastMessage {
            id,
            message: message.into(),
            kind,
        });
    }

    pub fn dismiss_toast(&mut self, id: &str) {
        self.toasts.retain(|t| t.id != id);
    }

    pub fn toggle_shortcut_help(&mut self) {
        self.show_shortcut_help = !self.show_shortcut_help;
    }

    pub fn export_action(&mut self) {
        self.record_export_progress(
            "selected references",
            ResearchProgressStatus::Queued,
            0,
            self.papers.len() as u32,
        );
        self.add_toast("Export queued", ToastKind::Info);
    }

    pub fn sync_action(&mut self) {
        self.add_toast("Sync started", ToastKind::Info);
    }

    fn batch_target_indices(&self) -> Vec<usize> {
        if self.selected_papers.is_empty() {
            vec![self.selected_paper]
        } else {
            self.selected_papers.iter().copied().collect()
        }
    }
}
