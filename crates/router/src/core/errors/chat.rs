#[derive(Debug, thiserror::Error)]
pub enum ChatErrors {
    #[error("User InternalServerError")]
    InternalServerError,
    #[error("Missing Config error")]
    MissingConfigError,
    #[error("Chat response deserialization failed")]
    ChatResponseDeserializationFailed,
<<<<<<< HEAD
    #[error("Unauthorized access")]
    UnauthorizedAccess,
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
}

impl common_utils::errors::ErrorSwitch<api_models::errors::types::ApiErrorResponse> for ChatErrors {
    fn switch(&self) -> api_models::errors::types::ApiErrorResponse {
        use api_models::errors::types::{ApiError, ApiErrorResponse as AER};
        let sub_code = "AI";
        match self {
            Self::InternalServerError => {
                AER::InternalServerError(ApiError::new("HE", 0, self.get_error_message(), None))
            }
            Self::MissingConfigError => {
                AER::InternalServerError(ApiError::new(sub_code, 1, self.get_error_message(), None))
            }
            Self::ChatResponseDeserializationFailed => {
                AER::BadRequest(ApiError::new(sub_code, 2, self.get_error_message(), None))
            }
<<<<<<< HEAD
            Self::UnauthorizedAccess => {
                AER::Unauthorized(ApiError::new(sub_code, 3, self.get_error_message(), None))
            }
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        }
    }
}

impl ChatErrors {
    pub fn get_error_message(&self) -> String {
        match self {
            Self::InternalServerError => "Something went wrong".to_string(),
            Self::MissingConfigError => "Missing webhook url".to_string(),
            Self::ChatResponseDeserializationFailed => "Failed to parse chat response".to_string(),
<<<<<<< HEAD
            Self::UnauthorizedAccess => "Not authorized to access the resource".to_string(),
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        }
    }
}
