use super::*;
use crate::AttachmentId;

impl ResearchIndexDocument {
    pub(super) fn attachment_id(&self) -> Option<AttachmentId> {
        parse_pdf_document_id(&self.id)
            .map(|(attachment_id, _)| attachment_id)
            .or_else(|| attachment_id_from_location(self.location.as_deref()))
    }

    pub(super) fn page(&self) -> Option<u32> {
        parse_pdf_document_id(&self.id)
            .map(|(_, page)| page)
            .or_else(|| page_from_location(self.location.as_deref()))
    }
}

pub(super) fn parse_pdf_document_id(document_id: &str) -> Option<(AttachmentId, u32)> {
    let rest = document_id.strip_prefix("pdf:")?;
    let (attachment_id, page) = rest.rsplit_once(":page:")?;
    Some((
        AttachmentId::from(attachment_id.to_string()),
        page.parse().ok()?,
    ))
}

fn attachment_id_from_location(location: Option<&str>) -> Option<AttachmentId> {
    let rest = location?.split_once("/attachment/")?.1;
    let attachment_id = rest.split('/').next()?;
    if attachment_id.is_empty() {
        return None;
    }
    Some(AttachmentId::from(attachment_id.to_string()))
}

fn page_from_location(location: Option<&str>) -> Option<u32> {
    let rest = location?.rsplit_once("/page/")?.1;
    rest.split('/').next()?.parse().ok()
}
