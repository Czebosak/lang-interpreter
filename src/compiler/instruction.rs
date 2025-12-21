use instruction_macro::Instruction;

#[derive(Instruction)]
pub enum Instruction<'a> {
    GrowStack { length: u32 },
    Move { addr: u32, data: &'a [u8] },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction() {
        println!("{:?}", Instruction::GrowStack { length: 8 }.to_bytes());
        println!("{:?}", Instruction::Move { addr: 0, data: &[4, 6, 3, 2] }.to_bytes());
        panic!("gg");
    }
}
