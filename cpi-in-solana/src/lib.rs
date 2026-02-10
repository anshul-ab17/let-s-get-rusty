use solana_program::{account_info::AccountInfo,
    entrypoint::{ProgramResult},
    entrypoint,
    pubkey::Pubkey
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,9
    account_info: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let iter: Iter<'_,AccountInfo<'_>>= accounts.iter();
}
