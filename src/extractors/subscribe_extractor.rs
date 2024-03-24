use serde::Deserialize;


#[derive(Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String
}