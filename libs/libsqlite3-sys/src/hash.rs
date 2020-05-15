use std::collections::HashMap;
use string::sqlite3UpperToLower;

extern crate libc_sys;

use libc_sys as libc;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct _ht {
    pub count: libc::c_uint,
    pub chain: *mut HashElem,
}

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
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub htsize: libc::c_uint,
    pub count: libc::c_uint,
    pub first: *mut HashElem,
    pub ht: *mut _ht,
    pub map: HashMap,
}

impl Hash {
    pub fn first_element(&self) -> HashElem {
        unsafe { *self.first }
    }

    pub fn new() -> Self {
        Hash {
            htsize: 0,
            count: 0,
            first: 0 as *mut HashElem,
            ht: 0 as *mut _ht,
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
        self.htsize = 0 as libc::c_int as libc::c_uint;
        self.ht = 0 as *mut _ht;
    }
    /* Remove all entries from a hash table.  Reclaim all memory.
    ** Call this routine to delete a hash table or to reset a hash table
    ** to the empty state.
    */
    pub fn sqlite3HashClear(&mut self) {
        unsafe {
            let mut elem: *mut HashElem =
                0 as *mut HashElem; /* For looping over all elements of the table */
            elem = self.first;
            self.first = 0 as *mut HashElem;
            sqlite3_free(self.ht as *mut libc::c_void);
            self.ht = 0 as *mut _ht;
            self.htsize = 0 as libc::c_int as libc::c_uint;
            while !elem.is_null() {
                let mut next_elem: *mut HashElem = (*elem).next;
                sqlite3_free(elem as *mut libc::c_void);
                elem = next_elem
            }
            self.count = 0 as libc::c_int as libc::c_uint;
        }
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
    pub fn insert_item(&mut self, mut pKey: *const libc::c_char, mut data: *mut libc::cvoid) -> *mut libc::c_void {
        let key = unsafe { ::std::ffi::CStr::from_ptr(pkey).to_str().unwrap() };
        match self.map.insert(key, data) {
            Some(x) => { return x as *mut libc::void; }
            None(x) => {
                return 0 as *mut libc::c_void;
            }
        }
    }

    pub fn insert_item_old(&mut self,
                           mut pKey: *const libc::c_char,
                           mut data: *mut libc::c_void)
                           -> *mut libc::c_void {
        unsafe {
            println!("hash insert {:#?}", std::ffi::CStr::from_ptr(data as *const libc::c_char));
            let mut h: libc::c_uint = 0; /* the hash of the key modulo hash table size */
            let mut elem: *mut HashElem = 0 as *mut HashElem; /* Used to loop thru the element list */
            let mut new_elem: *mut HashElem = 0 as *mut HashElem; /* New element added to the self */
            elem = self.find_element(pKey, &mut h);
            if !(*elem).data.is_null() {
                let mut old_data: *mut libc::c_void = (*elem).data;
                if data.is_null() {
                    self.removeElementGivenHash(elem, h);
                } else {
                    (*elem).data = data;
                    (*elem).pKey = pKey
                }
                return old_data;
            }
            if data.is_null() { return 0 as *mut libc::c_void; }
            new_elem =
                sqlite3Malloc(::std::mem::size_of::<HashElem>() as libc::c_ulong as
                    u64) as *mut HashElem;
            println!("NEW ELEM {:?}", new_elem);
            if new_elem.is_null() { return data; }
            (*new_elem).pKey = pKey;
            (*new_elem).data = data;
            (*self).count = (*self).count.wrapping_add(1);
            if (*self).count >= 10 as libc::c_int as libc::c_uint &&
                (*self).count >
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*self).htsize) {
                if self.rehash(
                    (*self).count.wrapping_mul(2))
                    != 0 {
                    h = Self::strHash(pKey).wrapping_rem((*self).htsize)
                }
            }


            self.insertElement(
                if !(*self).ht.is_null() {
                    &mut *(*self).ht.offset(h as isize)
                } else { 0 as *mut _ht }, new_elem);
            return 0 as *mut libc::c_void;
        }
    }


