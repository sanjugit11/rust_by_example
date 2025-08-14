cargo build  //compile
cargo run    //run

cargo install cargo-expand


// memory managenment
the stack  = automatically managed,limited ,local,local varibale of fn
the heap = No automatically managed, we have to put pointer,  no limit,access by anywhere in program,expensive 
         {pointer e = allocate integer 7} //7 store in a emeory )0xdsda  and 0xdsda stores in e
         ** now deallocate e memory
pointers = above
smart pointers = //automatically deallocate
         eg : box::new(_integer) //smat pointer