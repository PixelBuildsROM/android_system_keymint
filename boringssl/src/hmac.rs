use crate::{malloc_err, openssl_last_err};
use alloc::boxed::Box;
use alloc::vec::Vec;
#[cfg(soong)]
use bssl_ffi as ffi;
use kmr_common::{crypto, crypto::OpaqueOr, explicit, km_err, vec_try, Error};
use kmr_wire::keymint::Digest;
use log::error;

/// [`crypto::Hmac`] implementation based on BoringSSL.
pub struct BoringHmac;

impl crypto::Hmac for BoringHmac {
    fn begin(
        &self,
        key: OpaqueOr<crypto::hmac::Key>,
        digest: Digest,
    ) -> Result<Box<dyn crypto::AccumulatingOperation>, Error> {
        let key = explicit!(key)?;
        let op = BoringHmacOperation {
            ctx: unsafe {
                // Safety: raw pointer is immediately checked for null below.
                ffi::HMAC_CTX_new()
            },
        };
        if op.ctx.is_null() {
            return Err(malloc_err!());
        }

        let digest = digest_into_openssl_ffi(digest)?;
        #[cfg(soong)]
        let key_len = key.0.len();
        #[cfg(not(soong))]
        let key_len = key.0.len() as i32;

        let result = unsafe {
            // Safety: `op.ctx` is known non-null, as is the result of `digest_into_openssl_ffi`.
            // `key_len` is length of `key.0`, which is a valid `Vec<u8>`.
            ffi::HMAC_Init_ex(
                op.ctx,
                key.0.as_ptr() as *const libc::c_void,
                key_len,
                digest,
                core::ptr::null_mut(),
            )
        };
        if result != 1 {
            error!("Failed to HMAC_Init_ex()");
            return Err(openssl_last_err());
        }
        Ok(Box::new(op))
    }
}

/// [`crypto::HmacOperation`] implementation based on BoringSSL.
///
/// This implementation uses the `unsafe` wrappers around `HMAC_*` functions directly, because
/// BoringSSL does not support the `EVP_PKEY_HMAC` implementations that are used in the rust-openssl
/// crate.
pub struct BoringHmacOperation {
    // Safety: `ctx` is always non-null except for initial error path in `begin()`
    ctx: *mut ffi::HMAC_CTX,
}

impl core::ops::Drop for BoringHmacOperation {
    fn drop(&mut self) {
        unsafe {
            // Safety: `self.ctx` might be null (in the error path when `ffi::HMAC_CTX_new` fails)
            // but `ffi::HMAC_CTX_free` copes with null.
            ffi::HMAC_CTX_free(self.ctx);
        }
    }
}

impl crypto::AccumulatingOperation for BoringHmacOperation {
    fn update(&mut self, data: &[u8]) -> Result<(), Error> {
        let result = unsafe {
            // Safety: `self.ctx` is non-null, and `data` is a valid slice.
            ffi::HMAC_Update(self.ctx, data.as_ptr(), data.len())
        };
        if result != 1 {
            return Err(openssl_last_err());
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<Vec<u8>, Error> {
        let mut output_len = ffi::EVP_MAX_MD_SIZE as u32;
        let mut output = vec_try![0; ffi::EVP_MAX_MD_SIZE as usize]?;

        let result = unsafe {
            // Safety: `self.ctx` is non-null; `output_len` is correct size of `output` buffer.
            ffi::HMAC_Final(self.ctx, output.as_mut_ptr(), &mut output_len as *mut u32)
        };
        if result != 1 {
            return Err(openssl_last_err());
        }
        output.truncate(output_len as usize);
        Ok(output)
    }
}

/// Translate a [`keymint::Digest`] into a raw [`ffi::EVD_MD`].
fn digest_into_openssl_ffi(digest: Digest) -> Result<*const ffi::EVP_MD, Error> {
    unsafe {
        // Safety: all of the `EVP_<digest>` functions return a non-null result.
        match digest {
            Digest::Md5 => Ok(ffi::EVP_md5()),
            Digest::Sha1 => Ok(ffi::EVP_sha1()),
            Digest::Sha224 => Ok(ffi::EVP_sha224()),
            Digest::Sha256 => Ok(ffi::EVP_sha256()),
            Digest::Sha384 => Ok(ffi::EVP_sha384()),
            Digest::Sha512 => Ok(ffi::EVP_sha512()),
            d => Err(km_err!(UnsupportedDigest, "unknown digest {:?}", d)),
        }
    }
}
