pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    None,
    Timeout,
    PipeClosed,
    SysCallNotImplement,
    Common(String),
    CreateMMap(String),
    UnallignedAddress,
    UnallignedSize,
    NoEnoughMemory,
    AddressNotInRange,
    RootPageIdxNoExist,
    IOError(String),
    NoEnoughSpace,
    RangeUnavailable,
    Overflow,
    WrongELFFormat,
    ELFLoadError(&'static str),
    InterpreterFileErr,
    MMampError,
    UnmatchRegion,
    AddressDoesMatch,
    UnknownDevice,
    Locked,
    ZeroCount,
    QueueFull,
    NoData,
    NoUringReq,
    NoneIdx,
    AddressNotMap(u64),
    InvalidInput,
    NotExist,
    //todo handle this.
    ErrClosedForReceive,
    ErrConnectionReset,
    SysError(i32),
    FileMapError,

    // ERESTARTSYS is returned by an interrupted syscall to indicate that it
    // should be converted to EINTR if interrupted by a signal delivered to a
    // user handler without SA_RESTART set, and restarted otherwise.
    ERESTARTSYS,
    // ERESTARTNOINTR is returned by an interrupted syscall to indicate that it
    // should always be restarted.
    ERESTARTNOINTR,
    // ERESTARTNOHAND is returned by an interrupted syscall to indicate that it
    // should be converted to EINTR if interrupted by a signal delivered to a
    // user handler, and restarted otherwise.
    ERESTARTNOHAND,
    // ERESTART_RESTARTBLOCK is returned by an interrupted syscall to indicate
    // that it should be restarted using a custom function. The interrupted
    // syscall must register a custom restart function by calling
    // Task.SetRestartSyscallFn.
    ERESTARTRESTARTBLOCK,
    // ErrWouldBlock is an internal error used to indicate that an operation
    // cannot be satisfied immediately, and should be retried at a later
    // time, possibly when the caller has received a notification that the
    // operation may be able to complete. It is used by implementations of
    // the kio.File interface.
    ErrWouldBlock,
    //request was interrupted
    ErrInterrupted,
    // ErrExceedsFileSizeLimit is returned if a request would exceed the
    // file's size limit.
    ErrExceedsFileSizeLimit,

    // ErrNoWaitableEvent is returned by non-blocking Task.Waits (e.g.
    // waitpid(WNOHANG)) that find no waitable events, but determine that waitable
    // events may exist in the future. (In contrast, if a non-blocking or blocking
    // Wait determines that there are no tasks that can produce a waitable event,
    // Task.Wait returns ECHILD.)
    ErrNoWaitableEvent,

    Unimplemented(String),

    SerdeJson(String),
    ProtobufError(String),

    InvalidArgument(String),
    ContainerdShim(String),

    NotSupport,
}

impl Error {
    pub fn SystemErr(err: i32) -> Self {
        return Self::SysError(err);
    }

    pub fn Message(e: String) -> Self {
        return Self::Common(e);
    }

    pub fn MapRes(res: i32) -> Result<()> {
        if res == 0 {
            return Ok(());
        }

        if res < 0 {
            return Err(Error::SysError(-res));
        }

        panic!("MapRes get res {}", res);
    }
}

impl Default for Error {
    fn default() -> Self {
        Error::None
    }
}
