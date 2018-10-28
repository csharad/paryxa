pub mod question_answer;
pub mod question_option;
pub mod test_paper;
pub mod test_question;
pub mod test_attempt;
pub mod test_schedule;
pub mod test_subscription;
pub mod user;

/// Merge values of `Option<T>` and `Option<bool>` into a patch value
/// `Option<Option<T>>`.
trait JoinPatch<T> {
    fn join(self, is_null: Option<bool>) -> Option<Option<T>>;
}

impl<T> JoinPatch<T> for Option<T> {
    fn join(self, is_null: Option<bool>) -> Option<Option<T>> {
        if is_null.unwrap_or_default() {
            Some(None)
        } else if let Some(val) = self {
            Some(Some(val))
        } else {
            None
        }
    }
}
