use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Give file as an argument");
        exit(1);
    }

    /* let file_contents = fs::read(&args[1]).unwrap(); */
    let file = smol_file::SmolFile::load(&args[1]);

    let mut vm = smol_vm::Vm::default();
    vm.registers.ic = file.main_start;
    vm.instructions.instructions = file.instructions;
    for storage in file.storage.items {
        let mem = vm.stack.memory_mut();
        if let Some(data) = storage.init_data {
            let start = storage.offset as usize;
            let end = start + storage.size as usize;
            mem[start..end].copy_from_slice(&data);
        }
    }
    vm.run();
}
