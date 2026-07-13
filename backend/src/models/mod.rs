pub mod project;
pub mod user;

use serde::{Deserialize, Deserializer, Serialize};
use serde_with::de::{DeserializeAs, DeserializeAsWrap};

use crate::validation::Validatable;

/// Field wrapper for PATCH endpoints, to distinguish explicit JSON `null` from absent fields.
///
/// - Nullable field has value -> `Value(Some(v))`
/// - Nullable field is `null` -> `Value(None)`
/// - Nullable field absent    -> `Absent` (via `#[serde(default)]`)
///
/// For nullable columns (`PatchField<Option<T>>`), an explicit JSON `null`
/// will deserialize to `Value(None)` (explicitly clear the column),
/// since `Option<T>::deserialize` accepts `null`.
///
/// For non-nullable columns (`PatchField<T>`), an explicit JSON `null`
/// will be a deserialization error (400 Bad Request), since `T::deserialize`
/// rejects `null` unless `T` itself is nullable.
#[derive(Debug, Clone, PartialEq)]
pub enum PatchField<T> {
    Value(T),
    Absent,
}

impl<T> Default for PatchField<T> {
    fn default() -> Self {
        PatchField::Absent
    }
}

impl<'de, T> Deserialize<'de> for PatchField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(PatchField::Value)
    }
}

impl<'de, T, U> DeserializeAs<'de, PatchField<T>> for PatchField<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<PatchField<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            match PatchField::<DeserializeAsWrap<T, U>>::deserialize(deserializer)? {
                PatchField::Value(v) => PatchField::Value(v.into_inner()),
                PatchField::Absent => PatchField::Absent,
            },
        )
    }
}

/// `PatchField<T>` implements [`Validatable<T>`]:
/// - applies the validation rule when `Value`
/// - skips validation and returns `Ok(())` when `Absent`
impl<T> Validatable<T> for PatchField<T> {
    fn apply<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce(&T) -> Result<(), E>,
    {
        match self {
            PatchField::Value(v) => f(v),
            PatchField::Absent => Ok(()),
        }
    }
}

/// `PatchField<String>` fields can use `&str` rules with the same syntax as plain `String`
/// fields — Rust infers `T = str` from the rule signature, and this impl handles the deref.
impl Validatable<str> for PatchField<String> {
    fn apply<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce(&str) -> Result<(), E>,
    {
        match self {
            PatchField::Value(v) => f(v.as_str()),
            PatchField::Absent => Ok(()),
        }
    }
}

/// Query parameters for paginated list endpoints.
///
/// Both fields are optional; defaults are applied when absent.
/// `per_page` is capped at 100 to prevent runaway queries.
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(25).min(100)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }
}

/// Standard envelope for paginated list responses.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        Self {
            data,
            total,
            page: params.page(),
            limit: params.limit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::{Validator, rules};

    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(default)]
    struct TestStruct {
        name: PatchField<String>,
        age: PatchField<Option<i32>>,
    }

    impl Default for TestStruct {
        fn default() -> Self {
            Self {
                name: PatchField::Absent,
                age: PatchField::Absent,
            }
        }
    }

    // --- Deserialize ---

    #[test]
    fn test_patch_field_absent() {
        let json = r#"{}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.name, PatchField::Absent);
        assert_eq!(s.age, PatchField::Absent);
    }

    #[test]
    fn test_patch_field_value_present() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.name, PatchField::Value("Alice".to_string()));
        assert_eq!(s.age, PatchField::Value(Some(30)));
    }

    #[test]
    fn test_patch_field_non_nullable_rejects_null() {
        let json = r#"{"name": null}"#;
        let result: Result<TestStruct, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Expected deserialization to fail for explicit null on non-nullable field"
        );
    }

    #[test]
    fn test_patch_field_nullable_accepts_null() {
        let json = r#"{"age": null}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.age, PatchField::Value(None));
    }

    // --- Validatable ---

    #[test]
    fn patch_field_absent_skips_rule() {
        let mut v = Validator::new();
        let field: PatchField<String> = PatchField::Absent;
        v.field("name", &field).check(rules::not_empty);
        assert!(v.finish().is_ok());
    }

    #[test]
    fn patch_field_present_runs_rule() {
        let mut v = Validator::new();
        let field = PatchField::Value(String::new());
        v.field("name", &field).check(rules::not_empty);
        assert!(v.finish().is_err());
    }

    #[test]
    fn plain_and_patch_field_identical_syntax() {
        // Demonstrates that call sites look identical regardless of field type.
        let plain: String = "hello".into();
        let patch: PatchField<String> = PatchField::Value("hello".into());

        let mut vp = Validator::new();
        let mut vf = Validator::new();
        vp.field("name", &plain).check(rules::not_empty);
        vf.field("name", &patch).check(rules::not_empty);

        assert!(vp.finish().is_ok());
        assert!(vf.finish().is_ok());
    }
}
