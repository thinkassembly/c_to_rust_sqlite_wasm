use std::collections::HashMap;
use string::sqlite3UpperToLower;
use string_type_proxy::StringType;
extern crate libc_sys;
use sqlite3Malloc;
use libc_sys as libc;
use sqlite3_free;
use sqlite3MallocSize;
use memset;
use sqlite3StrICmp;
use std::convert::Into;



/* Each element in the hash table is an instance of the following
** structure.  All elements are stored on a single doubly-linked list.
**
** Again, this structure is intended to be opaque, but it can't really
** be opaque because it is used by macros.
*/
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct HashElem {
    pub next: *mut HashElem,
    pub prev: *mut HashElem,
    pub data: *mut libc::c_void,
    pub pKey: *const libc::c_char,
}


/* A complete hash table is an instance of the following structure.
** The internals of this structure are intended to be opaque -- client
** code should not attempt to access or modify the fields of this structure
** directly.  Change this structure only by using the routines below.
** However, some of the "procedures" and "functions" for modifying and
** accessing this structure are really macros, so we can't really make
** this structure opaque.
**
** All elements of the hash table are on a single doubly-linked list.
** Hash.first points to the head of this list.
**
** There are Hash.htsize buckets.  Each bucket points to a spot in
** the global doubly-linked list.  The contents of the bucket are the
** element pointed to plus the next _ht.count-1 elements in the list.
**
** Hash.htsize and Hash.ht may be zero.  In that case lookup is done
** by a linear search of the global list.  For small tables, the
** Hash.ht table is never allocated because if there are few elements
** in the table, it is faster to do a linear search than to manage
** the hash table.
*/
pub type DefaultHashDataType = * mut libc::c_void;
#[derive(Debug,   Clone)]
#[repr(C)]
pub struct Hash<D: std::fmt::Debug> {
    pub count: libc::c_uint,
    pub first: *mut HashElem,
    pub map: HashMap<&'static str,  D>,
}

impl<D: std::fmt::Debug> Hash<D> {

    pub fn new() -> Self {
        Hash {
            count: 0,
            first: 0 as *mut HashElem,
            map: HashMap::new(),
        }
    }
    /* Turn bulk memory into a hash table object by initializing the
    ** fields of the Hash structure.
    **
    ** "pNew" is a pointer to the hash table that is to be initialized.
    */
    pub fn sqlite3HashInit(&mut self) {
        self.first = 0 as *mut HashElem;
        self.count = 0 as libc::c_int as libc::c_uint;
    }
    /* Remove all entries from a hash table.  Reclaim all memory.
    ** Call this routine to delete a hash table or to reset a hash table
    ** to the empty state.
    */
    pub fn sqlite3HashClear(&mut self) {
           self.map.clear();
          self.map.shrink_to(0);
    }
    /* Insert an element into the hash table pH.  The key is pKey
** and the data is "data".
**
** If no element exists with a matching key, then a new
** element is created and NULL is returned.
**
** If another element already exists with the same key, then the
** new data replaces the old data and the old data is returned.
** The key is not copied in this instance.  If a malloc fails, then
** the new data is returned and the hash table is unchanged.
**
** If the "data" parameter to this function is NULL, then the
** element corresponding to "key" is removed from the hash table.
*/
    pub fn insert_item<T:Into<StringType> + Copy + std::fmt::Debug >(&mut self, mut pKey: T, mut data: D) -> *mut libc::c_void {
        println!("INSERT_ITEM {:?}",pKey.into().as_str_ref());
        println!("map {:?}",self.map);
        match self.map.insert(pKey.into().as_str_ref(), data) {
            Some(mut x) => { return  &mut x as  * const _ as  *mut libc::c_void; }
            None => {
                return 0 as *mut libc::c_void;
            }
        }
    }



    /* Attempt to locate an element of the hash table pH with a key
    ** that matches pKey.  Return the data for this element if it is
    ** found, or NULL if there is no match.
    */
    pub fn get_item<T:Into<StringType> + std::fmt::Debug +Copy >(&mut self, mut pKey:T) -> *mut libc::c_void {
        println!("Hash: Get item {:?}",pKey.into().as_str_ref());
        println!("map {:?}",self);
        match self.map.get(pKey.into().as_str_ref()) {
            Some(mut x) => {println!("{:?}",x); &mut x as * const _ as *mut libc::c_void},
            None => {
                println!("{:?} wasn't found ",pKey);
                0 as * mut libc::c_void
            }
        }
    }






}

impl<D: std::fmt::Debug> Drop for Hash<D> {
    fn drop(&mut self) {
        println!("Dropping Hash! {:?}",self);
    }
}