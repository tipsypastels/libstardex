mod evolution;
mod species;
mod r#type;

pub use evolution::*;
pub use species::*;
pub use r#type::*;

#[cfg(feature = "sync")]
type RefCnt<T> = std::sync::Arc<T>;

#[cfg(not(feature = "sync"))]
type RefCnt<T> = std::rc::Rc<T>;

type RefStr = RefCnt<str>;
