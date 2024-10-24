use invoke::invoke_unchecked;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Transfer lamports from a derived address.
///
/// ### Accounts:
///   0. `[WRITE]` Funding account
///   1. `[SIGNER]` Base for funding account
///   2. `[WRITE]` Recipient account
pub struct TransferWithSeed<'a, 'b, 'c> {
    /// Funding account.
    pub from: &'a NoStdAccountInfo,

    /// Base account.
    ///
    /// The account matching the base Pubkey below must be provided as
    /// a signer, but may be the same as the funding account and provided
    /// as account 0.
    pub base: &'a NoStdAccountInfo,

    /// Recipient account.
    pub to: &'a NoStdAccountInfo,

    /// Amount of lamports to transfer.
    pub lamports: u64,

    /// String of ASCII chars, no longer than `Pubkey::MAX_SEED_LEN`.
    pub seed: &'b str,

    /// Address of program that will own the new account.
    pub owner: &'c Pubkey,
}

impl<'a, 'b, 'c> TransferWithSeed<'a, 'b, 'c> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.from.to_meta_c(),
            self.base.to_meta_c_signer(),
            self.to.to_meta_c(),
        ];

        // instruction data
        // - [0..4  ]: instruction discriminator
        // - [4..12 ]: lamports amount
        // - [12..16]: seed length
        // - [16..  ]: seed (max 32)
        // - [.. +32]: owner pubkey
        let mut instruction_data = [0; 80];
        instruction_data[0] = 11;
        instruction_data[4..12].copy_from_slice(&self.lamports.to_le_bytes());
        instruction_data[12..16].copy_from_slice(&u32::to_le_bytes(self.seed.len() as u32));

        let offset = 16 + self.seed.len();
        instruction_data[16..offset].copy_from_slice(self.seed.as_bytes());
        instruction_data[offset..offset + 32].copy_from_slice(self.owner.as_ref());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: (offset + 32) as u64,
            program_id: &crate::ID,
        };

        invoke_unchecked(
            &instruction,
            &[
                self.from.to_info_c(),
                self.base.to_info_c(),
                self.to.to_info_c(),
            ],
            signers,
        )
    }
}
