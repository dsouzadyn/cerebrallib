pub mod cerebral;

#[cfg(test)]
mod tests {
    use super::cerebral::CerebralVM;
    #[test]
    fn create_vm() {
        let code = String::from("++++");
        let vm = CerebralVM::new(code);
        assert_eq!(vm.get_data_ptr(), 0);
        assert_eq!(vm.get_instruction_ptr(), 0);
    }

    #[test]
    fn increment_data_ptr() {
        let code = String::from (">>>>");
        let mut vm = CerebralVM::new(code);
        vm.execute();
        assert_eq!(vm.get_data_ptr(), 4);
    }
    #[test]
    fn decrement_data_ptr() {
        let code = String::from(">><<");
        let mut vm = CerebralVM::new(code);
        vm.execute();
        assert_eq!(vm.get_data_ptr(), 0);
    }
    #[test]
    fn instruction_ptr() {
        let code = String::from(">>>>");
        let mut vm = CerebralVM::new(code);
        vm.execute();
        assert_eq!(vm.get_instruction_ptr(), 4);
    }
}
