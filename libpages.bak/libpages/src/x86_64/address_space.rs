use core::cell::UnsafeCell;

use crate::levels::Pml4Entry;

use liblock::helpers::Lock;


/// Virtual address space - contains the PML4 table and some metadata
#[repr(C, align(4096))]
pub struct AddressSpace {
    /// Part of the address space belonging to the userspace
    /// - Locked by external lock
    user: UnsafeCell<[Pml4Entry; 256]>,
    /// Part of the address space belonging to the kernel
    /// - Locked by external lock
    kernel: UnsafeCell<[Pml4Entry; 256]>,

    /// A raw lock that locks the userspace entries
    user_lock: Lock,

    /// A raw Lock that locks the kernel entries
    kernel_lock: Lock,

    //  metadata: pre-allocated spaces, quick-maps
}
