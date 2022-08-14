#![no_main]

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct Pubkey([u8; 32]);

#[derive(BorshSerialize, BorshDeserialize)]
struct Account {
    id: Pubkey,
    owner_id: Pubkey,
    data: Vec<u8>,
    is_writable: bool,
    is_signer: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct InstructionIO {
    program_id: Pubkey,
    input_data: Vec<u8>,
    accounts: Vec<Account>,
    result: ProgramResult,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum ProgramError {
    Custom { code: usize },
}

type ProgramResult = Result<Option<Vec<u8>>, ProgramError>;

#[no_mangle]
pub unsafe extern "C" fn entrypoint(data: *mut u8, data_len: usize) {
    let mut data = std::slice::from_raw_parts_mut(data, data_len);
    let mut io = InstructionIO::try_from_slice(data).expect("runtime error: invalid io layout.");
    io.result = process_instruction(&io.program_id, &io.input_data, &io.accounts);
    io.serialize(&mut data)
        .expect("runtime error: failed to write");
}

fn process_instruction(
    _program_id: &Pubkey,
    _input_data: &[u8],
    _accounts: &[Account],
) -> ProgramResult {
    Ok(None)
}
