use std::rc::Rc;

use crate::{
    backend::{Context, Facade},
    context::CommandContext,
    gl,
    uniforms::{AsUniformValue, UniformValue},
    ContextExt, GlObject, TextureExt,
};

/// Corresponds to C `EGLImage`. Actually this is a `void*`.
pub type EglImage = *const std::ffi::c_void;

/// A special kind of texture bound to data from outside of OpenGL.
pub struct ExternalTexture {
    context: Rc<Context>,
    id: gl::types::GLuint,
}

impl ExternalTexture {
    /// Create an external texture from an `EGLImage`.
    /// It is user's responsibility to call EGL and bring an `EGLImage`.
    pub fn new<F: Facade>(facade: &F, image: EglImage) -> Self {
        let ctxt = facade.get_context().make_current();

        let id = unsafe {
            let mut id: gl::types::GLuint = 0;
            ctxt.gl.GenTextures(1, &mut id);

            ctxt.gl.BindTexture(gl::TEXTURE_EXTERNAL_OES, id);
            ctxt.gl
                .EGLImageTargetTexture2DOES(gl::TEXTURE_EXTERNAL_OES, image);
            ctxt.gl.BindTexture(gl::TEXTURE_EXTERNAL_OES, 0);

            id
        };

        Self {
            context: Rc::clone(facade.get_context()),
            id,
        }
    }
}

impl GlObject for ExternalTexture {
    type Id = gl::types::GLuint;

    #[inline]
    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<'a> AsUniformValue for &'a ExternalTexture {
    #[inline]
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::ExternalTexture(*self, None)
    }
}

impl AsUniformValue for ExternalTexture {
    #[inline]
    fn as_uniform_value(&self) -> crate::uniforms::UniformValue<'_> {
        UniformValue::ExternalTexture(self, None)
    }
}

impl<'a> TextureExt for &'a ExternalTexture {
    #[inline]
    fn get_texture_id(&self) -> gl::types::GLuint {
        self.id
    }

    #[inline]
    fn get_context(&self) -> &Rc<Context> {
        &self.context
    }

    #[inline]
    fn get_bind_point(&self) -> gl::types::GLenum {
        gl::TEXTURE_EXTERNAL_OES
    }

    #[inline]
    fn bind_to_current(&self, ctxt: &mut CommandContext<'_>) -> gl::types::GLenum {
        unsafe {
            ctxt.gl.BindTexture(gl::TEXTURE_EXTERNAL_OES, self.id);
        }
        gl::TEXTURE_EXTERNAL_OES
    }

    fn prepare_for_access(&self, _: &mut CommandContext<'_>, access_type: crate::TextureAccess) {}
}
