#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::Parameters, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn hello_world() {
    // Doesn't advance RNG of the runtime
    runtime::ret(CLValue::from_t("Hello, world!").unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn call() {

    let entry_points = {
        let mut entry_points = EntryPoints::new();

        let entry_point = EntryPoint::new(
            "hello_world",
            Parameters::default(),
            CLType::String,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(entry_point);


        entry_points
    };
    let (contract_hash, _contract_version) = storage::new_contract(entry_points, None, None, None);


    let result: String =
        runtime::call_contract(contract_hash, "hello_world", RuntimeArgs::default());

    // let uref2: URef = storage::new_uref(U512::from(1));
    let uref2:URef = storage::new_uref(String::from(result));
    runtime::put_key("result", Key::URef(uref2));
}
