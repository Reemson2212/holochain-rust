use crate::{
    wasm_engine::{api::ZomeApiResult, Runtime},
    NEW_RELIC_LICENSE_KEY,
};
use holochain_core_types::{
    self,
    dna::Dna,
    entry::{entry_type::EntryType, Entry},
    error::RibosomeRuntimeBits,
};
use holochain_persistence_api::cas::content::AddressableContent;

use std::str::FromStr;

pub fn get_entry_type(dna: &Dna, entry_type_name: &str) -> Result<EntryType, RibosomeRuntimeBits> {
#[holochain_tracing_macros::newrelic_autotrace(HOLOCHAIN_CORE)]
pub fn get_entry_type(dna: &Dna, entry_type_name: &str) -> Result<EntryType, Option<RuntimeValue>> {
    let entry_type = EntryType::from_str(&entry_type_name).map_err(|_| {
        holochain_core_types::error::WasmError::UnknownEntryType as RibosomeRuntimeBits
    })?;

    // Check if AppEntry is a valid AppEntryType
    if entry_type.is_app() {
        let result = dna.get_entry_type_def(entry_type_name);
        if result.is_none() {
            return Err(
                holochain_core_types::error::WasmError::UnknownEntryType
                    as RibosomeRuntimeBits,
            );
        }
    }
    // Done
    Ok(entry_type)
}

/// ZomeApiFunction::entry_address function code
/// args: [0] encoded MemoryAllocation as u64
/// Expected complex argument: entry_type_name and entry_value as JsonString
/// Returns an HcApiReturnCode as I64
pub fn invoke_entry_address(runtime: &mut Runtime, entry: Entry) -> ZomeApiResult {
    // Check if entry_type is valid
    let dna = runtime
        .context()?
        .state()
        .unwrap()
        .nucleus()
        .dna()
        .expect("Should have DNA");
    let maybe_entry_type = get_entry_type(&dna, &entry.entry_type().to_string());
    if let Err(err) = maybe_entry_type {
        return Ok(err);
    }

    // Return result
    runtime.store_result(Ok(entry.address()))
}
