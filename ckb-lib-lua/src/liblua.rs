use crate::code_hashes::CODE_HASH_LUA;
use ckb_std::dynamic_loading_c_impl::{CKBDLContext, Symbol};
use ckb_std::{
    debug,
};

const RUN_LUA_CODE: &[u8; 12] = b"run_lua_code";
type RunLuaCode = unsafe extern "C" fn(
    code: *const u8,
    code_size: u32,
) -> isize;

pub struct LibCKBLua {
    run_lua_code: Symbol<RunLuaCode>,
}

impl LibCKBLua {
    pub fn load<T>(context: &mut CKBDLContext<T>) -> Self {
        let lib = context
            .load(&CODE_HASH_LUA)
            .expect("load ckb_lib_lua");

        let run_lua_code = unsafe { lib.get(RUN_LUA_CODE).expect("load run_lua_code function") };
        LibCKBLua { run_lua_code }
    }

    pub fn run_lua_code(
        &self,
        code: &[u8],
    ) -> Result<(), isize> {
        let run_lua_code_fn = &self.run_lua_code;

        let error_code = unsafe {
            run_lua_code_fn(
                code.as_ptr(),
                code.len() as u32,
            )
        };
        debug!("error code: {}", error_code);
        if error_code != 0 {
            return Err(error_code);
        }
        Ok(())
    }
}
