#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct Sound {

    pub name: String,
    pub variant: u16,

}

impl Sound {

    pub fn of(name: &str, variant: u16) -> Self {
        Self {
            name: name.to_owned(),
            variant,
        }
    }

}