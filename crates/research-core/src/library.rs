use serde::{Deserialize, Serialize};

use crate::{
    AnnotationId, Attachment, AttachmentId, PdfAnnotation, ReferenceId, ReferenceItem,
    ResearchCollection, ResearchCollectionId, ResearchNote, ResearchNoteId, ResearchSnapshotV2,
    ResearchTag, ResearchTagId, Timestamp,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchMutationReport {
    pub changed: bool,
    pub message: String,
}

pub fn upsert_research_reference(
    mut snapshot: ResearchSnapshotV2,
    mut reference: ReferenceItem,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    reference.validate()?;
    reference.updated_at = now.clone();
    let message = if let Some(existing) = snapshot
        .references
        .iter_mut()
        .find(|existing| existing.id == reference.id)
    {
        reference.created_at = existing.created_at.clone();
        *existing = reference;
        "reference updated"
    } else {
        reference.created_at = now.clone();
        snapshot.references.push(reference);
        "reference added"
    };
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed(message)))
}

pub fn remove_research_reference(
    mut snapshot: ResearchSnapshotV2,
    reference_id: &ReferenceId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    let before = snapshot.references.len();
    snapshot
        .references
        .retain(|reference| &reference.id != reference_id);
    if before == snapshot.references.len() {
        return Err(format!(
            "reference {} does not exist",
            reference_id.as_str()
        ));
    }
    let removed_attachment_ids = snapshot
        .attachments
        .iter()
        .filter(|attachment| &attachment.reference_id == reference_id)
        .map(|attachment| attachment.id.clone())
        .collect::<Vec<_>>();
    snapshot
        .attachments
        .retain(|attachment| &attachment.reference_id != reference_id);
    snapshot.annotations.retain(|annotation| {
        &annotation.reference_id != reference_id
            && !removed_attachment_ids.contains(&annotation.attachment_id)
    });
    snapshot
        .notes
        .retain(|note| note.reference_id.as_ref() != Some(reference_id));
    touch_library(&mut snapshot, now);
    Ok((
        snapshot,
        changed("reference removed with dependent records"),
    ))
}

pub fn upsert_research_collection(
    mut snapshot: ResearchSnapshotV2,
    collection: ResearchCollection,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    if collection.name.trim().is_empty() {
        return Err("collection name is required".to_string());
    }
    let message = if let Some(existing) = snapshot
        .collections
        .iter_mut()
        .find(|existing| existing.id == collection.id)
    {
        *existing = collection;
        "collection updated"
    } else {
        snapshot.collections.push(collection);
        "collection added"
    };
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed(message)))
}

pub fn remove_research_collection(
    mut snapshot: ResearchSnapshotV2,
    collection_id: &ResearchCollectionId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    let before = snapshot.collections.len();
    snapshot
        .collections
        .retain(|collection| &collection.id != collection_id);
    if before == snapshot.collections.len() {
        return Err(format!(
            "collection {} does not exist",
            collection_id.as_str()
        ));
    }
    for reference in &mut snapshot.references {
        reference
            .collections
            .retain(|existing_id| existing_id != collection_id);
    }
    touch_library(&mut snapshot, now);
    Ok((
        snapshot,
        changed("collection removed from library and references"),
    ))
}

pub fn upsert_research_tag(
    mut snapshot: ResearchSnapshotV2,
    tag: ResearchTag,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    if tag.label.trim().is_empty() {
        return Err("tag label is required".to_string());
    }
    let message = if let Some(existing) = snapshot
        .tags
        .iter_mut()
        .find(|existing| existing.id == tag.id)
    {
        *existing = tag;
        "tag updated"
    } else {
        snapshot.tags.push(tag);
        "tag added"
    };
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed(message)))
}

pub fn remove_research_tag(
    mut snapshot: ResearchSnapshotV2,
    tag_id: &ResearchTagId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    let before = snapshot.tags.len();
    snapshot.tags.retain(|tag| &tag.id != tag_id);
    if before == snapshot.tags.len() {
        return Err(format!("tag {} does not exist", tag_id.as_str()));
    }
    for reference in &mut snapshot.references {
        reference.tags.retain(|existing_id| existing_id != tag_id);
    }
    for note in &mut snapshot.notes {
        note.tags.retain(|existing_id| existing_id != tag_id);
    }
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed("tag removed from references and notes")))
}

pub fn upsert_research_attachment(
    mut snapshot: ResearchSnapshotV2,
    mut attachment: Attachment,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    ensure_reference_exists(&snapshot, &attachment.reference_id)?;
    if attachment.content_hash.trim().is_empty() {
        return Err("attachment content hash is required".to_string());
    }
    attachment.updated_at = now.clone();
    let message = if let Some(existing) = snapshot
        .attachments
        .iter_mut()
        .find(|existing| existing.id == attachment.id)
    {
        attachment.created_at = existing.created_at.clone();
        *existing = attachment;
        "attachment updated"
    } else {
        attachment.created_at = now.clone();
        snapshot.attachments.push(attachment);
        "attachment added"
    };
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed(message)))
}

