use crate::errors::Error;
use anyhow::Result;
use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: &str) -> Result<Self> {
        if validate_email(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::ValidateSubscriberEmailError(format!(
                "{} is not a valid subscriber email.",
                s
            ))
            .into())
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use pretty_assertions::assert_eq;
    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_eq!(SubscriberEmail::parse(&email).is_err(), true);
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_eq!(SubscriberEmail::parse(&email).is_err(), true);
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_eq!(SubscriberEmail::parse(&email).is_err(), true);
    }
}
