#[cfg(not(feature = "gl"))]
pub mod gfx;

#[cfg(not(feature = "gl"))]
pub use self::gfx::*;

use super::*;

pub trait ObjectRenderData {

}