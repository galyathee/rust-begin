// This module is part of the crate's public API
pub mod my_module {
    // This function is public and can be accessed from outside the module
    pub fn public_function() {
        println!("This is a public function.");
    }

    // This function is public within the current crate but not outside
    pub(crate) fn crate_function() {
        println!("This is a crate-level function.");
    }

    // This function is public within the parent module but not outside
    pub(super) fn super_function() {
        println!("This is a super-level function.");
    }

    // Use a submodule function
    pub fn path_function() {
        submodule::path_function();
    }

    // This module is public
    pub mod submodule {
        // This struct is public and can be accessed from outside the submodule
        pub struct MyStruct {
            pub field: i32,
        }

        // This function is public within the submodule but not outside
        pub fn submodule_function() {
            println!("This is a function in the submodule.");
        }

        // This function is public within the specified path but not outside
        // Can only function using the preceeding: use crate::visibility_modifiers::my_module
        pub(crate) fn path_function() {
            println!("This is a path-level function within my_module.");
        }
    }

    // This function is private and not part of the crate's public API
    fn private_function() {
        println!("This is a private function.");
    }

    pub fn use_private_function() {
        private_function();
    }
}

// set pub to use in main.rs
pub fn main() {
    // Access public items from the crate's public API
    my_module::public_function();
    my_module::crate_function();
    my_module::super_function();
    my_module::path_function();
    my_module::use_private_function();

    // Access items from the submodule's public API
    let my_struct = my_module::submodule::MyStruct { field: 42 };
    println!("Field value: {}", my_struct.field);
    my_module::submodule::submodule_function();
    my_module::submodule::path_function();

    // Private items are not accessible from here
    // my_module::private_function(); // This would result in a compile error
}
