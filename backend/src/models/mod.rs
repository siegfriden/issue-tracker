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

/// `PatchField<T>` implements [`Validatable<U>`] if `T` also implements `Validatable<U>`:
/// - when `Value`  -> invoke `T`'s `Validatable<U>` implementation (trait delegation)
/// - when `Absent` -> skips validation and returns `Ok(())`
impl<T, U: ?Sized> Validatable<U> for PatchField<T>
where
    T: Validatable<U>,
{
    fn apply<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce(&U) -> Result<(), E>,
    {
        match self {
            PatchField::Value(v) => v.apply(f),
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
    fn patch_field_absent() {
        let json = r#"{}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.name, PatchField::Absent);
        assert_eq!(s.age, PatchField::Absent);
    }

    #[test]
    fn patch_field_value_present() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let s: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(s.name, PatchField::Value("Alice".to_string()));
        assert_eq!(s.age, PatchField::Value(Some(30)));
    }

    #[test]
    fn patch_field_non_nullable_rejects_null() {
        let json = r#"{"name": null}"#;
        let result: Result<TestStruct, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Expected deserialization to fail for explicit null on non-nullable field"
        );
    }

    #[test]
    fn patch_field_nullable_accepts_null() {
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
    fn patch_field_and_plain_identical_syntax() {
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

    #[test]
    fn patch_field_option_string_validation() {
        let mut v = Validator::new();

        let absent: PatchField<Option<String>> = PatchField::Absent;
        let null_val: PatchField<Option<String>> = PatchField::Value(None);
        let str_val: PatchField<Option<String>> = PatchField::Value(Some("valid".into()));
        let str_invalid: PatchField<Option<String>> = PatchField::Value(Some("".into()));

        v.field("absent", &absent).check(rules::not_empty);
        v.field("null_val", &null_val).check(rules::not_empty);
        v.field("str_val", &str_val).check(rules::not_empty);

        assert!(v.finish().is_ok());

        let mut v_err = Validator::new();
        v_err.field("invalid", &str_invalid).check(rules::not_empty);
        assert!(v_err.finish().is_err());
    }

    #[test]
    fn patch_field_various_types_validation() {
        use chrono::{DateTime, Utc};
        use uuid::Uuid;
        let mut v = Validator::new();

        // i32 and Option<i32>
        let pf_i32: PatchField<i32> = PatchField::Value(10);
        let pf_opt_i32: PatchField<Option<i32>> = PatchField::Value(Some(20));
        let pf_opt_i32_null: PatchField<Option<i32>> = PatchField::Value(None);
        let plain_opt_i32: Option<i32> = Some(30);

        fn is_positive(val: &i32) -> Result<(), &'static str> {
            if *val > 0 {
                Ok(())
            } else {
                Err("must be positive")
            }
        }

        v.field("i32", &pf_i32).check(is_positive);
        v.field("opt_i32", &pf_opt_i32).check(is_positive);
        v.field("opt_i32_null", &pf_opt_i32_null).check(is_positive);
        v.field("plain_opt_i32", &plain_opt_i32).check(is_positive);

        // Date and Option<Date>
        let pf_date: PatchField<DateTime<Utc>> = PatchField::Value(Utc::now());
        let pf_opt_date: PatchField<Option<DateTime<Utc>>> = PatchField::Value(Some(Utc::now()));

        fn always_valid_date(_d: &DateTime<Utc>) -> Result<(), &'static str> {
            Ok(())
        }

        v.field("date", &pf_date).check(always_valid_date);
        v.field("opt_date", &pf_opt_date).check(always_valid_date);

        // Uuid and Option<Uuid>
        let pf_uuid: PatchField<Uuid> = PatchField::Value(Uuid::new_v4());
        let pf_opt_uuid: PatchField<Option<Uuid>> = PatchField::Value(Some(Uuid::new_v4()));
        let plain_opt_uuid: Option<Uuid> = Some(Uuid::new_v4());

        fn not_nil(id: &Uuid) -> Result<(), &'static str> {
            if !id.is_nil() {
                Ok(())
            } else {
                Err("cannot be nil")
            }
        }

        v.field("uuid", &pf_uuid).check(not_nil);
        v.field("opt_uuid", &pf_opt_uuid).check(not_nil);
        v.field("plain_opt_uuid", &plain_opt_uuid).check(not_nil);

        assert!(v.finish().is_ok());
    }
}
