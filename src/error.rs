pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ShapeError(#[from] ndarray::ShapeError),
    #[error("{0}")]
    GraphError(String),
    #[error("Array2<i32> cannot be converted to Array2<bool>")]
    ConvertToBoolMatrix,
    #[error("Variable with type i32 is not 1 or 0")]
    I32ToBool,
    #[error("")]
    CannotReshapeMatrix,
    #[error("Number of matching columns when building the tree is zero")]
    UnresolvedTree,
}