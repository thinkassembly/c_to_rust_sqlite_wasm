use libc_sys as libc;

type mut_char_ptr = * mut libc::c_char;
type char_ptr = * const libc::c_char;
#[derive(Debug,Copy,Clone)]
pub struct StringType{
    pub str_type:u8,
    pub str:&'static str,
    pub character_pointer:char_ptr,
    pub mut_character_pointer:mut_char_ptr
}
impl StringType {
    pub fn as_str_ref(&mut self) -> &'static str {
        match self.str_type {
            0 => {
                self.str
            }
            1 => {
               unsafe { ::std::ffi::CStr::from_ptr(self.character_pointer).to_str().unwrap() }
            }
            2 => {
              unsafe { ::std::ffi::CStr::from_ptr(self.mut_character_pointer).to_str().unwrap() }
            }
            _ => panic!()
        }
    }
}
impl From<mut_char_ptr> for StringType {
    fn from(string:mut_char_ptr) -> Self {
        StringType{
            str_type:2,
            character_pointer:string,
             mut_character_pointer:0 as mut_char_ptr,

            str:""
        }
    }
}
impl From<char_ptr> for StringType {
    fn from(string:char_ptr) -> Self {
        StringType{
            str_type:1,
            character_pointer:string,
            mut_character_pointer:0 as mut_char_ptr,
            str:""
        }
    }
}
impl From<&'static str> for StringType {
    fn from(string:&'static str) -> Self {
        StringType{
            str_type:0,
            character_pointer:0 as char_ptr,
            mut_character_pointer:0 as mut_char_ptr,
            str:string
        }
    }

}