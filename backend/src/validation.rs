//! Ergonomic validation primitives.
//!
//! # Usage
//!
//! Use [`Validator::field`] to start a rule chain for a field, then call
//! [`.check()`](FieldValidator::check) for each rule.
//!
//! ```ignore
//! struct TestStruct { subject: String }
//!
//! impl TestStruct {
//!     fn validate(&self) -> Result<(), ValidationErrors> {
//!         let mut v = Validator::new();
//!         v.field("subject", &self.subject)
//!             .check(rules::not_empty)
//!             .check(rules::max_len(255));
//!         v.finish()
//!     }
//! }
//! ```

pub type ValidationErrors = std::collections::HashMap<String, Vec<String>>;

/// Implemented by types that can have a validation rule applied to their inner value.
///
/// This trait lets [`FieldValidator::check`] work identically regardless of
/// whether the field is a plain value or a wrapper (the wrapper decides).
pub trait Validatable<T: ?Sized> {
    fn apply<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce(&T) -> Result<(), E>;
}

/// Implements [`Validatable<T>`] for a base/primitive type `T`, where the validation
/// rule is applied directly to `self`.
///
/// # Example
/// ```
/// impl_validatable!(i32);
/// // expands to:
/// // impl Validatable<i32> for i32 {
/// //     fn apply<E, F>(&self, f: F) -> Result<(), E>
/// //     where
/// //         F: FnOnce(&i32) -> Result<(), E>,
/// //     {
/// //         f(self)
/// //     }
/// // }
/// ```
macro_rules! impl_validatable {
    ($t:ty) => {
        impl Validatable<$t> for $t {
            fn apply<E, F>(&self, f: F) -> Result<(), E>
            where
                F: FnOnce(&$t) -> Result<(), E>,
            {
                f(self)
            }
        }
    };
}

// Explicitly implement Validatable for base primitives.
// We do this instead of a blanket `impl<T> Validatable<T> for T` so that
// wrapper types (e.g. `Option<T>`, `PatchField<T>`) can implement their own
// recursive delegation to `T`'s `Validatable` implementation, without triggering
// Rust's overlapping implementation error (E0119).
impl_validatable!(i32);
impl_validatable!(i64);
impl_validatable!(f32);
impl_validatable!(f64);
impl_validatable!(bool);
impl_validatable!(usize);
impl_validatable!(uuid::Uuid);
impl_validatable!(chrono::DateTime<chrono::Utc>);
impl_validatable!(str);

/// Allows using `&str` rules for `String` fields.
impl Validatable<str> for String {
    fn apply<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce(&str) -> Result<(), E>,
    {
        f(self.as_str())
    }
}

/// Trait delegation: `Option<T>` delegates to `T`'s `Validatable<U>` implementation.
/// This allows `Option<String>` to automatically use `&str` rules,
/// `Option<i32>` to use `&i32` rules, etc.
impl<T, U: ?Sized> Validatable<U> for Option<T>
where
    T: Validatable<U>,
{
    fn apply<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce(&U) -> Result<(), E>,
    {
        match self {
            Some(v) => v.apply(f),
            None => Ok(()),
        }
    }
}

/// Collects validation errors.
///
/// Use [`field`](Validator::field) to validate fields, then
/// [`finish`](Validator::finish) to get the result.
pub struct Validator(ValidationErrors);

impl Validator {
    pub fn new() -> Self {
        Self(ValidationErrors::new())
    }

    /// Begin validating a field. Returns a [`FieldValidator`] builder.
    pub fn field<'a, V: ?Sized>(
        &'a mut self,
        name: &'a str,
        value: &'a V,
    ) -> FieldValidator<'a, V> {
        FieldValidator {
            validator: self,
            field: name,
            value,
        }
    }

    /// Record an error for `field`. Used internally by [`FieldValidator::check`].
    fn push_error(&mut self, field: &str, msg: impl Into<String>) {
        self.0
            .entry(field.to_string())
            .or_default()
            .push(msg.into());
    }

    /// Return the validation result.
    pub fn finish(self) -> Result<(), ValidationErrors> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(self.0)
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder returned by [`Validator::field`].
///
/// Chain [`.check()`](FieldValidator::check) calls to apply multiple rules.
///
/// ```ignore
/// v.field("subject", &self.subject)
///     .check(rules::not_empty)
///     .check(rules::max_len(255));
/// ```
pub struct FieldValidator<'a, V: ?Sized> {
    validator: &'a mut Validator,
    field: &'a str,
    value: &'a V,
}

