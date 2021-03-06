use ncube_data::{ReqCtx, SuccessResponse};
use ncube_handlers::source as handlers;
use tracing::instrument;
use warp::Filter;

use crate::http::authenticate_remote_req;

#[instrument]
async fn list(_ctx: ReqCtx, workspace_slug: String) -> Result<impl warp::Reply, warp::Rejection> {
    let sources = handlers::list_source_tags(&workspace_slug).await?;
    let response = SuccessResponse::new(sources);

    Ok(warp::reply::json(&response))
}

#[instrument]
async fn remove(
    _ctx: ReqCtx,
    workspace_slug: String,
    tag: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    handlers::remove_source_tag(&workspace_slug, &tag).await?;

    Ok(warp::reply())
}

// FIXME: turn this into /workspaces/<workspace_id>/sources/tags endpoint
pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate_remote_req()
        .and(warp::path!("workspaces" / String / "source-tags"))
        .and(warp::get())
        .and_then(list)
        .or(authenticate_remote_req()
            .and(warp::path!("workspaces" / String / "source-tags" / String))
            .and(warp::delete())
            .and_then(remove))
}
