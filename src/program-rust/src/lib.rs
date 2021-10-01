use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Fibonacci {
    pub val: u32, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

impl Fibonacci {
    pub fn calculate(&mut self, my_data: u8) {
        self.val = fibonacci(my_data);
    }
}

fn fibonacci(n: u8) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}


// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Calculate fibonacci and store the number of times the account has been greeted
    let mut fibo_account = Fibonacci::try_from_slice(&account.data.borrow())?;
    fibo_account.calculate(_instruction_data[0]);
    fibo_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("fibonacci value generated {}", fibo_account.val);

    Ok(())
}


// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let mut instruction_data: Vec<u8> = Vec::new();
        instruction_data.push(0);

        let accounts = vec![account];

        assert_eq!(
            Fibonacci::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .val,
            1
        );
        instruction_data[0] = 1;
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            Fibonacci::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .val,
            1
        );
        instruction_data[0] = 2;
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            Fibonacci::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .val,
            2
        );
        instruction_data[0] = 5;
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            Fibonacci::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .val,
            13
        );
    }
}
