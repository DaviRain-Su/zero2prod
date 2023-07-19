use crate::errors::Error;
use anyhow::Result;
use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns an instance of `SubscribeName` if the input satisfies all our validation constraints
    /// on subscriber name, `false` otherwise.
    pub fn parse(s: &str) -> Result<Self> {
        // `.trim()` returns a view over the input `s` without trailing
        // whiterspace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as "user-percived"
        // character: `a` is a single grapheme, but is tis composed of two characters
        // (`a` and ``).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grpheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;

        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        // Return `false` if any of our coditions have veen violated
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(Error::ValidateSubscriberNameError("Subscriber name is invalid".into()).into())
        } else {
            Ok(Self(s.to_owned()))
        }
    }
}
