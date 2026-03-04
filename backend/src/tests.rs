use crate::dto::user::{CreateUser, UserLogin};
use validator::Validate;
use crate::dto::server_dto::CreateServer;
use crate::dto::channel_dto::CreateChannel;
use crate::dto::message_schema_input::SimpleMessageSchema;

#[cfg(test)]
mod tests {
    use super::*;

       // Ce test est juste pour valider la branche de création
    #[test]
    fn test_create_user_valid() {
        let user = CreateUser {
            username: "Ivana".to_string(),
            email: "ivana@epitech.eu".to_string(),
            password: "password123".to_string(),
        };
        assert!(user.validate().is_ok());
    }

    #[test]
    fn test_create_user_invalid_email() {
        let user = CreateUser {
            username: "Ivana".to_string(),
            email: "format_incorrect".to_string(),
            password: "password123".to_string(),
        };
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_login_validation() {
        let login = UserLogin {
            email: "test@epitech.eu".to_string(),
            password: "password".to_string(),
        };
        assert!(login.validate().is_ok());
    }

    #[test]
    fn test_username_logic() {
        let _user = CreateUser {
            username: "Iv".to_string(),
            email: "test@test.com".to_string(),
            password: "pass".to_string(),
        };
    }
}

//test pour la création de serveurs
#[cfg(test)]
mod additional_tests {
    use super::*;
    #[test]
    fn test_server_validation() {
        let valid_server = CreateServer { name: "Gaming Room".to_string() };
        assert!(valid_server.validate().is_ok());

        let invalid_server = CreateServer { name: "Ab".to_string() };
        assert!(invalid_server.validate().is_err());
    }



//test pour la création du channel

    #[test]
    fn test_channel_validation() {
        let valid_chan = CreateChannel { name: "général".to_string() };
        assert!(valid_chan.validate().is_ok());

        let invalid_chan = CreateChannel { name: "".to_string() };
        assert!(invalid_chan.validate().is_err());
    }



//test pour la création des message
    #[test]
    fn test_message_content() {
        let msg = SimpleMessageSchema { content: "Coucou !".to_string() };
        assert_eq!(msg.content, "Coucou !");
        assert!(!msg.content.is_empty());
    }
}