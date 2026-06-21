use super::checks::run_non_ai_writing_checks;
use super::document::{
    document_contains_heading, ensure_section_heading, find_section_mut, mark_section_drafting,
    paragraph_block,
};
use super::*;
use crate::Timestamp;
use tench_document_core::{BlockNode, ImageSource, InlineNode, Marks};

pub fn insert_manuscript_asset_placement(
    mut manuscript: ResearchManuscript,
    placement: ManuscriptAssetPlacement,
) -> Result<ResearchManuscript, String> {
    let asset = manuscript
        .assets
        .iter()
        .find(|asset| asset.id == placement.asset_id)
        .cloned()
        .ok_or_else(|| format!("unknown manuscript asset {}", placement.asset_id.as_str()))?;
    let label = manuscript_cross_reference_label(
        &manuscript,
        &ManuscriptCrossReferenceTarget::Asset {
            asset_id: asset.id.clone(),
        },
    )?;
    let section_title = mark_section_drafting(
        &mut manuscript,
        &placement.section_id,
        placement.now.clone(),
    )?;
    ensure_section_heading(&mut manuscript.document, &section_title);
    match asset.kind {
        AssetKind::Figure | AssetKind::Supplement => {
            manuscript.document.content.push(BlockNode::Image {
                source: image_source_for_asset(&asset),
                alt: asset
                    .alt_text
                    .clone()
                    .or_else(|| Some(asset.caption.value.clone())),
                width: None,
                height: None,
            });
            manuscript
                .document
                .content
                .push(paragraph_block(format!("{label}. {}", asset.caption.value)));
        }
        AssetKind::Table => {
            manuscript
                .document
                .content
                .push(paragraph_block(format!("{label}. {}", asset.caption.value)));
        }
        AssetKind::Equation => {
            manuscript.document.content.push(BlockNode::CodeBlock {
                language: Some("latex-math".to_string()),
                code: asset.caption.value.clone(),
            });
            manuscript.document.content.push(paragraph_block(label));
        }
    }
    manuscript.document.metadata.updated_at = Some(placement.now.0);
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn create_manuscript_asset(
    mut manuscript: ResearchManuscript,
    mut asset: ManuscriptAsset,
    now: Timestamp,
) -> Result<ResearchManuscript, String> {
    if manuscript
        .assets
        .iter()
        .any(|existing| existing.id == asset.id)
    {
        return Err(format!(
            "duplicate manuscript asset id {}",
            asset.id.as_str()
        ));
    }
    validate_manuscript_asset(&asset)?;
    if asset.order == 0 {
        asset.order = next_asset_order(&manuscript, asset.kind);
    }
    if asset.label.trim().is_empty() {
        asset.label = fallback_asset_label(asset.kind, asset.order);
    }
    manuscript.assets.push(asset);
    sort_manuscript_assets(&mut manuscript.assets);
    manuscript.updated_at = now;
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn update_manuscript_asset(
    mut manuscript: ResearchManuscript,
    mut asset: ManuscriptAsset,
    now: Timestamp,
) -> Result<ResearchManuscript, String> {
    validate_manuscript_asset(&asset)?;
    let existing = manuscript
        .assets
        .iter_mut()
        .find(|existing| existing.id == asset.id)
        .ok_or_else(|| format!("unknown manuscript asset {}", asset.id.as_str()))?;
    if asset.order == 0 {
        asset.order = existing.order;
    }
    if asset.label.trim().is_empty() {
        asset.label = existing.label.clone();
    }
    *existing = asset;
    sort_manuscript_assets(&mut manuscript.assets);
    manuscript.updated_at = now;
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn build_manuscript_asset_numbering(
    manuscript: &ResearchManuscript,
) -> Vec<ManuscriptAssetNumbering> {
    let mut assets = manuscript.assets.clone();
    sort_manuscript_assets(&mut assets);
    let mut figure_count = 0;
    let mut table_count = 0;
    let mut equation_count = 0;
    let mut supplement_count = 0;

    assets
        .into_iter()
        .map(|asset| {
            let number = match asset.kind {
                AssetKind::Figure => {
                    figure_count += 1;
                    figure_count
                }
                AssetKind::Table => {
                    table_count += 1;
                    table_count
                }
                AssetKind::Equation => {
                    equation_count += 1;
                    equation_count
                }
                AssetKind::Supplement => {
                    supplement_count += 1;
                    supplement_count
                }
            };
            ManuscriptAssetNumbering {
                asset_id: asset.id,
                kind: asset.kind,
                number,
                label: format!("{} {number}", asset_kind_label(asset.kind)),
                caption: asset.caption,
                order: asset.order,
            }
        })
        .collect()
}

pub fn insert_manuscript_cross_reference(
    mut manuscript: ResearchManuscript,
    insertion: ManuscriptCrossReferenceInsertion,
) -> Result<ResearchManuscript, String> {
    if manuscript
        .cross_references
        .iter()
        .any(|existing| existing.id == insertion.id)
    {
        return Err(format!(
            "duplicate manuscript cross-reference id {}",
            insertion.id.as_str()
        ));
    }
    let label = manuscript_cross_reference_label(&manuscript, &insertion.target)?;
    let section = find_section_mut(&mut manuscript.outline.sections, &insertion.section_id)
        .ok_or_else(|| {
            format!(
                "unknown manuscript section {}",
                insertion.section_id.as_str()
            )
        })?;
    section.status = SectionStatus::NeedsRevision;
    let section_title = section.title.value.clone();

    if !document_contains_heading(&manuscript.document, &section_title) {
        manuscript.document.content.push(BlockNode::Heading {
            level: 2,
            content: vec![InlineNode::Text {
                text: section_title,
                marks: Marks::default(),
            }],
            attrs: Default::default(),
        });
    }
    manuscript.document.content.push(BlockNode::Paragraph {
        content: vec![InlineNode::Text {
            text: format!("See {label}."),
            marks: Marks::default(),
        }],
        attrs: Default::default(),
    });
    manuscript.cross_references.push(ManuscriptCrossReference {
        id: insertion.id,
        section_id: insertion.section_id,
        target: insertion.target,
        label,
        created_at: insertion.now.clone(),
    });
    manuscript.document.metadata.updated_at = Some(insertion.now.0.clone());
    manuscript.updated_at = insertion.now;
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

fn image_source_for_asset(asset: &ManuscriptAsset) -> ImageSource {
    if let Some(path) = asset
        .source
        .path
        .as_ref()
        .filter(|path| !path.trim().is_empty())
    {
        return ImageSource::Referenced { path: path.clone() };
    }
    if let Some(attachment_id) = &asset.source.attachment_id {
        return ImageSource::Referenced {
            path: format!("research://attachment/{}", attachment_id.as_str()),
        };
    }
    if let Some(note_id) = &asset.source.note_id {
        return ImageSource::Referenced {
            path: format!("research://note/{}", note_id.as_str()),
        };
    }
    ImageSource::Referenced {
        path: format!("research://asset/{}", asset.id.as_str()),
    }
}

fn validate_manuscript_asset(asset: &ManuscriptAsset) -> Result<(), String> {
    match asset.source.kind {
        AssetSourceKind::Attachment if asset.source.attachment_id.is_none() => {
            return Err(format!(
                "asset {} uses attachment source without attachment id",
                asset.id.as_str()
            ));
        }
        AssetSourceKind::LocalFile
            if asset
                .source
                .path
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty() =>
        {
            return Err(format!(
                "asset {} uses local file source without path",
                asset.id.as_str()
            ));
        }
        AssetSourceKind::Note if asset.source.note_id.is_none() => {
            return Err(format!(
                "asset {} uses note source without note id",
                asset.id.as_str()
            ));
        }
        AssetSourceKind::Generated
        | AssetSourceKind::Attachment
        | AssetSourceKind::LocalFile
        | AssetSourceKind::Note => {}
    }
    Ok(())
}

fn next_asset_order(manuscript: &ResearchManuscript, kind: AssetKind) -> u32 {
    manuscript
        .assets
        .iter()
        .filter(|asset| asset.kind == kind)
        .map(|asset| asset.order)
        .max()
        .unwrap_or(0)
        .saturating_add(1)
}

fn sort_manuscript_assets(assets: &mut [ManuscriptAsset]) {
    assets.sort_by(|left, right| {
        asset_kind_rank(left.kind)
            .cmp(&asset_kind_rank(right.kind))
            .then(left.order.cmp(&right.order))
            .then(left.id.as_str().cmp(right.id.as_str()))
    });
}

fn asset_kind_rank(kind: AssetKind) -> u8 {
    match kind {
        AssetKind::Figure => 0,
        AssetKind::Table => 1,
        AssetKind::Equation => 2,
        AssetKind::Supplement => 3,
    }
}

fn asset_kind_label(kind: AssetKind) -> &'static str {
    match kind {
        AssetKind::Figure => "Figure",
        AssetKind::Table => "Table",
        AssetKind::Equation => "Equation",
        AssetKind::Supplement => "Supplement",
    }
}

fn fallback_asset_label(kind: AssetKind, order: u32) -> String {
    format!("{} {order}", asset_kind_label(kind))
}

fn manuscript_cross_reference_label(
    manuscript: &ResearchManuscript,
    target: &ManuscriptCrossReferenceTarget,
) -> Result<String, String> {
    match target {
        ManuscriptCrossReferenceTarget::Asset { asset_id } => {
            build_manuscript_asset_numbering(manuscript)
                .into_iter()
                .find(|numbering| numbering.asset_id == *asset_id)
                .map(|numbering| numbering.label)
                .ok_or_else(|| format!("unknown manuscript asset {}", asset_id.as_str()))
        }
    }
}
