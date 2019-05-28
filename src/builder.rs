use std::{mem, ptr};

use boolector_sys as ffi;

use crate::{GenerateModel, Solver};

/// Solver instance builder.
pub struct Builder {
    ptr: *mut ffi::Btor,
}

impl Builder {
    /// Start building a solver instance.
    ///
    /// By default the instance will have these properties:
    ///
    /// * no model generation;
    /// * non-incremental.
    pub fn new() -> Self {
        Builder {
            ptr: unsafe { ffi::boolector_new() },
        }
    }

    /// Consume the builder and return the constructed instance.
    pub fn finish<'a>(mut self) -> Solver<'a> {
        unsafe {
            Solver::from_ffi(mem::replace(&mut self.ptr, ptr::null_mut()))
        }
    }

    /// Enable model generation.
    pub fn generate_model(self, mode: GenerateModel) -> Self {
        unsafe {
            ffi::boolector_set_opt(
                self.ptr,
                ffi::BtorOption_BTOR_OPT_MODEL_GEN,
                mode as u32,
            );
        }

        self
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::boolector_delete(self.ptr)
            }
        }
    }
}