pub fn upsert_research_annotation(
    mut snapshot: ResearchSnapshotV2,
    mut annotation: PdfAnnotation,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    ensure_reference_exists(&snapshot, &annotation.reference_id)?;
    ensure_attachment_exists(&snapshot, &annotation.attachment_id)?;
    annotation.updated_at = now.clone();
    let message = if let Some(existing) = snapshot
        .annotations
        .iter_mut()
        .find(|existing| existing.id == annotation.id)
    {
        annotation.created_at = existing.created_at.clone();
        *existing = annotation;
        "annotation updated"
    } else {
        annotation.created_at = now.clone();
        snapshot.annotations.push(annotation);
        "annotation added"
    };
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed(message)))
}

pub fn upsert_research_note(
    mut snapshot: ResearchSnapshotV2,
    mut note: ResearchNote,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    if let Some(reference_id) = &note.reference_id {
        ensure_reference_exists(&snapshot, reference_id)?;
    }
    if let Some(annotation_id) = &note.annotation_id {
        ensure_annotation_exists(&snapshot, annotation_id)?;
    }
    if note.title.trim().is_empty() {
        return Err("note title is required".to_string());
    }
    note.updated_at = now.clone();
    let message = if let Some(existing) = snapshot
        .notes
        .iter_mut()
        .find(|existing| existing.id == note.id)
    {
        note.created_at = existing.created_at.clone();
        *existing = note;
        "note updated"
    } else {
        note.created_at = now.clone();
        snapshot.notes.push(note);
        "note added"
    };
    touch_library(&mut snapshot, now);
    Ok((snapshot, changed(message)))
}

