
use libtest::{TestRng, dbg, print, println};
use crate::misc::Alignment;
use core::num::NonZero;

use crate::AlignedNonNull;

use crate::extent_alloc::extent::{Extent, MutableExtent, RemainingExtents};
use crate::extent_alloc::helpers::{AllocMap, PartExtractionError, SuitableExtent};
use crate::extent_alloc::{
    extent::{TakenExtent, InternalExtent, RawExtent},
    layout::{ExtentLayout, LayoutDescriptor},
    raw_alloc::RawExtentAlloc,
};

const MIN_EXT_COUNT: usize = 128;
const MAX_EXT_COUNT: usize = 256;



use alloc::vec::Vec;


#[test]
fn find_suitable_extent() {

    let mut rand = TestRng::new();

    for _ in 0..500 {

        let count = rand.next_range(256..512);

        let exts = random_extents::<2048>(count, &mut rand);

        let mut alloc = RawExtentAlloc::uninit();
        let init_result = unsafe { alloc.initialize(exts.iter().cloned()) };
        assert!(matches!(init_result, Ok(())));

        let (free, _) = distinguish_free_and_used(exts);



        for _ in 0..128 {

            let layout = ExtentLayout::from_pages(NonZero::new(rand.next_range(512..4096) as u64).unwrap()).unwrap();

            //  find any suitable extent
            match alloc.free_map().find_suitable(layout.clone()) {
                Some(SuitableExtent { suitable, allocated }) => {

                    assert!(allocated.fits_into(&suitable));

                    assert!(allocated.address().alignment() >= layout.align());
                    assert!(allocated.size() >= layout.size());

                    let was_found = free.iter().find(|ext| ext.address() == suitable.address() && ext.size() == suitable.size() );

                    assert!(matches!(was_found, Some(_)), "suitable extent was found manually but was not found by the tested function");
                },
                None => {   //  assert that any valid extent does not exist

                    let was_found = free.iter().find(|suitable| {

                        //  align the address up to `layout.align()` to figure out where would the extent start
                        let aligned_up = /*unsafe*/ {
                            //  safety: align_up() always return a number larger than the input
                            NonZero::new/*_unchecked*/(suitable.address().get().align_up(layout.align().get() as usize)).unwrap()
                        };

                        let allocated = InternalExtent::new(AlignedNonNull::new(aligned_up).unwrap(), layout.size());

                        allocated.fits_into(&suitable)
                    } );

                    assert!(matches!(was_found, None));
                }
            }
        }
    }

}


#[test]
fn fragmented_insertion() {

    let mut rand = TestRng::new();

    for _ in 0..1000 {

        let count = rand.next_range(256..512);
        let exts = random_extents::<2048>(count, &mut rand);

        let mut allocator = RawExtentAlloc::uninit();
        let init_result = unsafe { allocator.initialize(exts.iter().cloned()) };
        assert!(matches!(init_result, Ok(())));

        let (mut free, mut used) = distinguish_free_and_used(exts);

        for _ in 0..200 {

            if used.is_empty() { break }

            let idx = rand.next_range(..used.len());
            let to_insert = used.remove(idx);
            free.push(to_insert.clone());

            let free_map = unsafe { allocator.free_map_mut() };
            free_map.insert_extent_fragmented(&MutableExtent::from_regular(to_insert))
                .expect("insertion failed");

            verify_map(&free, /*used_map*/free_map);
        }

    }

}

#[test]
fn extract_exact() {

    let mut rand = TestRng::new();

    for _ in 0..1000 {

        let count = rand.next_range(256..512);
        let exts = random_extents::<2048>(count, &mut rand);

        let mut allocator = RawExtentAlloc::uninit();
        let init_result = unsafe { allocator.initialize(exts.iter().cloned()) };
        assert!(matches!(init_result, Ok(())));

        let (mut free, /*mut used*/ _) = distinguish_free_and_used(exts);

        for _ in 0..150 {

            let idx = rand.next_range(..free.len());

            let extent = MutableExtent::from_regular(free.remove(idx));

            unsafe { allocator.free_map_mut() }.extract_exact_extent(extent.clone())
            .expect("extraction failed");

            verify_map(&free, allocator.free_map());

            //  the extent is no longer present in the map
            let result = unsafe { allocator.free_map_mut() }.extract_exact_extent(extent.clone());
            assert!(matches!(result, Err(())));

            if free.is_empty() { break }
        }

    }
}


