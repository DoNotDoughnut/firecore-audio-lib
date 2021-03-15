#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Sound {

    pub name: String,
    pub variant: u16,

}