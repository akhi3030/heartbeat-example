use std::cell::RefCell;

thread_local! {
    static GLOBAL: RefCell<u32> = RefCell::new(0);
}

#[ic_cdk_macros::query]
fn print() {
    GLOBAL.with(|global_ref| {
        let global = global_ref.borrow();
        ic_cdk::print(format!("global is {}", global));
    })
}

fn increment() {
    GLOBAL.with(|global_ref| {
        let mut global = global_ref.borrow_mut();
        *global = *global + 1;
    })
}

#[export_name = "canister_heartbeat"]
fn heartbeat() {
    increment();
}
