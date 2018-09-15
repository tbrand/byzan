#[macro_use]
#[path = "./mod_tests.rs"]
mod mod_tests;

use self::mod_tests::*;
use bc::mem::Mem;

define_tests!(Mem);
