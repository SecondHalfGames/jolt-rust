fn main() {
    unsafe {
        jolt_sys::JPC_RegisterDefaultAllocator();
        jolt_sys::JPC_FactoryInit();
        jolt_sys::JPC_RegisterTypes();

        let temp_allocator = jolt_sys::JPC_TempAllocatorImpl_new(10 * 1024 * 1024);

        jolt_sys::JPC_TempAllocatorImpl_delete(temp_allocator);
    }

    println!("Hello, world!");
}
