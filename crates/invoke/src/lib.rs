#![no_std]

use core::mem::MaybeUninit;

use solana_nostd_entrypoint::{
    solana_program::entrypoint::ProgramResult, AccountInfoC, InstructionC, NoStdAccountInfo,
};
use solana_program::program_error::ProgramError;

#[inline(always)]
pub fn invoke<const ACCOUNTS: usize>(
    instruction: &InstructionC,
    account_infos: &[&NoStdAccountInfo; ACCOUNTS],
) -> ProgramResult {
    invoke_signed(instruction, account_infos, &[])
}

pub fn invoke_signed<const ACCOUNTS: usize>(
    instruction: &InstructionC,
    accounts: &[&NoStdAccountInfo; ACCOUNTS],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    if (instruction.accounts_len as usize) < ACCOUNTS {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    const UNINIT: MaybeUninit<AccountInfoC> = MaybeUninit::<AccountInfoC>::uninit();
    let mut infos = [UNINIT; ACCOUNTS];

    let metas = unsafe { core::slice::from_raw_parts(instruction.accounts, ACCOUNTS) };

    for index in 0..ACCOUNTS {
        let info = &accounts[index];
        let meta = &metas[index];

        if *info.key() != unsafe { *meta.pubkey } {
            return Err(ProgramError::InvalidArgument);
        }

        if meta.is_writable {
            let _ = info.try_borrow_mut_data();
            let _ = info.try_borrow_mut_lamports();
        } else {
            let _ = info.try_borrow_data();
            let _ = info.try_borrow_lamports();
        }

        infos[index].write(info.to_info_c());
    }

    invoke_unchecked(instruction, infos, signers_seeds)?;

    Ok(())
}

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
pub fn invoke_unchecked<const ACCOUNTS: usize>(
    instruction: &InstructionC,
    account_infos: [MaybeUninit<AccountInfoC>; ACCOUNTS],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            instruction as *const InstructionC as *const u8,
            account_infos.as_ptr() as *const u8,
            account_infos.len() as u64,
            signers_seeds.as_ptr() as *const u8,
            signers_seeds.len() as u64,
        );
    }

    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &account_infos, &signers_seeds));

    Ok(())
}

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}
