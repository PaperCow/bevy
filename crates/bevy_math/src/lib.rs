#![warn(missing_docs)]

//! Provides math types and functionality for the Bevy game engine.
//!
//! The commonly used types are vectors like [`Vec2`], [`Vec3`] and [`Vec4`] and
//! matrices like [`Mat3`] and [`Mat4`].

pub use glam::*;

/// The `bevy_math` prelude.
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        BVec2, BVec3, BVec4, EulerRot, IVec2, IVec3, IVec4, Mat3, Mat4, Quat, UVec2, UVec3, UVec4,
        Vec2, Vec3, Vec4,
    };
}
