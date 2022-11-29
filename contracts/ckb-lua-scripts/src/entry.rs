use core::result::Result;
use ckb_lib_lua::LibCKBLua;
use ckb_std::{
    debug,
    high_level::{load_script, load_witness_args},
    ckb_types::{bytes::Bytes, prelude::*}, dynamic_loading_c_impl::CKBDLContext, ckb_constants::Source,
};

use crate::{error::Error, helper::{blake2b_160}};

// lua_code_hash_len(20bytes) + extra_parameter_len(8bytes)
// extra_parameter: price = udt/ckb * 10^8. 
// For example: If the price of udt/ckb is 2.5, the extra_parameter will be (int)(2.5*10^8).
const ARGS_LEN: usize = 28;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    if args.len() != ARGS_LEN {
        return Err(Error::ScriptArgsInvalid);
    }

    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let witness_lock: Bytes = witness_args
            .lock()
            .to_opt()
            .ok_or(Error::WitnessArgsParseError)?
            .unpack();
    if witness_lock.is_empty() {
        return Err(Error::WitnessArgsParseError);
    }
    let lua_code_hash = blake2b_160(&witness_lock);
    if &lua_code_hash != &args[0..20] {
        return Err(Error::LuaCodeHashError);
    }
    let mut price_bytes = [0u8; 8];
    price_bytes.copy_from_slice(&args[20..]);
    let price = u64::from_be_bytes(price_bytes);

    let mut context = unsafe { CKBDLContext::<[u8; 1024 * 1024]>::new() };
    let lib_lua = LibCKBLua::load(&mut context);

    lib_lua.run_lua_code(&witness_lock).map_err(|e| {
        debug!("lua error: {}", e);
        Error::LuaScriptExecuteError
    })?;

    Ok(())
}

