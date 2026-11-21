// Box, Rc, Weak and RefCell: ownership/move paths and reference counting.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::string::String;
use core::cell::RefCell;

fn body(seed: u64) -> u64 {
    let b = Box::new(seed ^ 0x1234_5678_9abc_def0);
    emit_u64(*b);

    let mut big = Box::new([0u8; 4096]);
    for i in 0..big.len() {
        big[i] = (i as u64).wrapping_mul(seed).wrapping_add(7) as u8;
    }
    let mut sum = 0u64;
    for &x in big.iter() {
        sum = sum.wrapping_add(x as u64);
    }
    emit_u64(sum);
    emit_u8(big[0] ^ big[4095]);

    let boxed: Box<[u64]> = (0..64u64).map(|i| i.wrapping_mul(seed)).collect();
    emit_usize(boxed.len());
    emit_u64(boxed[63]);

    let rc = Rc::new(seed & 0xffff);
    emit_usize(Rc::strong_count(&rc));
    let rc2 = Rc::clone(&rc);
    emit_usize(Rc::strong_count(&rc));
    emit_u64(*rc2);
    let weak: Weak<u64> = Rc::downgrade(&rc);
    emit_bool(weak.upgrade().is_some());
    drop(rc2);
    emit_usize(Rc::strong_count(&rc));
    drop(rc);
    emit_bool(weak.upgrade().is_none());

    let shared = Rc::new(RefCell::new(0u64));
    for i in 0..32u64 {
        let cur = *shared.borrow();
        *shared.borrow_mut() = cur.wrapping_add(i ^ seed);
    }
    emit_u64(*shared.borrow());
    emit_usize(Rc::strong_count(&shared));

    let mut unique = Rc::new(String::from("unique"));
    if let Some(s) = Rc::get_mut(&mut unique) {
        s.push_str("-mutated");
    }
    emit_str(&unique);
    let rc_str: Rc<str> = Rc::from("shared-str");
    emit_usize(rc_str.len());
    let clone_str = Rc::clone(&rc_str);
    emit_bool(Rc::ptr_eq(&rc_str, &clone_str));

    let nested: Rc<RefCell<alloc::vec::Vec<u64>>> = Rc::new(RefCell::new(alloc::vec::Vec::new()));
    for i in 0..16 {
        nested.borrow_mut().push(i * i);
    }
    emit_usize(nested.borrow().len());
    emit_u64(nested.borrow().iter().sum());

    let out = *shared.borrow() ^ sum ^ (*b);
    out
}
