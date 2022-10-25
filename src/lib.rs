use hash::{Hash, HashAlgorithm};
use trinci_sdk::{
    rmp_deserialize, rmp_serialize, rmp_serialize_named, tai::AssetTransferArgs as TransferArgs,
    AppContext, PackedValue, Serializable, WasmError, WasmResult,
};
mod types;
use types::*;

mod ecdsa;
mod ed25519;
mod hash;
mod macros;
mod sign;

const VERSION: &str = env!("CARGO_PKG_VERSION");

trinci_sdk::app_export!(init, init_fortress, collect_resource, send_resources);

#[inline]
fn is_initialized() -> bool {
    !trinci_sdk::load_data(INIT_KEY).is_empty()
}

/// Initialize plane.
fn init(ctx: AppContext, init_args: InitArgs) -> WasmResult<()> {
    if is_initialized() {
        return Ok(());
    }

    let config = PlaneConfig::new(ctx.caller, init_args.clone());

    trinci_sdk::store_account_data_mp!(CONFIG_KEY, &config)?;
    trinci_sdk::store_data(INIT_KEY, &[1]);

    // emit event
    trinci_sdk::emit_data(
        &format!("{}:init", ctx.caller),
        &init_args.serialize().unwrap_or_default(),
    );

    Ok(())
}

/// Initialize a fortress.
///
/// The caller become the asset creator.
fn init_fortress(ctx: AppContext, args: InitFortressArgs) -> WasmResult<()> {
    if is_initialized() {
        return Ok(());
    }

    let founded_fortress = Fortress::new(args.clone())?;

    trinci_sdk::store_account_data_mp!(CONFIG_KEY, &config)?;
    trinci_sdk::store_data(INIT_KEY, &[1]);

    // emit event
    trinci_sdk::emit_data(
        &format!("{}:fortress_foundation", ctx.caller),
        &args.serialize().unwrap_or_default(),
    );

    Ok(())
}

/// Collect fortress resource
fn collect_resource(ctx: AppContext, _args: PackedValue) -> WasmResult<u64> {
    // TODO: load data

    // call method

    // retrun value
    todo!()
}

/// Collect fortress resource
fn send_resources(ctx: AppContext, send_args: SendResourcesArgs) -> WasmResult<u64> {
    // TODO: load data

    // call method

    // retrun value
    todo!()
}