#[ignore = "TODO"]
#[test]
fn extract_extent_part() {

    let mut rand = TestRng::new();

    for _ in 0..1000 {

        let count = 10;//rand.next_range(256..512);
        let exts = random_extents::<2048>(count, &mut rand);

        let mut allocator = RawExtentAlloc::uninit();
        let init_result = unsafe { allocator.initialize(exts.iter().cloned()) };
        assert!(matches!(init_result, Ok(())));

        let (mut free, used) = distinguish_free_and_used(exts);


        for _ in 0..150 {

            let idx = rand.next_range(..free.len());

            println!("before: {:?}\n{:?}\n\n\n", free, allocator.free_map());

            let big = MutableExtent::from_regular(free.remove(idx));

            let small: MutableExtent<1024> = {
                let address = rand.next_range(big.address().get() as usize..big.end_address().get() as usize+1024) as u64;
                let address = AlignedNonNull::new_down(NonZero::new(address).unwrap()).unwrap();

                let size = rand.next_range(1..(big.end_address().get().saturating_sub(address.get()) / 1024) as usize) as u64;
                let size = NonZero::new(size).unwrap();

                MutableExtent::new(address, size)
            };

            match unsafe { allocator.free_map_mut() }.extract_extent_part(big.clone(), small.clone()) {
                Ok(_) => assert!(small.fits_into(&big)),
                Err(e) => {
                    assert!(!small.fits_into(&big));
                    assert!(matches!(e, PartExtractionError::PartDoesNotFit));
                    continue;
                }
            }

            let RemainingExtents { left: front, right: remainder } = match small.remove_from(big) {
                Ok(rem) => rem,
                Err(_) => panic!("failed to remove small from big"),
            };

            if let Some(front) = front {
                free.push(front.into_regular());
            }

            if let Some(rem) = remainder {
                free.push(rem.into_regular());
            }

            println!("after: {:?}\n{:?}\n\n\n", free, allocator.free_map());

            verify_map(&free, allocator.free_map());


            todo!();

            if free.is_empty() { break }

        }

    }

}



/*#[ignore = "todo"]
#[test]
fn extract_from() {

    let mut rand = TestRng::new();

    for _ in 0..500 {

        let count = rand.next_range(MIN_EXT_COUNT..MAX_EXT_COUNT);
        let exts = random_extents(count, &mut rand);

        let allocator: CachedExtentAllocator<1024> = CachedExtentAllocator::uninit();
        assert!(matches!(unsafe { allocator.initialize(exts.iter().cloned()) }, Ok(()) ));

        let (mut free, mut used) = distinguish_free_and_used(exts);

        let mut alloc = allocator.map.lock();

        for _ in 0..150 {
            let idx = rand.next_range(..free.len());

            let to_extract = free.remove(idx);

            if to_extract.size().get() == 1 { continue; }

            let layout = {
                let count = NonZero::new(to_extract.size().get() / 2).unwrap();
                ExtentLayout::from_pages(count).expect("failed to construct ExtentLayout")
            };

            insert_extent_to_fragmented(&to_extract, &mut alloc.used)
                .expect("insert_extent_to_fragmented() returned Err");

            used.push(to_extract.clone());

            verify_map(&used, &alloc.used);

            let extracted = extract_extent_from(MutableExtent::from(to_extract.clone()), layout, &mut alloc.free)
                .expect("extract_exact_extent_from() returned Err");

            let put_back = {

                let addr = to_extract.address().aligned_add(extracted.size().get() as usize);
                let size = NonZero::new(to_extract.size().get() - extracted.size().get()).unwrap();
                RawExtent::new(addr, size)
            };

            free.push(put_back);

            println!("vec: {}", free.len());
            println!("map: {}:{}", alloc.free.map.len(),
                alloc.free.size_index.iter().map(|(_, by_align)| by_align.iter().map(|(_, addr)| addr.len() ).sum::<usize>() ).sum::<usize>()
            );

            verify_map(&free, &alloc.free);
            //verify_map(&used, &alloc.used);


            if free.is_empty() { break }

        }
    }

}*/





