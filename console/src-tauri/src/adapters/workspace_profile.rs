use lunar::adapters::meta::RequestMeta;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceProfile {
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
    pub meta: Option<RequestMeta>,
}

impl From<CreateWorkspaceProfile> for lunar::adapters::workspace_profiles::CreateWorkspaceProfile {
    fn from(p: CreateWorkspaceProfile) -> Self {
        Self {
            first_name: p.first_name,
            last_name: p.last_name,
            profile_picture: p.profile_picture,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspaceProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
    pub meta: Option<RequestMeta>,
}

impl From<UpdateWorkspaceProfile> for lunar::adapters::workspace_profiles::UpdateWorkspaceProfile {
    fn from(p: UpdateWorkspaceProfile) -> Self {
        Self {
            first_name: p.first_name,
            last_name: p.last_name,
            profile_picture: p.profile_picture,
        }
    }
}
