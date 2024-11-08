use nostd_entrypoint_invoke::{invoke_signed, write_bytes, UNINIT_BYTE};
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Create a new account.
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Funding account
///   1. `[WRITE, SIGNER]` New account
pub struct CreateAccount<'a> {
    /// Funding account.
    pub from: &'a NoStdAccountInfo,

    /// New account.
    pub to: &'a NoStdAccountInfo,

    /// Number of lamports to transfer to the new account.
    pub lamports: u64,

    /// Number of bytes of memory to allocate.
    pub space: u64,

    /// Address of program that will own the new account.
    pub owner: &'a Pubkey,
}

impl<'a> CreateAccount<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 2] =
            [self.from.to_meta_c_signer(), self.to.to_meta_c_signer()];

        // instruction data
        // - [0..4  ]: instruction discriminator
        // - [4..12 ]: lamports
        // - [12..20]: account space
        // - [20..52]: owner pubkey
        let mut instruction_data = [UNINIT_BYTE; 52];
        // create account instruction has a '0' discriminator
        write_bytes(&mut instruction_data, &[0, 0, 0, 0]);
        write_bytes(&mut instruction_data[4..12], &self.lamports.to_le_bytes());
        write_bytes(&mut instruction_data[12..20], &self.space.to_le_bytes());
        write_bytes(&mut instruction_data[20..52], &self.owner.as_ref());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 2,
            data: instruction_data.as_ptr() as _,
            data_len: 52,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.from, self.to], signers)
    }
}
