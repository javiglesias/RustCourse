//Course Crate
pub mod greetIt {//topLevel module
    pub mod Eng {//submodule
        pub fn hey() ->String {
            "Hello".to_string()
        }
        pub fn dew() -> String {
            "Bye".to_string()
        }
    }
    pub mod Esp {
        pub fn hey() ->String {
            "Hola".to_string()
        }
        pub fn dew() -> String {
            "Dewe".to_string()
        }
    }
}
#[test]
fn test_Eng() {
    assert_eq!("Hello",greetIt::Eng::hey());
    //assert_eq!("test|",greetIt::Eng::dew());
}
#[test]
#[should_panic]
//! this also generates documentation
fn test_Esp() {
    //assert_eq!("test|",greetIt::Eng::hey());
    assert_eq!("Dew",greetIt::Esp::dew());
}
//comment line ignore by the compiler
/*multiLine comment ignored by the compiler*/
///applies to the code `follows` it.

/// this is used for generate documentation

//! #Examples
//! ```
//! The examples here has to work.
//! ```
//rustdoc fichero.rs to generate the documentation
