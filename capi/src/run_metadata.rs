//! The Run Metadata stores additional information about a run, like the
//! platform and region of the game. All of this information is optional.

use super::output_str;
use livesplit_core::RunMetadata;
use run_metadata_variables_iter::OwnedRunMetadataVariablesIter;
use std::os::raw::c_char;

/// type
pub type OwnedRunMetadata = Box<RunMetadata>;

/// Accesses the speedrun.com Run ID of the run. This Run ID specify which
/// Record on speedrun.com this run is associated with. This should be
/// changed once the Personal Best doesn't match up with that record
/// anymore. This may be empty if there's no association.
#[no_mangle]
pub extern "C" fn RunMetadata_run_id(this: &RunMetadata) -> *const c_char {
    output_str(this.run_id())
}

/// Accesses the name of the platform this game is run on. This may be empty
/// if it's not specified.
#[no_mangle]
pub extern "C" fn RunMetadata_platform_name(this: &RunMetadata) -> *const c_char {
    output_str(this.platform_name())
}

/// Returns <TRUE> if this speedrun is done on an emulator. However <FALSE>
/// may also indicate that this information is simply not known.
#[no_mangle]
pub extern "C" fn RunMetadata_uses_emulator(this: &RunMetadata) -> bool {
    this.uses_emulator()
}

/// Accesses the name of the region this game is from. This may be empty if
/// it's not specified.
#[no_mangle]
pub extern "C" fn RunMetadata_region_name(this: &RunMetadata) -> *const c_char {
    output_str(this.region_name())
}

/// Returns an iterator iterating over all the variables and their values
/// that have been specified.
#[no_mangle]
pub extern "C" fn RunMetadata_variables(
    this: &'static RunMetadata,
) -> OwnedRunMetadataVariablesIter {
    Box::new(this.variables())
}
