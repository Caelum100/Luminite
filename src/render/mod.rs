#[cfg(not(feature = "gl"))]
pub mod gfx;

#[cfg(not(feature = "gl"))]
pub use self::gfx::*;

use super::*;

/// A render backend.
pub trait RenderBackend {
    type Backend;
    /// Render data associated with an object
    type ObjectRender;
    type RenderContext;
}

/// Produces a model-view-projection matrix
/// for the specified object.
fn mvp_matrix<B: RenderBackend>(object: &Object<B>) -> (Mat4, Mat4) {
    use glm::ext::*;
    let translation = translate(&num::one(), object.location.to_vec());

    let rotation: Mat4 = rotate(
        &num::one(),
        radians(object.location.yaw),
        vec3(0.0, 1.0, 0.0),
    )
        * rotate(
            &num::one(),
            radians(object.location.pitch),
            vec3(1.0, 0.0, 0.0),
        );

    let scale: Mat4 = num::one();
    let model = translation * rotation * scale;

    // TODO moving camera
    let view = look_at(
        vec3(4.0, 3.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    );

    // TODO view distance, custom aspect ratio
    let projection = perspective(45.0f32, 4.0 / 3.0, 0.1, 128.0);
    (projection * view * model, view * model)
}
