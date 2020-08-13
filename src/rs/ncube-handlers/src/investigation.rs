use ncube_data::VerifySegmentReq;
use ncube_stores::{investigation_store, methodology_store, search_store, segment_store};
use tracing::instrument;

use crate::{ensure_workspace, workspace_database, HandlerError};

#[instrument]
pub async fn verify_segment(
    workspace: &str,
    investigation: &str,
    segment_req: &VerifySegmentReq,
) -> Result<(), HandlerError> {
    ensure_workspace(&workspace).await?;

    let database = workspace_database(&workspace).await?;

    let investigation_store = investigation_store(database.clone());
    let methodology_store = methodology_store(database.clone());
    let segment_store = segment_store(database.clone());
    let search_store = search_store(database.clone());

    let investigation = investigation_store
        .show(&investigation)
        .await?
        .ok_or_else(|| {
            HandlerError::NotFound(format!(
                "Investigation '{}' could not be found.",
                investigation
            ))
        })?;

    let methodology = methodology_store
        .show(&investigation.methodology)
        .await?
        .ok_or_else(|| {
            HandlerError::NotFound(format!(
                "Methodology '{}' could not be found.",
                investigation.methodology
            ))
        })?;

    let segment = segment_store
        .show(&segment_req.segment)
        .await?
        .ok_or_else(|| {
            HandlerError::NotFound(format!(
                "Segment '{}' could not be found.",
                segment_req.segment
            ))
        })?;

    let units = search_store.data_list(&segment.query).await?;

    investigation_store
        .verify_segment(
            investigation.id,
            segment.id,
            units,
            &methodology.initial_state,
        )
        .await?;

    Ok(())
}
