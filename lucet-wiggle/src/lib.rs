pub use lucet_wiggle_generate::bindings;
pub use lucet_wiggle_macro::from_witx;
pub use wiggle::{
    witx, BorrowChecker, GuestError, GuestErrorType, GuestMemory, GuestPtr, GuestSlice, GuestStr,
    GuestType, GuestTypeTransparent, Pointee,
};

pub mod generate {
    pub use lucet_wiggle_generate::*;
}

pub mod runtime {
    use lucet_runtime::vmctx::Vmctx;
    use wiggle::GuestMemory;

    pub struct LucetMemory<'a> {
        vmctx: &'a Vmctx,
    }

    impl<'a> LucetMemory<'a> {
        pub fn new(vmctx: &'a Vmctx) -> LucetMemory {
            LucetMemory { vmctx }
        }
    }

    unsafe impl<'a> GuestMemory for LucetMemory<'a> {
        fn base(&self) -> (*mut u8, u32) {
            let mem = self.vmctx.heap_mut();
            let len = mem.len() as u32;
            let ptr = mem.as_ptr();
            (ptr as *mut u8, len)
        }
    }
}
