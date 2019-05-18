pub mod cerebral;

#[cfg(test)]
mod tests {
    use super::cerebral::CerebralVM;
    use std::io::{self};
    #[test]
    fn create_vm() {
        let inp = io::stdin();
        let out = io::stdout();
        let code = String::from("++++");
        let vm = CerebralVM::new(code, inp, out);
        assert_eq!(vm.get_data_ptr(), 0);
        assert_eq!(vm.get_instruction_ptr(), 0);
    }

    #[test]
    fn increment_data_ptr() {
        let inp = io::stdin();
        let out = io::stdout();
        let code = String::from (">>>>");
        let mut vm = CerebralVM::new(code, inp, out);
        vm.execute();
        assert_eq!(vm.get_data_ptr(), 4);
    }
    #[test]
    fn decrement_data_ptr() {
        let inp = io::stdin();
        let out = io::stdout();
        let code = String::from(">><<");
        let mut vm = CerebralVM::new(code, inp, out);
        vm.execute();
        assert_eq!(vm.get_data_ptr(), 0);
    }
    #[test]
    fn instruction_ptr() {
        let inp = io::stdin();
        let out = io::stdout();
        let code = String::from(">>>>");
        let mut vm = CerebralVM::new(code, inp, out);
        vm.execute();
        assert_eq!(vm.get_instruction_ptr(), 4);
    }
}
