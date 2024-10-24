use solana_nostd_entrypoint::{
    solana_program::entrypoint::ProgramResult, AccountInfoC, InstructionC,
};

/// Invoke a cross-program instruction with signatures but don't enforce Rust's
/// aliasing rules.
///
/// This function is like [`invoke_signed`] except that it does not check that
/// [`RefCell`]s within [`AccountInfo`]s are properly borrowable as described in
/// the documentation for that function. Those checks consume CPU cycles that
/// this function avoids.
///
/// [`RefCell`]: std::cell::RefCell
///
/// # Safety
///
/// __This function is incorrectly missing an `unsafe` declaration.__
///
/// If any of the writable accounts passed to the callee contain data that is
/// borrowed within the calling program, and that data is written to by the
/// callee, then Rust's aliasing rules will be violated and cause undefined
/// behavior.
pub fn invoke_unchecked(
    instruction: &InstructionC,
    account_infos: &[AccountInfoC],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const InstructionC as *const u8,
            infos.as_ptr() as *const u8,
            infos.len() as u64,
            seeds.as_ptr() as *const u8,
            seeds.len() as u64,
        );
    }

    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &account_infos, &signers_seeds));

    Ok(())
}
