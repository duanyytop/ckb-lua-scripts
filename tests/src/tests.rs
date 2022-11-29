use super::*;
use ckb_testtool::ckb_hash::blake2b_256;
use ckb_testtool::context::Context;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
const MAX_CYCLES: u64 = 70_000_000;

#[test]
fn test_success() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("ckb-lua-scripts");
    let out_point = context.deploy_cell(contract_bin);

    let lua_bin: Bytes = fs::read("../ckb-lua/build/libckblua.so").expect("load lua")
    .into();
    let lua_out_point = context.deploy_cell(lua_bin);
    let lua_dep = CellDep::new_builder()
        .out_point(lua_out_point)
        .build();

    let mut args: Vec<u8> = vec![];
    let lua_code: &[u8] = b"ckb.exit_script(32)";
    let lua_code_hash = &blake2b_256(lua_code)[0..20];
    args.extend(lua_code_hash);
    args.extend(&[123u8; 8]);
    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::from(args))
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    let witness_args = WitnessArgsBuilder::default()
        .lock(Some(Bytes::from(lua_code)).pack())
        .build();
    let witnesses = vec![witness_args.as_bytes().pack()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(lua_dep)
        .witnesses(witnesses)
        .build();

    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

