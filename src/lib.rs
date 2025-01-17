#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use reth::revm::interpreter::opcode::BoxedInstruction;

/// This contains the custom instruction context required for `AUTH` and `AUTHCALL` instructions,
/// notably for the `authorized` context variable.
pub mod context;

/// The implementation of [EIP-3074](https://eips.ethereum.org/EIPS/eip-3074): `AUTH` and
/// `AUTHCALL` instructions.
pub mod instructions;

/// Association of instruction opcode and correspondent boxed instruction.
pub struct BoxedInstructionWithOpCode<'a, H> {
    /// Opcode.
    pub opcode: u8,
    /// Boxed instruction.
    pub boxed_instruction: BoxedInstruction<'a, H>,
}
