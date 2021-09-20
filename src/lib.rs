mod envy;

pub use crate::envy::Envy;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Envy;

    #[test]
    fn valid_field() {
        let envy = Envy::debug().unwrap();
        assert_eq!(envy.get("KEY"), String::from("VALUE"));
        assert_eq!(envy.get("KEY2"), String::from("#VALUE"));
    }

    #[test]
    fn commented_field() {
        let envy = Envy::debug().unwrap();
        assert_eq!(envy.get("COMMENT"), String::new());
    }

    #[test]
    fn invalid_field() {
        let envy = Envy::debug().unwrap();
        assert_eq!(envy.get("BROKEN"), String::new());
        assert_eq!(envy.get("BROKEN2"), String::new());
    }

    #[test]
    fn comment_at_end() {
        let envy = Envy::debug().unwrap();
        assert_eq!(envy.get("COMMENT2"), String::from("OK"));
    }

    #[test]
    fn debug() {
        let envy = Envy::debug().unwrap();
        assert_eq!(envy.get("ENV"), String::from("DEBUG"));
        assert!(envy.amount() == 4);
    }

    #[test]
    fn release() {
        let envy = Envy::release().unwrap();
        assert_eq!(envy.get("ENV"), String::from("RELEASE"));
        assert!(envy.amount() == 4);
    }

    #[test]
    fn test() {
        let envy = Envy::test().unwrap();
        assert_eq!(envy.get("ENV"), String::from("TEST"));
        assert!(envy.amount() == 4);
    }

    #[test]
    fn try_from() {
        let envy = Envy::try_from(".custom.env").unwrap();
        assert_eq!(envy.get("ENV"), String::from("CUSTOM"));
        assert!(envy.amount() == 4);
    }

    #[test]
    fn try_from_fails() {
        let envy = Envy::try_from(".notok-custom.env");
        assert!(envy.is_err());
    }

    #[test]
    fn current() {
        let envy = Envy::current().unwrap();
        assert_eq!(envy.get("ENV"), String::from("TEST"));
        assert!(envy.amount() == 4);
    }

    // TODO Should test debug and release for current
}
