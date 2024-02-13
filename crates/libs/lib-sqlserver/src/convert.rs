pub trait TryFromRow<T>: Sized {
    fn try_from_row(row: T) -> Result<Self, crate::model::Error>;
}
