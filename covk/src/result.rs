use crate::*;

/// Result type for Vulkan functions
pub type Result<T> = core::result::Result<T, vk::Result>;

impl core::error::Error for vk::Result {}

impl vk::Result {
    /// `true` if the result is a success code.
    pub const fn is_success(self) -> bool {
        self.0 >= 0
    }

    /// `true` if the result is an error code.
    pub const fn is_error(self) -> bool {
        self.0 < 0
    }

    pub fn result<T>(self, ok: impl FnOnce() -> Option<T>) -> core::result::Result<T, Self> {
        if self.is_success() {
            Ok(ok().ok_or(self)?)
        } else {
            Err(self)
        }
    }

    pub fn result_multi<T>(
        self,
        ok: impl FnOnce() -> Option<T>,
    ) -> core::result::Result<(T, Self), Self> {
        if matches!(self, vk::Result::Success | vk::Result::Incomplete) {
            Ok((ok().ok_or(self)?, self))
        } else {
            Err(self)
        }
    }

    pub fn result_by<T>(
        self,
        is_success: impl FnOnce(Self) -> bool,
        ok: impl FnOnce() -> Option<T>,
    ) -> core::result::Result<(T, Self), Self> {
        if is_success(self) {
            Ok((ok().ok_or(self)?, self))
        } else {
            Err(self)
        }
    }
}