/// Produces `(free, used)` from the `Vec<TakenExtent<..>>`
pub(crate) fn distinguish_free_and_used(exts: Vec<TakenExtent<1024, InternalExtent<1024>>>) -> (Vec<InternalExtent<1024>>, Vec<InternalExtent<1024>>) {
    let mut free = Vec::new();
    let mut used = Vec::new();

    for ext in exts {
        match ext {
            TakenExtent::Free(ext) => free.push(ext),
            TakenExtent::Used(ext) => used.push(ext),
        }
    }

    (free, used)
}

/// Verifies the map agains the vector
#[track_caller]
pub(crate) fn verify_map(vec: &Vec<InternalExtent<1024>>, map: &AllocMap<1024>) {

    let mut size_index_count: usize = 0;


    //  verify the map against the vector
    assert!(vec.len() == map.map.len());

    for ext in vec.iter() {
        match map.map.get(&ext.address()) {
            Some(size) => assert!(ext.size() == *size),
            None => panic!("an extent is not present in the map"),
        }
    }


    //  verify the size_index against the map
    for (size, by_align) in map.size_index.iter() {
        for (_, addresses) in by_align.iter() {
            size_index_count += addresses.len();

            for address in addresses.iter() {
                match map.map.get(address) {
                    Some(map_size) => assert!(*map_size == *size),
                    None => panic!("size_index contains address that is not registered in the map"),
                }
            }
        }
    }

    assert!(size_index_count == map.map.len());
}








/// Generates random `Vec<TakenExtent<>` filled with `Free` non-overlapping entries
///
/// This function uses a counter for the address, so the extents are naturally sorted based on address
pub(crate) fn random_free_extents<const MAX_EXT_COUNT: usize>(count: usize, rand: &mut TestRng) -> Vec<TakenExtent<1024, InternalExtent<1024>>> {

    let mut vec = Vec::with_capacity(count);
    let mut address = NonZero::new(1).unwrap();

    for _ in 0..count {
        vec.push(TakenExtent::Free(random_ext::<MAX_EXT_COUNT>(rand, &mut address)));
    }

    vec
}



/// Generates a vector of random extents
///
/// This function uses a counter for the address, so the extents are naturally sorted based on address
pub(crate) fn random_extents<const MAX_EXT_SIZE: usize>(count: usize, rand: &mut TestRng) -> Vec<TakenExtent<1024, InternalExtent<1024>>> {

    let mut vec = Vec::with_capacity(count);
    let mut address = NonZero::new(1024).unwrap();

    for _ in 0..count {
        if rand.next() & 1 != 0 {
            vec.push(TakenExtent::Used(random_ext::<MAX_EXT_COUNT>(rand, &mut address)));
        } else {
            vec.push(TakenExtent::Free(random_ext::<MAX_EXT_COUNT>(rand, &mut address)));
        }
    }

    vec
}

pub(crate) fn random_ext<const MAX_EXT_SIZE: usize>(rand: &mut TestRng, addr: &mut NonZero<u64>) -> InternalExtent<1024> {
    let pages = rand.next_range(2..MAX_EXT_SIZE) as u64;
    let ret = InternalExtent::new(
        AlignedNonNull::new_up(*addr),
        NonZero::new(pages).unwrap());
    *addr = addr.saturating_add(pages.saturating_mul(1024));
    ret
}
