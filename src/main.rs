
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]
#![test_runner(class_Os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box,vec, vec::Vec, rc::Rc};
use class_Os::println;
use x86_64::structures::paging::frame;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use class_Os::allocator;
    use class_Os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};
    

    println!("Hello World{}", "!");
    class_Os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500{
        vec.push(i);
    }

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    class_Os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    class_Os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    class_Os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


