type Id = u32;
struct Handle(Id);

// destructor
impl Drop for Handle {
    fn drop(&mut self) {
        println!("handle {} is dropped!", self.0)
    }
}

pub fn scope() {
    let handle_0 = Handle(0);
    let _ = Handle(1);
    let handle_2 = create_handle();
    Handle(4);
    // order of dropped is 1, 2, 4, 3, 0
    // _ -> mean ignore this value hence Handle 1 is dropped immediately
    // Handle 2 is dropped next as create function did not return Handle 2 (mean out of scope)
    // Handle 4 is dropped as it is not assigned to any variable (no ownership out of scope)
    // Handle 3 is dropped before Handle 0 as Handle 3 is added to the stack later than Handle 0
}

fn create_handle() -> Handle {
    Handle(2);
    Handle(3) // note Handle 3 is being returned only hence it will stay in scope longer than Handle 1
}
