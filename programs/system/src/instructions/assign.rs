use invoke::invoke_unchecked;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Assign account to a program
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Assigned account public key
pub struct Assign<'a, 'b> {
    /// Account to be assigned.
    pub account: &'a NoStdAccountInfo,

    /// Program account to assign as owner.
    pub owner: &'b Pubkey,
}

impl<'a, 'b> Assign<'a, 'b> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 1] = [self.account.to_meta_c_signer()];

        // instruction data
        // -  [0..4 ]: instruction discriminator
        // -  [4..36]: owner pubkey
        let mut instruction_data = [0; 36];
        instruction_data[0] = 1;
        instruction_data[4..36].copy_from_slice(self.owner.as_ref());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 1,
            data: instruction_data.as_ptr(),
            data_len: 36,
            program_id: &crate::ID,
        };

        invoke_unchecked(&instruction, &[self.account.to_info_c()], signers)
    }
}
