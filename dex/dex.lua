ERROR_LOAD_SCRIPT = 100
ERROR_INVALID_ARGS = 101

ARGS_MIN_LEN = 20

local function verify()
    local _code_hash, _hash_type, args, err = ckb.load_and_unpack_script()
    if err ~= nil then
        ckb.exit_script(ERROR_LOAD_SCRIPT)
    end
    if #args ~= 28 then
        ckb.exit_script(ERROR_INVALID_ARGS)
    end
    local extra_param = string.sub(args, ARGS_MIN_LEN+1, -1)

    local fmt = ">I8"
    local interest = fmt:unpack(extra_param, 1)

    ckb.dump(type(interest))
end

verify()