    /* Attempt to locate an element of the hash table pH with a key
    ** that matches pKey.  Return the data for this element if it is
    ** found, or NULL if there is no match.
    */
    pub fn get_item(&mut self, mut pKey: *const libc::c_char) -> *mut libc::c_void {
        println!("hash table {:?}", self.first_element());
        unsafe { (*self.find_element(pKey, 0 as *mut libc::c_uint)).data }
    }

    /* This function (for internal use only) locates an element in an
    ** hash table that matches the given key.  If no element is found,
    ** a pointer to a static null element with HashElem.data==0 is returned.
    ** If pH is not NULL, then the hash for this key is written to *pH.
    */
    fn find_element(&self,
                    mut pKey: *const libc::c_char,
                    mut pHash: *mut libc::c_uint)
                    -> *mut HashElem
    /* Write the hash value here */
    {
        unsafe {
            let mut elem: *mut HashElem =
                0 as *mut HashElem; /* Used to loop thru the element list */
            let mut count: libc::c_uint = 0; /* Number of elements left to test */
            let mut h: libc::c_uint = 0; /* The computed hash */
            static mut nullElement: HashElem =
                {
                    let mut init =
                        HashElem {
                            next: 0 as *const HashElem as *mut HashElem,
                            prev: 0 as *const HashElem as *mut HashElem,
                            data: 0 as *const libc::c_void as *mut libc::c_void,
                            pKey: 0 as *const libc::c_char,
                        };
                    init
                };
            if !(*self).ht.is_null() {
                /*OPTIMIZATION-IF-TRUE*/
                let mut pEntry: *mut _ht = 0 as *mut _ht;
                h = Self::strHash(pKey).wrapping_rem((*self).htsize);
                pEntry = &mut *(*self).ht.offset(h as isize) as *mut _ht;
                elem = (*pEntry).chain;
                count = (*pEntry).count
            } else {
                h = 0 as libc::c_int as libc::c_uint;
                elem = (*self).first;
                count = (*self).count
            }
            if !pHash.is_null() { *pHash = h }
            loop {
                let fresh45 = count;
                count = count.wrapping_sub(1);
                if !(fresh45 != 0) { break; }
                if sqlite3StrICmp((*elem).pKey, pKey) == 0 as libc::c_int {
                    return elem;
                }
                elem = (*elem).next
            }
            return &mut nullElement;
        }
    }

