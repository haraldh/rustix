//! The `rustix` `Error` type.
//!
//! This type holds an OS error code, which conceptually corresponds to an
//! `errno` value.
#![allow(unsafe_code)]
#![allow(missing_docs)]

use super::super::wasi_filesystem;
use crate::imp::fd::RawFd;

/// The error type for `rustix` APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Error(wasi_filesystem::Errno);

impl Error {
    /// Extract the raw OS error number from this error.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_io_error(io_err: &std::io::Error) -> Option<Self> {
        let bits = io_err.raw_os_error()?;
        Some(Self::from_raw_os_error(bits))
    }

    /// Extract the OS error number from this error.
    #[inline]
    pub(crate) const fn os_error(self) -> wasi_filesystem::Errno {
        self.0
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        self.0 as i32
    }

    /// Construct an `Error` from a raw OS error number.
    #[inline]
    pub fn from_raw_os_error(raw: i32) -> Self {
        todo!("from_raw_os_error")
    }
}

impl Error {
    pub const ACCES: Self = Self(wasi_filesystem::Errno::Access as _);
    pub const ADDRINUSE: Self = Self(wasi_filesystem::Errno::Addrinuse as _);
    pub const ADDRNOTAVAIL: Self = Self(wasi_filesystem::Errno::Addrnotavail as _);
    pub const AFNOSUPPORT: Self = Self(wasi_filesystem::Errno::Afnosupport as _);
    pub const AGAIN: Self = Self(wasi_filesystem::Errno::Again as _);
    pub const ALREADY: Self = Self(wasi_filesystem::Errno::Already as _);
    pub const BADMSG: Self = Self(wasi_filesystem::Errno::Badmsg as _);
    pub const BUSY: Self = Self(wasi_filesystem::Errno::Busy as _);
    pub const CANCELED: Self = Self(wasi_filesystem::Errno::Canceled as _);
    pub const CHILD: Self = Self(wasi_filesystem::Errno::Child as _);
    pub const CONNABORTED: Self = Self(wasi_filesystem::Errno::Connaborted as _);
    pub const CONNREFUSED: Self = Self(wasi_filesystem::Errno::Connrefused as _);
    pub const CONNRESET: Self = Self(wasi_filesystem::Errno::Connreset as _);
    pub const DEADLK: Self = Self(wasi_filesystem::Errno::Deadlk as _);
    pub const DESTADDRREQ: Self = Self(wasi_filesystem::Errno::Destaddrreq as _);
    pub const DOM: Self = Self(wasi_filesystem::Errno::Dom as _);
    pub const DQUOT: Self = Self(wasi_filesystem::Errno::Dquot as _);
    pub const EXIST: Self = Self(wasi_filesystem::Errno::Exist as _);
    pub const FAULT: Self = Self(wasi_filesystem::Errno::Fault as _);
    pub const FBIG: Self = Self(wasi_filesystem::Errno::Fbig as _);
    pub const HOSTUNREACH: Self = Self(wasi_filesystem::Errno::Hostunreach as _);
    pub const IDRM: Self = Self(wasi_filesystem::Errno::Idrm as _);
    pub const ILSEQ: Self = Self(wasi_filesystem::Errno::Ilseq as _);
    pub const INTR: Self = Self(wasi_filesystem::Errno::Intr as _);
    pub const INVAL: Self = Self(wasi_filesystem::Errno::Inval as _);
    pub const INPROGRESS: Self = Self(wasi_filesystem::Errno::Inprogress as _);
    pub const IO: Self = Self(wasi_filesystem::Errno::Io as _);
    pub const ISCONN: Self = Self(wasi_filesystem::Errno::Isconn as _);
    pub const ISDIR: Self = Self(wasi_filesystem::Errno::Isdir as _);
    pub const LOOP: Self = Self(wasi_filesystem::Errno::Loop as _);
    pub const MFILE: Self = Self(wasi_filesystem::Errno::Mfile as _);
    pub const MLINK: Self = Self(wasi_filesystem::Errno::Mlink as _);
    pub const MSGSIZE: Self = Self(wasi_filesystem::Errno::Msgsize as _);
    pub const MULTIHOP: Self = Self(wasi_filesystem::Errno::Multihop as _);
    pub const NAMETOOLONG: Self = Self(wasi_filesystem::Errno::Nametoolong as _);
    pub const NETDOWN: Self = Self(wasi_filesystem::Errno::Netdown as _);
    pub const NETUNREACH: Self = Self(wasi_filesystem::Errno::Netunreach as _);
    pub const NETRESET: Self = Self(wasi_filesystem::Errno::Netreset as _);
    pub const NFILE: Self = Self(wasi_filesystem::Errno::Nfile as _);
    pub const NOBUFS: Self = Self(wasi_filesystem::Errno::Nobufs as _);
    pub const NODEV: Self = Self(wasi_filesystem::Errno::Nodev as _);
    pub const NOENT: Self = Self(wasi_filesystem::Errno::Noent as _);
    pub const NOEXEC: Self = Self(wasi_filesystem::Errno::Noexec as _);
    pub const NOLCK: Self = Self(wasi_filesystem::Errno::Nolck as _);
    pub const NOLINK: Self = Self(wasi_filesystem::Errno::Nolink as _);
    pub const NOMEM: Self = Self(wasi_filesystem::Errno::Nomem as _);
    pub const NOMSG: Self = Self(wasi_filesystem::Errno::Nomsg as _);
    pub const NOPROTOOPT: Self = Self(wasi_filesystem::Errno::Noprotoopt as _);
    pub const NOSPC: Self = Self(wasi_filesystem::Errno::Nospc as _);
    pub const NOSYS: Self = Self(wasi_filesystem::Errno::Nosys as _);
    pub const NOTCONN: Self = Self(wasi_filesystem::Errno::Notconn as _);
    pub const NOTDIR: Self = Self(wasi_filesystem::Errno::Notdir as _);
    pub const NOTEMPTY: Self = Self(wasi_filesystem::Errno::Notempty as _);
    pub const NOTRECOVERABLE: Self = Self(wasi_filesystem::Errno::Notrecoverable as _);
    pub const NOTSOCK: Self = Self(wasi_filesystem::Errno::Notsock as _);
    pub const NOTSUP: Self = Self(wasi_filesystem::Errno::Notsup as _);
    pub const NOTTY: Self = Self(wasi_filesystem::Errno::Notty as _);
    pub const NXIO: Self = Self(wasi_filesystem::Errno::Nxio as _);
    // On WASI, `EOPNOTSUPP` has the same value as `ENOTSUP`.
    pub const OPNOTSUPP: Self = Self(wasi_filesystem::Errno::Notsup as _);
    pub const OVERFLOW: Self = Self(wasi_filesystem::Errno::Overflow as _);
    pub const OWNERDEAD: Self = Self(wasi_filesystem::Errno::Ownerdead as _);
    pub const PERM: Self = Self(wasi_filesystem::Errno::Perm as _);
    pub const PIPE: Self = Self(wasi_filesystem::Errno::Pipe as _);
    pub const PROTO: Self = Self(wasi_filesystem::Errno::Proto as _);
    pub const PROTONOSUPPORT: Self = Self(wasi_filesystem::Errno::Protonosupport as _);
    pub const PROTOTYPE: Self = Self(wasi_filesystem::Errno::Prototype as _);
    pub const RANGE: Self = Self(wasi_filesystem::Errno::Range as _);
    pub const ROFS: Self = Self(wasi_filesystem::Errno::Rofs as _);
    pub const SPIPE: Self = Self(wasi_filesystem::Errno::Spipe as _);
    pub const SRCH: Self = Self(wasi_filesystem::Errno::Srch as _);
    pub const STALE: Self = Self(wasi_filesystem::Errno::Stale as _);
    pub const TIMEDOUT: Self = Self(wasi_filesystem::Errno::Timedout as _);
    pub const TOOBIG: Self = Self(wasi_filesystem::Errno::Toobig as _);
    pub const TXTBSY: Self = Self(wasi_filesystem::Errno::Txtbsy as _);
    pub const XDEV: Self = Self(wasi_filesystem::Errno::Xdev as _);
}

impl From<wasi_filesystem::Errno> for Error {
    #[inline]
    fn from(os: wasi_filesystem::Errno) -> Self {
        Self(os)
    }
}
