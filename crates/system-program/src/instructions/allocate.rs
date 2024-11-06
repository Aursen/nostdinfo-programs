use nostd_entrypoint_invoke::invoke_unchecked;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::entrypoint::ProgramResult;

/// Allocate space in a (possibly new) account without funding.
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` New account
pub struct Allocate<'a> {
    /// Account to be assigned.
    pub account: &'a NoStdAccountInfo,

    /// Number of bytes of memory to allocate.
    pub space: u64,
}

impl<'a> Allocate<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 1] = [self.account.to_meta_c_signer()];

        // instruction data
        // -  [0..4 ]: instruction discriminator
        // -  [4..12]: space
        let mut instruction_data = [0; 12];
        instruction_data[0] = 8;
        instruction_data[4..12].copy_from_slice(&self.space.to_le_bytes());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 1,
            data: instruction_data.as_ptr(),
            data_len: 12,
            program_id: &crate::ID,
        };

        invoke_unchecked(&instruction, &[self.account.to_info_c()], signers)
    }
}
