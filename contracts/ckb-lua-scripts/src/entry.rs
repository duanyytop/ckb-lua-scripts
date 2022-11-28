use core::result::Result;
use ckb_lib_lua::LibCKBLua;
use ckb_std::{
    debug,
    high_level::{load_script},
    ckb_types::{bytes::Bytes, prelude::*}, dynamic_loading_c_impl::CKBDLContext,
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    let lua_code: &[u8] = b"ckb.exit_script(32)";

    let mut context = unsafe { CKBDLContext::<[u8; 1024 * 1024]>::new() };
    let lib_lua = LibCKBLua::load(&mut context);

    lib_lua.run_lua_code(lua_code).map_err(|e| {
        debug!("lua error: {}", e);
        Error::MyError
    })?;

    Ok(())
}

