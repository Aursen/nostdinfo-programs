use nostd_entrypoint_invoke::invoke_unchecked;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Drive state of Uninitialized nonce account to Initialized, setting the nonce value.
///
/// The `Pubkey` parameter specifies the entity authorized to execute nonce
/// instruction on the account
///
/// No signatures are required to execute this instruction, enabling derived
/// nonce account addresses.
///
/// ### Accounts:
///   0. `[WRITE]` Nonce account
///   1. `[]` RecentBlockhashes sysvar
///   2. `[]` Rent sysvar
pub struct InitializeNonceAccount<'a, 'b> {
    /// Nonce account.
    pub account: &'a NoStdAccountInfo,

    /// RecentBlockhashes sysvar.
    pub recent_blockhashes_sysvar: &'a NoStdAccountInfo,

    /// Rent sysvar.
    pub rent_sysvar: &'a NoStdAccountInfo,

    /// Lamports to withdraw.
    ///
    /// The account balance muat be left above the rent exempt reserve
    /// or at zero.
    pub authority: &'b Pubkey,
}

impl<'a, 'b> InitializeNonceAccount<'a, 'b> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.recent_blockhashes_sysvar.to_meta_c(),
            self.rent_sysvar.to_meta_c(),
        ];

        // instruction data
        // -  [0..4 ]: instruction discriminator
        // -  [4..36]: authority pubkey
        let mut instruction_data = [0; 36];
        instruction_data[0] = 6;
        instruction_data[4..36].copy_from_slice(self.authority.as_ref());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 36,
            program_id: &crate::ID,
        };

        invoke_unchecked(
            &instruction,
            &[
                self.account.to_info_c(),
                self.recent_blockhashes_sysvar.to_info_c(),
                self.rent_sysvar.to_info_c(),
            ],
            signers,
        )
    }
}
