fn main() {
    unsafe {
        jolt_sys::JPC_RegisterDefaultAllocator();
        jolt_sys::JPC_FactoryInit();
        jolt_sys::JPC_RegisterTypes();

        let temp_allocator = jolt_sys::JPC_TempAllocatorImpl_new(10 * 1024 * 1024);

        let job_system = jolt_sys::JPC_JobSystemThreadPool_new2(
            jolt_sys::JPC_MAX_PHYSICS_JOBS as _,
            jolt_sys::JPC_MAX_PHYSICS_BARRIERS as _,
        );

        let physics_system = jolt_sys::JPC_PhysicsSystem_new();

        jolt_sys::JPC_PhysicsSystem_delete(physics_system);
        jolt_sys::JPC_JobSystemThreadPool_delete(job_system);
        jolt_sys::JPC_TempAllocatorImpl_delete(temp_allocator);
    }

    println!("Hello, world!");
}
