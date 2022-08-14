use secp256k1::hashes::sha256;
use std::collections::HashMap;
use thiserror::Error;
use wasmer::{imports, CompileError, Function, Instance, Memory, MemoryType, Module, Store};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pubkey([u8; 20]);

pub struct Account {
    id: Pubkey,
    owner_id: Pubkey,
    data: Vec<u8>,
    is_writable: bool,
    is_signer: bool,
}

pub struct Instruction<'i> {
    program_id: Pubkey,
    data: &'i [u8],
    accounts: &'i [u8],
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("program id={program_id:?} not found.")]
    ProgramNotFound { program_id: Pubkey },
    #[error("program id={program_id:?} has invalid bytecode. source: {src}")]
    ProgramInvalid {
        program_id: Pubkey,
        src: CompileError,
    },
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

struct Program {
    entrypoint: Function,
}

impl Program {
    pub fn load(account: &Account) -> RuntimeResult<Self> {
        let store = Store::default();

        let memory = Memory::new(&store, MemoryType::new(1, None, false)).unwrap();

        let imports = imports! {
            "env" => {
                "memory" => memory,
            }
        };

        let module =
            Module::new(&store, &account.data).map_err(|err| RuntimeError::ProgramInvalid {
                program_id: account.id,
                src: err,
            })?;

        let instance = Instance::new(&module, &imports).unwrap();

        let entrypoint = instance.exports.get_function("entrypoint").unwrap().clone();

        Ok(Self { entrypoint })
    }

    fn process_instruction(&self, instruction: &Instruction) {}
}

pub fn process_instruction(
    instruction: &Instruction,
    accounts: &mut HashMap<Pubkey, Account>,
) -> RuntimeResult<()> {
    // move program loading to outer block, ex: process_transaction
    let program_account =
        accounts
            .get(&instruction.program_id)
            .ok_or(RuntimeError::ProgramNotFound {
                program_id: instruction.program_id,
            })?;

    let program = Program::load(program_account)?;
    program.process_instruction(instruction);

    Ok(())
}

struct Transaction {
    predecessor_hashes: Vec<sha256::Hash>,
}
