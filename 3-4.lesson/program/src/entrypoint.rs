use crate::processor::process_instruction;
use solana_program::entrypoint;

entrypoint!(process_instruction);
// entry point is compulsory and strict in implementation
// entrypoint macro from solana_program library
// processor crate points to our processor.rs file
// which contains the whole logic of our program
