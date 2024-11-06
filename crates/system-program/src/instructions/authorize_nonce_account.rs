use nostd_entrypoint_invoke::invoke_unchecked;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Change the entity authorized to execute nonce instructions on the account.
///
/// The `Pubkey` parameter identifies the entity to authorize.
///
/// ### Accounts:
///   0. `[WRITE]` Nonce account
///   1. `[SIGNER]` Nonce authority
pub struct AuthorizeNonceAccount<'a, 'b> {
    /// Nonce account.
    pub account: &'a NoStdAccountInfo,

    /// Nonce authority.
    pub authority: &'a NoStdAccountInfo,

    /// New entity authorized to execute nonce instructions on the account.
    pub new_authority: &'b Pubkey,
}

impl<'a, 'b> AuthorizeNonceAccount<'a, 'b> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 2] =
            [self.account.to_meta_c(), self.authority.to_meta_c_signer()];

        // instruction data
        // -  [0..4 ]: instruction discriminator
        // -  [4..12]: lamports
        let mut instruction_data = [0; 36];
        instruction_data[0] = 7;
        instruction_data[4..36].copy_from_slice(self.new_authority.as_ref());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 2,
            data: instruction_data.as_ptr(),
            data_len: 36,
            program_id: &crate::ID,
        };

        invoke_unchecked(
            &instruction,
            &[self.account.to_info_c(), self.authority.to_info_c()],
            signers,
        )
    }
}
