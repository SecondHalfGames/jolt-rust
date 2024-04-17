fn main() {
    unsafe {
        jolt_sys::JPC_RegisterDefaultAllocator();
        jolt_sys::JPC_FactoryInit();
    }
}
