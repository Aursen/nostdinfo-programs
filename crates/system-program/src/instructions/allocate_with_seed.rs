use nostd_entrypoint_invoke::invoke_signed;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Allocate space for and assign an account at an address derived
/// from a base public key and a seed.
///
/// ### Accounts:
///   0. `[WRITE]` Allocated account TODO CHECK
///   1. `[SIGNER]` Base account
pub struct AllocateWithSeed<'a, 'b, 'c> {
    /// Allocated account.
    pub account: &'a NoStdAccountInfo,

    /// Base account.
    ///
    /// The account matching the base Pubkey below must be provided as
    /// a signer, but may be the same as the funding account and provided
    /// as account 0.
    pub base: &'a NoStdAccountInfo,

    /// String of ASCII chars, no longer than `Pubkey::MAX_SEED_LEN`.
    pub seed: &'b str,

    /// Number of bytes of memory to allocate.
    pub space: u64,

    /// Address of program that will own the new account.
    pub owner: &'c Pubkey,
}

impl<'a, 'b, 'c> AllocateWithSeed<'a, 'b, 'c> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 2] = [
            self.account.to_meta_c_signer(),
            self.base.to_meta_c_signer(),
        ];

        // instruction data
        // - [0..4  ]: instruction discriminator
        // - [4..36 ]: base pubkey
        // - [36..40]: seed length
        // - [40..  ]: seed (max 32)
        // - [..  +8]: account space
        // - [.. +32]: owner pubkey
        let mut instruction_data = [0; 112];
        instruction_data[0] = 9;
        instruction_data[4..36].copy_from_slice(self.base.key().as_ref());
        instruction_data[36..40].copy_from_slice(&u32::to_le_bytes(self.seed.len() as u32));

        let offset = 40 + self.seed.len();
        instruction_data[40..offset].copy_from_slice(self.seed.as_bytes());
        instruction_data[offset..offset + 8].copy_from_slice(&self.space.to_le_bytes());
        instruction_data[offset + 8..offset + 40].copy_from_slice(self.owner.as_ref());

        let instruction = InstructionC {
            program_id: &crate::ID,
            accounts: account_metas.as_ptr(),
            accounts_len: 2,
            data: instruction_data.as_ptr(),
            data_len: (offset + 40) as u64,
        };

        invoke_signed(&instruction, &[self.account, self.base], signers)
    }
}
