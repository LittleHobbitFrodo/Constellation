
use core::ops::{Shl, Shr};

use libmem::AlignedAddress;
use libtest::{TestRng, print, println, dbg};

use crate::{levels::TableIndexer, virt_addr::VirtualAddress};


/// Generates a index addition test (ex. `VirtualAddress::add_pt()`)
macro_rules! generate_add_test {
    ($test_name:ident, $shift:expr, $func:ident, $max_rand:expr) => {

        #[test]
        fn $test_name() {
            let mut rand = TestRng::new();

            for _ in 0..10_000 {

                let mut address = VirtualAddress::from_parts(rand.next() & 1 == 0, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16);

                let original = address;

                let added = rand.next_range(..$max_rand);

                let expected = {
                    let orig = original.as_ptr() as u64;

                    let raw = orig.shl(16i32).saturating_add((added as u64).shl(16 + $shift)).shr(16)
                        | (VirtualAddress::HIGH_BITS * original.is_kernel_address() as u64);

                    VirtualAddress::from(raw as *const u8)
                };

                address.$func(added);

                assert!(address.is_kernel_address() == original.is_kernel_address());
                assert!(address.offset() == expected.offset());
                assert!(address.pt_index().as_u16() == expected.pt_index().as_u16());
                assert!(address.pd_index().as_u16() == expected.pd_index().as_u16());
                assert!(address.pdpt_index().as_u16() == expected.pdpt_index().as_u16());
                assert!(address.pml4_index().into_u16() == expected.pml4_index().into_u16());
            }
        }

    };
}

generate_add_test!(add_offset, 0, add_offset, u32::MAX as usize /2);
generate_add_test!(add_pt, VirtualAddress::PT_SHIFT, add_pt, u32::MAX as usize /2);
generate_add_test!(add_pd, VirtualAddress::PD_SHIFT, add_pd, u16::MAX as usize + 8);
generate_add_test!(add_pdpt, VirtualAddress::PDPT_SHIFT, add_pdpt, u16::MAX as usize + 8);
generate_add_test!(add_pml4, VirtualAddress::PML4_SHIFT, add_pml4, u16::MAX as usize);





/// Generates a index subtraction test (ex. `VirtualAddress::sub_pt()`)
macro_rules! generate_sub_test {
    ($test_name:ident, $shift:expr, $func:ident, $max_rand:expr) => {

        #[test]
        fn $test_name() {
            let mut rand = TestRng::new();

            for _ in 0..10_000 {

                let mut address = VirtualAddress::from_parts(rand.next() & 1 == 0, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16);

                let original = address;

                let subtracted = rand.next_range(..$max_rand);

                let expected = {
                    let orig = original.as_ptr() as u64;

                    let sub = subtracted.shl($shift as u32) as u64;

                    let raw = (orig & !VirtualAddress::HIGH_BITS).saturating_sub(sub)
                        | (VirtualAddress::HIGH_BITS * original.is_kernel_address() as u64);
                    VirtualAddress::from(raw as *const u8)
                };

                address.$func(subtracted);


                assert!(address.is_kernel_address() == original.is_kernel_address());
                assert!(address.offset() == expected.offset());
                assert!(address.pt_index().as_u16() == expected.pt_index().as_u16());
                assert!(address.pd_index().as_u16() == expected.pd_index().as_u16());
                assert!(address.pdpt_index().as_u16() == expected.pdpt_index().as_u16());
                assert!(address.pml4_index().into_u16() == expected.pml4_index().into_u16());
            }
        }

    };
}


generate_sub_test!(sub_offset, 0, sub_offset, u32::MAX as usize / 2);
generate_sub_test!(sub_pt, VirtualAddress::PT_SHIFT, sub_pt, u32::MAX as usize / 2);
generate_sub_test!(sub_pd, VirtualAddress::PD_SHIFT, sub_pd, u16::MAX as usize + 8);
generate_sub_test!(sub_pdpt, VirtualAddress::PDPT_SHIFT, sub_pdpt, u16::MAX as usize + 8);
generate_sub_test!(sub_pml4, VirtualAddress::PML4_SHIFT, sub_pml4, u16::MAX as usize);






/// Tests direct index manipulation
macro_rules! generate_set_test {
    ($test_name:ident, $shift:expr, $mask:expr, $func:ident) => {

        #[test]
        fn $test_name() {
            let mut rand = TestRng::new();

            for _ in 0..10_000 {

                let mut address = VirtualAddress::from_parts(rand.next() & 1 == 0, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16);

                let original = address;

                let new = rand.next_range(..$mask as usize) as u16;

                let expected = {
                    let orig = original.as_ptr() as u64;

                    let raw = (orig & !($mask << $shift))
                        | ((new as u64 & $mask) << $shift);
                    VirtualAddress::from(raw as *const u8)
                };

                println!("new:  {new}");
                println!("addr: {address:?}");
                println!("exp:  {expected:?}");

                address.$func(new);

                assert!(address.is_kernel_address() == original.is_kernel_address());
                assert!(address.offset() == expected.offset());
                assert!(address.pt_index().as_u16() == expected.pt_index().as_u16());
                assert!(address.pd_index().as_u16() == expected.pd_index().as_u16());
                assert!(address.pdpt_index().as_u16() == expected.pdpt_index().as_u16());
                assert!(address.pml4_index().into_u16() == expected.pml4_index().into_u16());
            }
        }
    };
}


generate_set_test!(set_offset, 0, VirtualAddress::OFFSET_MASK, set_offset);
generate_set_test!(set_pt, VirtualAddress::PT_SHIFT, VirtualAddress::NINE_BIT_MASK, set_pt);
generate_set_test!(set_pd, VirtualAddress::PD_SHIFT, VirtualAddress::NINE_BIT_MASK, set_pd);
generate_set_test!(set_pdpt, VirtualAddress::PDPT_SHIFT, VirtualAddress::NINE_BIT_MASK, set_pdpt);
generate_set_test!(set_pml4, VirtualAddress::PML4_SHIFT, VirtualAddress::NINE_BIT_MASK, set_pml4);


#[test]
fn make_kernel() {
    let mut rand = TestRng::new();

    for _ in 0..10_000 {

        let mut address = VirtualAddress::from_parts(false, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16);

        let original = address;

        address.make_kernel();

        assert!(address.is_kernel_address() == true);
        assert!(address.offset() == original.offset());
        assert!(address.pt_index().as_u16() == original.pt_index().as_u16());
        assert!(address.pd_index().as_u16() == original.pd_index().as_u16());
        assert!(address.pdpt_index().as_u16() == original.pdpt_index().as_u16());
        assert!(address.pml4_index().into_u16() == original.pml4_index().into_u16());
    }
}


#[test]
fn make_user() {
    let mut rand = TestRng::new();

    for _ in 0..10_000 {

        let mut address = VirtualAddress::from_parts(false, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16, rand.next() as u16);

        let original = address;

        address.make_user();

        assert!(address.is_kernel_address() == false);
        assert!(address.offset() == original.offset());
        assert!(address.pt_index().as_u16() == original.pt_index().as_u16());
        assert!(address.pd_index().as_u16() == original.pd_index().as_u16());
        assert!(address.pdpt_index().as_u16() == original.pdpt_index().as_u16());
        assert!(address.pml4_index().into_u16() == original.pml4_index().into_u16());
    }
}
