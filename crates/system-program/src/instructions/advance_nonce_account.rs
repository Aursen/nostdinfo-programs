use nostd_entrypoint_invoke::invoke_unchecked;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::entrypoint::ProgramResult;

/// Consumes a stored nonce, replacing it with a successor.
///
/// ### Accounts:
///   0. `[WRITE]` Nonce account
///   1. `[]` RecentBlockhashes sysvar
///   2. `[SIGNER]` Nonce authority
pub struct AdvanceNonceAccount<'a> {
    /// Nonce account.
    pub account: &'a NoStdAccountInfo,

    /// RecentBlockhashes sysvar.
    pub recent_blockhashes_sysvar: &'a NoStdAccountInfo,

    /// Nonce authority.
    pub authority: &'a NoStdAccountInfo,
}

impl<'a> AdvanceNonceAccount<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.recent_blockhashes_sysvar.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        let data = &[4];

        // instruction
        let instruction = InstructionC {
            program_id: &crate::ID,
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: data.as_ptr(),
            data_len: 1,
        };

        invoke_unchecked(
            &instruction,
            &[
                self.account.to_info_c(),
                self.recent_blockhashes_sysvar.to_info_c(),
                self.authority.to_info_c(),
            ],
            signers,
        )
    }
}
