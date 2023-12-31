pub mod commit_check;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GitConfig {
    pub commit_check: Option<commit_check::CommitCheckConfig>,
}