impl<'a, V: ?Sized> FieldValidator<'a, V> {
    /// Apply rule `f` to the field value and record any error.
    ///
    /// Returns `&mut Self` so rules can be chained.
    pub fn check<T, E, F>(&mut self, f: F) -> &mut Self
    where
        T: ?Sized,
        V: Validatable<T>,
        F: FnOnce(&T) -> Result<(), E>,
        E: Into<String>,
    {
        // `&str` and `&V` are both Copy — copy them out so the borrow checker
        // sees no overlap between these reads and the mutable borrow of
        // `self.validator` below.
        let field = self.field;
        let value = self.value;
        if let Err(e) = value.apply(f) {
            self.validator.push_error(field, e);
        }
        self
    }
}

/// Reusable validation rules.
pub mod rules {
    /// Fails if the string is empty or contains only whitespace.
    pub fn not_empty(s: &str) -> Result<(), &'static str> {
        if s.trim().is_empty() {
            Err("Cannot be empty")
        } else {
            Ok(())
        }
    }

    /// Fails if `s.len() > max`.
    pub fn max_len(max: usize) -> impl Fn(&str) -> Result<(), String> {
        move |s| {
            if s.len() > max {
                Err(format!("Cannot exceed {max} characters"))
            } else {
                Ok(())
            }
        }
    }

    /// Fails if `s.len() < min`.
    pub fn min_len(min: usize) -> impl Fn(&str) -> Result<(), String> {
        move |s| {
            if s.len() < min {
                Err(format!("Must be at least {min} characters"))
            } else {
                Ok(())
            }
        }
    }

    /// Fails if `s.len()` is outside `[min, max]` (inclusive).
    pub fn between_len(min: usize, max: usize) -> impl Fn(&str) -> Result<(), String> {
        move |s| {
            let len = s.len();
            if len < min || len > max {
                Err(format!("Must be between {min} and {max} characters"))
            } else {
                Ok(())
            }
        }
    }

    /// Fails if `s` is not a valid email address (RFC 5321/5322 via `email_address` crate).
    pub fn email(s: &str) -> Result<(), &'static str> {
        if email_address::EmailAddress::is_valid(s) {
            Ok(())
        } else {
            Err("Must be a valid email address")
        }
    }

    /// Fails if `s` contains characters other than lowercase ASCII letters, digits, or
    /// hyphens, or if `s` starts or ends with a hyphen.
    pub fn slug(s: &str) -> Result<(), &'static str> {
        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err("Can only contain lowercase letters, digits, and hyphens");
        }
        if s.starts_with('-') || s.ends_with('-') {
            return Err("Cannot start or end with a hyphen");
        }
        Ok(())
    }

    /// Password strength: at least 8 characters AND at least one non-alphabetic character.
    pub fn password_strength(s: &str) -> Result<(), &'static str> {
        if s.len() < 8 {
            return Err("Must be at least 8 characters");
        }
        if !s.chars().any(|c| !c.is_alphabetic()) {
            return Err("Must contain at least one non-letter character (digit or symbol)");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::rules::*;
    use super::*;

    // --- rules ---

    #[test]
    fn not_empty_rejects_blank() {
        assert!(not_empty("").is_err());
        assert!(not_empty("   ").is_err());
        assert!(not_empty("x").is_ok());
    }

    #[test]
    fn max_len_boundary() {
        let rule = max_len(3);
        assert!(rule("abc").is_ok());
        assert!(rule("abcd").is_err());
    }

    #[test]
    fn min_len_boundary() {
        let rule = min_len(3);
        assert!(rule("abc").is_ok());
        assert!(rule("ab").is_err());
    }

    #[test]
    fn between_len_boundaries() {
        let rule = between_len(3, 5);
        assert!(rule("abc").is_ok());
        assert!(rule("ab").is_err());
        assert!(rule("abcdef").is_err());
    }

    #[test]
    fn email_valid_and_invalid() {
        assert!(email("user@example.com").is_ok());
        assert!(email("not-an-email").is_err());
        assert!(email("@nodomain.com").is_err());
        assert!(email("noatsign.com").is_err());
    }

    #[test]
    fn slug_valid_and_invalid() {
        assert!(slug("my-project-1").is_ok());
        assert!(slug("UPPER").is_err());
        assert!(slug("has space").is_err());
        assert!(slug("under_score").is_err());
        assert!(slug("-leading").is_err());
        assert!(slug("trailing-").is_err());
    }

    #[test]
    fn password_strength_cases() {
        assert!(password_strength("short1").is_err());
        assert!(password_strength("allletter").is_err());
        assert!(password_strength("validpass1").is_ok());
        assert!(password_strength("validpass!").is_ok());
    }

    // --- Validator ---

    #[test]
    fn validator_collects_all_errors() {
        let mut v = Validator::new();
        v.field("name", "").check(not_empty);
        v.field("name", "toolongname").check(max_len(5));
        let errs = v.finish().unwrap_err();
        assert_eq!(errs["name"].len(), 2);
    }

    #[test]
    fn validator_ok_when_no_errors() {
        let mut v = Validator::new();
        v.field("name", "valid").check(not_empty);
        assert!(v.finish().is_ok());
    }
}