pub fn convert_annotation_to_note(
    snapshot: ResearchSnapshotV2,
    note_id: ResearchNoteId,
    annotation_id: &AnnotationId,
    title: impl Into<String>,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    let annotation = snapshot
        .annotations
        .iter()
        .find(|annotation| &annotation.id == annotation_id)
        .cloned()
        .ok_or_else(|| format!("annotation {} does not exist", annotation_id.as_str()))?;
    let mut body = String::new();
    if let Some(selected_text) = &annotation.selected_text {
        body.push_str("> ");
        body.push_str(&selected_text.replace('\n', " "));
        body.push_str("\n\n");
    }
    if let Some(note_markdown) = &annotation.note_markdown {
        body.push_str(note_markdown.trim());
    }
    if body.trim().is_empty() {
        body.push_str("Annotation note");
    }
    upsert_research_note(
        snapshot,
        ResearchNote {
            id: note_id,
            reference_id: Some(annotation.reference_id),
            annotation_id: Some(annotation.id),
            title: title.into(),
            body_markdown: body,
            tags: Vec::new(),
            backlinks: Vec::new(),
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        now,
    )
}

pub fn merge_research_references(
    mut snapshot: ResearchSnapshotV2,
    target_id: &ReferenceId,
    source_id: &ReferenceId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    if target_id == source_id {
        return Err("cannot merge a reference into itself".to_string());
    }
    let target_index = snapshot
        .references
        .iter()
        .position(|reference| &reference.id == target_id)
        .ok_or_else(|| format!("target reference {} does not exist", target_id.as_str()))?;
    let source_index = snapshot
        .references
        .iter()
        .position(|reference| &reference.id == source_id)
        .ok_or_else(|| format!("source reference {} does not exist", source_id.as_str()))?;

    let source = snapshot.references[source_index].clone();
    {
        let target = &mut snapshot.references[target_index];
        merge_unique(&mut target.collections, source.collections);
        merge_unique(&mut target.tags, source.tags);
        merge_unique_by(&mut target.urls, source.urls, |url| url.url.clone());
        if target.abstract_text.is_none() {
            target.abstract_text = source.abstract_text;
        }
        if target.venue.is_none() {
            target.venue = source.venue;
        }
        if target.citekey.is_none() {
            target.citekey = source.citekey;
        }
        target.favorite |= source.favorite;
        target.rating = target.rating.max(source.rating);
        target.updated_at = now.clone();
    }

    snapshot.references.remove(source_index);
    for attachment in &mut snapshot.attachments {
        if &attachment.reference_id == source_id {
            attachment.reference_id = target_id.clone();
            attachment.updated_at = now.clone();
        }
    }
    for annotation in &mut snapshot.annotations {
        if &annotation.reference_id == source_id {
            annotation.reference_id = target_id.clone();
            annotation.updated_at = now.clone();
        }
    }
    for note in &mut snapshot.notes {
        if note.reference_id.as_ref() == Some(source_id) {
            note.reference_id = Some(target_id.clone());
            note.updated_at = now.clone();
        }
    }

    touch_library(&mut snapshot, now);
    Ok((
        snapshot,
        changed("references merged without dropping dependent records"),
    ))
}

fn ensure_reference_exists(
    snapshot: &ResearchSnapshotV2,
    reference_id: &ReferenceId,
) -> Result<(), String> {
    snapshot
        .references
        .iter()
        .any(|reference| &reference.id == reference_id)
        .then_some(())
        .ok_or_else(|| format!("reference {} does not exist", reference_id.as_str()))
}

fn ensure_attachment_exists(
    snapshot: &ResearchSnapshotV2,
    attachment_id: &AttachmentId,
) -> Result<(), String> {
    snapshot
        .attachments
        .iter()
        .any(|attachment| &attachment.id == attachment_id)
        .then_some(())
        .ok_or_else(|| format!("attachment {} does not exist", attachment_id.as_str()))
}

fn ensure_annotation_exists(
    snapshot: &ResearchSnapshotV2,
    annotation_id: &AnnotationId,
) -> Result<(), String> {
    snapshot
        .annotations
        .iter()
        .any(|annotation| &annotation.id == annotation_id)
        .then_some(())
        .ok_or_else(|| format!("annotation {} does not exist", annotation_id.as_str()))
}

fn touch_library(snapshot: &mut ResearchSnapshotV2, now: Timestamp) {
    snapshot.library.updated_at = now;
}

fn changed(message: impl Into<String>) -> ResearchMutationReport {
    ResearchMutationReport {
        changed: true,
        message: message.into(),
    }
}

fn merge_unique<T: Eq>(target: &mut Vec<T>, source: Vec<T>) {
    for item in source {
        if !target.contains(&item) {
            target.push(item);
        }
    }
}

fn merge_unique_by<T, K: Eq>(target: &mut Vec<T>, source: Vec<T>, key: impl Fn(&T) -> K) {
    for item in source {
        let source_key = key(&item);
        if !target.iter().any(|existing| key(existing) == source_key) {
            target.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AttachmentKind, ColorRgba, DateParts, LibraryId, LocalizedField, PageRect,
        PdfAnnotationKind, ReadingStatus, ReferenceIdentifiers, ReferenceKind, ReferenceMetadata,
        ResearchLocale,
    };

    fn now(value: &str) -> Timestamp {
        Timestamp(value.to_string())
    }

    fn snapshot() -> ResearchSnapshotV2 {
        crate::new_research_library_snapshot(
            LibraryId::from("lib"),
            "Library",
            "/tmp/lib",
            ResearchLocale::parse("en-US").unwrap(),
            now("2026-05-04T00:00:00Z"),
        )
    }

    fn reference(id: &str, title: &str) -> ReferenceItem {
        ReferenceItem {
            id: ReferenceId::from(id),
            kind: ReferenceKind::JournalArticle,
            title: LocalizedField::plain(title),
            subtitle: None,
            creators: Vec::new(),
            issued: DateParts {
                year: Some(2026),
                month: None,
                day: None,
                raw: None,
            },
            abstract_text: None,
            language: None,
            venue: None,
            identifiers: ReferenceIdentifiers::default(),
            urls: Vec::new(),
            collections: Vec::new(),
            tags: Vec::new(),
            status: ReadingStatus::Unread,
            favorite: false,
            rating: None,
            citekey: None,
            citekey_locked: false,
            metadata: ReferenceMetadata::default(),
            created_at: now("2026-05-04T00:00:00Z"),
            updated_at: now("2026-05-04T00:00:00Z"),
        }
    }

    #[test]
    fn reference_upsert_and_remove_keeps_dependents_consistent() {
        let (snapshot, _) = upsert_research_reference(
            snapshot(),
            reference("ref_1", "Paper"),
            now("2026-05-04T00:01:00Z"),
        )
        .expect("add reference");
        let (snapshot, _) = upsert_research_attachment(
            snapshot,
            Attachment {
                id: AttachmentId::from("att_1"),
                reference_id: ReferenceId::from("ref_1"),
                kind: AttachmentKind::Pdf,
                title: "PDF".to_string(),
                stored_path: "attachments/paper.pdf".to_string(),
                original_path: None,
                mime_type: "application/pdf".to_string(),
                size_bytes: 10,
                content_hash: "hash".to_string(),
                page_count: Some(1),
                text_indexed: false,
                created_at: now("2026-05-04T00:00:00Z"),
                updated_at: now("2026-05-04T00:00:00Z"),
            },
            now("2026-05-04T00:02:00Z"),
        )
        .expect("add attachment");
        let (snapshot, _) = upsert_research_annotation(
            snapshot,
            PdfAnnotation {
                id: AnnotationId::from("ann_1"),
                attachment_id: AttachmentId::from("att_1"),
                reference_id: ReferenceId::from("ref_1"),
                kind: PdfAnnotationKind::Highlight,
                page: 1,
                rects: vec![PageRect {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                }],
                color: ColorRgba {
                    r: 255,
                    g: 220,
                    b: 0,
                    a: 255,
                },
                selected_text: Some("claim".to_string()),
                note_markdown: None,
                created_at: now("2026-05-04T00:00:00Z"),
                updated_at: now("2026-05-04T00:00:00Z"),
            },
            now("2026-05-04T00:03:00Z"),
        )
        .expect("add annotation");
        let (snapshot, _) = remove_research_reference(
            snapshot,
            &ReferenceId::from("ref_1"),
            now("2026-05-04T00:04:00Z"),
        )
        .expect("remove reference");

        assert!(snapshot.references.is_empty());
        assert!(snapshot.attachments.is_empty());
        assert!(snapshot.annotations.is_empty());
    }

    #[test]
    fn annotation_converts_to_note() {
        let (snapshot, _) = upsert_research_reference(
            snapshot(),
            reference("ref_1", "Paper"),
            now("2026-05-04T00:01:00Z"),
        )
        .expect("add reference");
        let mut snapshot = snapshot;
        snapshot.attachments.push(Attachment {
            id: AttachmentId::from("att_1"),
            reference_id: ReferenceId::from("ref_1"),
            kind: AttachmentKind::Pdf,
            title: "PDF".to_string(),
            stored_path: "attachments/paper.pdf".to_string(),
            original_path: None,
            mime_type: "application/pdf".to_string(),
            size_bytes: 10,
            content_hash: "hash".to_string(),
            page_count: Some(1),
            text_indexed: false,
            created_at: now("2026-05-04T00:00:00Z"),
            updated_at: now("2026-05-04T00:00:00Z"),
        });
        snapshot.annotations.push(PdfAnnotation {
            id: AnnotationId::from("ann_1"),
            attachment_id: AttachmentId::from("att_1"),
            reference_id: ReferenceId::from("ref_1"),
            kind: PdfAnnotationKind::Highlight,
            page: 1,
            rects: Vec::new(),
            color: ColorRgba {
                r: 255,
                g: 220,
                b: 0,
                a: 255,
            },
            selected_text: Some("important claim".to_string()),
            note_markdown: Some("Follow up.".to_string()),
            created_at: now("2026-05-04T00:00:00Z"),
            updated_at: now("2026-05-04T00:00:00Z"),
        });

        let (snapshot, _) = convert_annotation_to_note(
            snapshot,
            ResearchNoteId::from("note_1"),
            &AnnotationId::from("ann_1"),
            "Annotation note",
            now("2026-05-04T00:02:00Z"),
        )
        .expect("convert");

        assert_eq!(snapshot.notes.len(), 1);
        assert!(snapshot.notes[0].body_markdown.contains("important claim"));
    }

    #[test]
    fn merge_preserves_notes_attachments_and_tags() {
        let (snapshot, _) = upsert_research_reference(
            snapshot(),
            reference("target", "Target"),
            now("2026-05-04T00:01:00Z"),
        )
        .expect("add target");
        let mut source = reference("source", "Source");
        source.tags.push(ResearchTagId::from("tag_1"));
        source.favorite = true;
        let (mut snapshot, _) =
            upsert_research_reference(snapshot, source, now("2026-05-04T00:02:00Z"))
                .expect("add source");
        snapshot.notes.push(ResearchNote {
            id: ResearchNoteId::from("note_1"),
            reference_id: Some(ReferenceId::from("source")),
            annotation_id: None,
            title: "Note".to_string(),
            body_markdown: "Body".to_string(),
            tags: Vec::new(),
            backlinks: Vec::new(),
            created_at: now("2026-05-04T00:00:00Z"),
            updated_at: now("2026-05-04T00:00:00Z"),
        });

        let (snapshot, _) = merge_research_references(
            snapshot,
            &ReferenceId::from("target"),
            &ReferenceId::from("source"),
            now("2026-05-04T00:03:00Z"),
        )
        .expect("merge");

        assert_eq!(snapshot.references.len(), 1);
        assert!(snapshot.references[0].favorite);
        assert!(snapshot.references[0]
            .tags
            .contains(&ResearchTagId::from("tag_1")));
        assert_eq!(
            snapshot.notes[0].reference_id.as_ref(),
            Some(&ReferenceId::from("target"))
        );
    }
}
