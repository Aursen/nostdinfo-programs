use nostd_entrypoint_invoke::invoke_signed;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::entrypoint::ProgramResult;

/// Transfer lamports.
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Funding account
///   1. `[WRITE]` Recipient account
pub struct Transfer<'a> {
    /// Funding account.
    pub from: &'a NoStdAccountInfo,

    /// Recipient account.
    pub to: &'a NoStdAccountInfo,

    /// Amount of lamports to transfer.
    pub lamports: u64,
}

impl<'a> Transfer<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 2] = [self.from.to_meta_c_signer(), self.to.to_meta_c()];

        // instruction data
        // -  [0..4 ]: instruction discriminator
        // -  [4..12]: lamports amount
        let mut instruction_data = [0; 12];
        instruction_data[0] = 2;
        instruction_data[4..12].copy_from_slice(&self.lamports.to_le_bytes());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 2,
            data: instruction_data.as_ptr(),
            data_len: 12,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.from, self.to], signers)
    }
}
