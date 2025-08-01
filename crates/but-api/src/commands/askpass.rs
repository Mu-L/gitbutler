//! In place of commands.rs
use gitbutler_id::id::Id;
use gitbutler_repo_actions::askpass::{self, AskpassRequest};
use serde::Deserialize;

use crate::{IpcContext, error::Error};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitPromptResponseParams {
    pub id: Id<AskpassRequest>,
    pub response: Option<String>,
}

pub async fn submit_prompt_response(
    _ipc_ctx: &IpcContext,
    params: SubmitPromptResponseParams,
) -> Result<(), Error> {
    askpass::get_broker()
        .handle_response(params.id, params.response)
        .await;
    Ok(())
}