    /* Remove a single entry from the hash table given a pointer to that
    ** element and a hash on the element's key.
    */
    fn removeElementGivenHash(&mut self,
                              mut elem: *mut HashElem,
                              mut h: libc::c_uint)
    /* Hash value for the element */
    {
        unsafe {
            let mut pEntry: *mut _ht = 0 as *mut _ht;
            if !(*elem).prev.is_null() {
                (*(*elem).prev).next = (*elem).next
            } else { (*self).first = (*elem).next }
            if !(*elem).next.is_null() { (*(*elem).next).prev = (*elem).prev }
            if !(*self).ht.is_null() {
                pEntry = &mut *(*self).ht.offset(h as isize) as *mut _ht;
                if (*pEntry).chain == elem { (*pEntry).chain = (*elem).next }
                (*pEntry).count = (*pEntry).count.wrapping_sub(1)
            }
            sqlite3_free(elem as *mut libc::c_void);
            (*self).count = (*self).count.wrapping_sub(1);
            if (*self).count == 0 as libc::c_int as libc::c_uint {
                self.sqlite3HashClear();
            };
        }
    }
    /* Resize the hash table so that it cantains "new_size" buckets.
    **
    ** The hash table might fail to resize if sqlite3_malloc() fails or
    ** if the new size is the same as the prior size.
    ** Return TRUE if the resize occurs and false if not.
    */
    pub fn rehash(&mut self, mut new_size: libc::c_uint)
                  -> libc::c_int {
        unsafe {
            let mut new_ht: *mut _ht = 0 as *mut _ht; /* The new hash table */
            let mut elem: *mut HashElem =
                0 as *mut HashElem; /* For looping over existing elements */
            let mut next_elem: *mut HashElem = 0 as *mut HashElem;
            if (new_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<_ht>() as
                libc::c_ulong) >
                1024 as libc::c_int as libc::c_ulong {
                new_size =
                    (1024 as libc::c_int as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<_ht>() as
                        libc::c_ulong) as
                        libc::c_uint
            }
            if new_size == (*self).htsize { return 0 as libc::c_int; }
            /* The inability to allocates space for a larger hash table is
          ** a performance hit but it is not a fatal error.  So mark the
          ** allocation as a benign. Use sqlite3Malloc()/memset(0) instead of
          ** sqlite3MallocZero() to make the allocation, as sqlite3MallocZero()
          ** only zeroes the requested number of bytes whereas this module will
          ** use the actual amount of space allocated for the hash table (which
          ** may be larger than the requested amount).
          */
            new_ht =
                sqlite3Malloc((new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<_ht>()
                    as libc::c_ulong)
                    as u64) as *mut _ht;
            if new_ht.is_null() { return 0 as libc::c_int; }
            sqlite3_free((*self).ht as *mut libc::c_void);
            (*self).ht = new_ht;
            new_size =
                (sqlite3MallocSize(new_ht as *mut libc::c_void) as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<_ht>() as
                    libc::c_ulong) as libc::c_uint;
            (*self).htsize = new_size;
            memset(new_ht as *mut libc::c_void, 0 as libc::c_int,
                   (new_size as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<_ht>() as
                       libc::c_ulong));
            elem = (*self).first;
            (*self).first = 0 as *mut HashElem;
            while !elem.is_null() {
                let mut h: libc::c_uint =
                    Self::strHash((*elem).pKey).wrapping_rem(new_size);
                next_elem = (*elem).next;
                self.insertElement(&mut *new_ht.offset(h as isize), elem);
                elem = next_elem
            }
            return 1 as libc::c_int;
        }
    }


    /*
    ** The hashing function.
    */
    fn strHash(mut z: *const libc::c_char) -> libc::c_uint {
        unsafe {
            let mut h: u32 = 0;
            let mut c: libc::c_uchar = 0;
            loop {
                let fresh44 = z;
                z = z.offset(1);
                c = *fresh44 as libc::c_uchar;
                if !(c as libc::c_int != 0) { break; }
                /*OPTIMIZATION-IF-TRUE*/
                /* Knuth multiplicative hashing.  (Sorting & Searching, p. 510).
            ** 0x9e3779b1 is 2654435761 which is the closest prime number to
            ** (2**32)*golden_ratio, where golden_ratio = (sqrt(5) - 1)/2. */
                h = h.wrapping_add(sqlite3UpperToLower[c as usize] as u32);
                h = h.wrapping_mul(0x9e3779b1 as u32)
            }
            return h;
        }
    }
    /* Link pNew element into the hash table pH.  If pEntry!=0 then also
    ** insert pNew into the pEntry hash bucket.
    */
    fn insertElement(&mut self, mut pEntry: *mut _ht,
                     mut pNew: *mut HashElem)
    /* The element to be inserted */
    {
        unsafe {
            let mut pHead: *mut HashElem =
                0 as *mut HashElem; /* First element already in pEntry */
            if !pEntry.is_null() {
                pHead =
                    if (*pEntry).count != 0 {
                        (*pEntry).chain
                    } else { 0 as *mut HashElem };
                (*pEntry).count = (*pEntry).count.wrapping_add(1);
                (*pEntry).chain = pNew
            } else { pHead = 0 as *mut HashElem }
            if !pHead.is_null() {
                (*pNew).next = pHead;
                (*pNew).prev = (*pHead).prev;
                if !(*pHead).prev.is_null() {
                    (*(*pHead).prev).next = pNew
                } else { (*self).first = pNew }
                (*pHead).prev = pNew
            } else {
                (*pNew).next = (*self).first;
                if !(*self).first.is_null() { (*(*self).first).prev = pNew }
                (*pNew).prev = 0 as *mut HashElem;
                (*self).first = pNew
            };
        }
    }
}


