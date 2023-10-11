pub mod decode;
pub mod instruction;

#[derive(Copy, Clone)]
pub enum Isa {
    Rv32,
    Rv64,
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
