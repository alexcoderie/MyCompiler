use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::mem::size_of;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    OCall,
    OCallExt,
    OCastID,
    ODrop,
    OEnter,
    OEqD,
    OHalt,
    OInsert,
    OJtI,
    OLoad,
    OOffset,
    OPushFpAddr,
    OPushCtA,
    OPushCtI,
    ORet,
    OStore,
    OSubD,
    OAddI,
}

pub struct Instr {
    opcode: Opcode,
    args: [Arg; 2],
    last: *mut Instr,
    next: *mut Instr,
}

pub union Arg {
    i: i64,
    d: f64,
    addr: *mut u8,
}

pub static mut INSTRUCTIONS: *mut Instr = ptr::null_mut();
static mut LAST_INSTRUCTION: *mut Instr = ptr::null_mut();

static mut SP: *mut u8 = ptr::null_mut();
static mut STACK: *mut u8 = ptr::null_mut();
static mut STACK_AFTER: *mut u8 = ptr::null_mut();

const STACK_SIZE: usize = 1024;
const GLOBAL_SIZE: usize = 1024;

static mut GLOBALS: [u8; GLOBAL_SIZE] = [0; GLOBAL_SIZE];
static mut N_GLOBALS: usize = 0;

unsafe fn err(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}

unsafe fn pushd(d: f64) {
    if SP.add(size_of::<f64>()) > STACK_AFTER {
        err("out of stack");
    }
    *(SP as *mut f64) = d;
    SP = SP.add(size_of::<f64>());
}

unsafe fn popd() -> f64 {
    SP = SP.sub(size_of::<f64>());
    if SP < STACK {
        err("not enough stack bytes for popd");
    }
    *(SP as *mut f64)
}

unsafe fn pushi(i: i64) {
    if SP.add(size_of::<i64>()) > STACK_AFTER {
        err("out of stack");
    }
    *(SP as *mut i64) = i;
    SP = SP.add(size_of::<i64>());
}

unsafe fn popi() -> i64 {
    SP = SP.sub(size_of::<i64>());
    if SP < STACK {
        err("not enough stack bytes for popi");
    }
    *(SP as *mut i64)
}

unsafe fn pusha(a: *mut u8) {
    if SP.add(size_of::<*mut u8>()) > STACK_AFTER {
        err("out of stack");
    }
    *(SP as *mut *mut u8) = a;
    SP = SP.add(size_of::<*mut u8>());
}

unsafe fn popa() -> *mut u8 {
    SP = SP.sub(size_of::<*mut u8>());
    if SP < STACK {
        err("not enough stack bytes for popa");
    }
    *(SP as *mut *mut u8)
}

pub unsafe fn create_instr(opcode: Opcode) -> *mut Instr {
    let layout = Layout::new::<Instr>();
    let i = alloc(layout) as *mut Instr;
    if i.is_null() {
        err("failed to allocate memory for Instr");
    }
    (*i).opcode = opcode;
    (*i).last = ptr::null_mut();
    (*i).next = ptr::null_mut();
    i
}

pub unsafe fn insert_instr_after(after: *mut Instr, i: *mut Instr) {
    (*i).next = (*after).next;
    (*i).last = after;
    (*after).next = i;
    if (*i).next.is_null() {
        LAST_INSTRUCTION = i;
    }
}

pub unsafe fn add_instr(opcode: Opcode) -> *mut Instr {
    let i = create_instr(opcode);
    if LAST_INSTRUCTION.is_null() {
        INSTRUCTIONS = i;
    } else {
        (*LAST_INSTRUCTION).next = i;
        (*i).last = LAST_INSTRUCTION;
    }
    LAST_INSTRUCTION = i;
    i
}

pub unsafe fn add_instr_i(opcode: Opcode, val: i64) -> *mut Instr {
    let i = create_instr(opcode);
    (*i).args[0].i = val;
    if LAST_INSTRUCTION.is_null() {
        INSTRUCTIONS = i;
    } else {
        (*LAST_INSTRUCTION).next = i;
        (*i).last = LAST_INSTRUCTION;
    }
    LAST_INSTRUCTION = i;
    i
}

pub unsafe fn alloc_global(size: usize) -> *mut u8 {
    if N_GLOBALS + size > GLOBAL_SIZE {
        err("insufficient globals space");
    }
    let p = GLOBALS.as_mut_ptr().add(N_GLOBALS);
    N_GLOBALS += size;
    p
}

unsafe fn put_i() {
    println!("#{}", popi());
}

pub unsafe fn run(mut ip: *mut Instr) {
    let mut i_val1: i64;
    let mut i_val2: i64;
    let mut d_val1: f64;
    let mut d_val2: f64;
    let mut a_val1: *mut u8;
    let mut fp: *mut u8 = ptr::null_mut();
    let mut old_sp: *mut u8;
    
    SP = alloc(Layout::from_size_align(STACK_SIZE, 1).unwrap());
    STACK = SP;
    STACK_AFTER = SP.add(STACK_SIZE);

    while !ip.is_null() {
        println!("{:?}/{:?}", ip, SP.offset_from(STACK));

        match (*ip).opcode {
            Opcode::OCall => {
                a_val1 = (*ip).args[0].addr;
                println!("CALL\t{:?}", a_val1);
                pusha((*ip).next as *mut u8);
                ip = a_val1 as *mut Instr;
            }
            Opcode::OCallExt => {
                println!("CALLEXT\t{:?}", (*ip).args[0].addr);
                let func: fn() = std::mem::transmute((*ip).args[0].addr);
                func();
                ip = (*ip).next;
            }
            Opcode::OCastID => {
                i_val1 = popi();
                d_val1 = i_val1 as f64;
                println!("CAST_I_D\t({} -> {})", i_val1, d_val1);
                pushd(d_val1);
                ip = (*ip).next;
            }
            Opcode::ODrop => {
                i_val1 = (*ip).args[0].i;
                println!("DROP\t{}", i_val1);
                if SP.sub(i_val1 as usize) < STACK {
                    err("not enough stack bytes");
                }
                SP = SP.sub(i_val1 as usize);
                ip = (*ip).next;
            }
            Opcode::OEnter => {
                i_val1 = (*ip).args[0].i;
                println!("ENTER\t{}", i_val1);
                pusha(fp);
                fp = SP;
                SP = SP.add(i_val1 as usize);
                ip = (*ip).next;
            }
            Opcode::OEqD => {
                d_val1 = popd();
                d_val2 = popd();
                println!("EQ_D\t({}=={} -> {})", d_val2, d_val1, (d_val2 == d_val1) as i64);
                pushi((d_val2 == d_val1) as i64);
                ip = (*ip).next;
            }
            Opcode::OHalt => {
                println!("HALT");
                break;
            }
            Opcode::OInsert => {
                i_val1 = (*ip).args[0].i;
                i_val2 = (*ip).args[1].i;
                println!("INSERT\t{},{}", i_val1, i_val2);
                if SP.add(i_val2 as usize) > STACK_AFTER {
                    err("out of stack");
                }
                ptr::copy(SP.sub(i_val1 as usize), SP.add(i_val2 as usize).sub(i_val1 as usize), i_val1 as usize);
                ptr::copy(SP.sub(i_val1 as usize), SP.sub(i_val1 as usize).add(i_val2 as usize), i_val2 as usize);
                SP = SP.add(i_val2 as usize);
                ip = (*ip).next;
            }
            Opcode::OJtI => {
                i_val1 = popi();
                println!("JT\t{:?}\t({})", (*ip).args[0].addr, i_val1);
                ip = if i_val1 != 0 {
                    (*ip).args[0].addr as *mut Instr
                } else {
                    (*ip).next
                };
            }
            Opcode::OLoad => {
                i_val1 = (*ip).args[0].i;
                a_val1 = popa();
                println!("LOAD\t{}\t({:?})", i_val1, a_val1);
                if SP.add(i_val1 as usize) > STACK_AFTER {
                    err("out of stack");
                }
                ptr::copy_nonoverlapping(a_val1, SP, i_val1 as usize);
                SP = SP.add(i_val1 as usize);
                ip = (*ip).next;
            }
            Opcode::OOffset => {
                i_val1 = popi();
                a_val1 = popa();
                println!("({}+{} -> {:?})", a_val1 as usize, i_val1, (a_val1 as usize + i_val1 as usize) as *mut u8);
                pusha((a_val1 as usize + i_val1 as usize) as *mut u8);
                ip = (*ip).next;
            }
            Opcode::OPushFpAddr => {
                i_val1 = (*ip).args[0].i;
                println!("PUSHFPADDR\t{}\t({:?})", i_val1, (fp.offset(i_val1 as isize)));
                pusha(fp.offset(i_val1 as isize));
                ip = (*ip).next;
            }
            Opcode::OPushCtA => {
                a_val1 = (*ip).args[0].addr;
                println!("PUSHCT_A\t{:?}", a_val1);
                pusha(a_val1);
                ip = (*ip).next;
            }
            Opcode::OPushCtI => {
                i_val1 = (*ip).args[0].i;
                println!("PUSHCT_I\t{:p}", i_val1 as *const ());
                pushi(i_val1);
                ip = (*ip).next;
            }
            Opcode::ORet => {
                i_val1 = (*ip).args[0].i;
                i_val2 = (*ip).args[1].i;
                println!("RET\t{},{}", i_val1, i_val2);
                old_sp = SP;
                SP = fp;
                fp = popa();
                ip = popa() as *mut Instr;
                if SP.sub(i_val1 as usize) < STACK {
                    err("not enough stack bytes");
                }
                SP = SP.sub(i_val1 as usize);
                ptr::copy(SP, old_sp.sub(i_val2 as usize), i_val2 as usize);
                SP = SP.add(i_val2 as usize);
            }
            Opcode::OStore => {
                i_val1 = (*ip).args[0].i;
                if SP.sub(size_of::<*mut u8>().wrapping_add(i_val1 as usize)) < STACK {
                    err("not enough stack bytes for SET");
                }
                a_val1 = *(SP.sub(size_of::<*mut u8>().wrapping_add(i_val1 as usize)) as *mut *mut u8);
                println!("STORE\t{}\t({:?})", i_val1, a_val1);
                ptr::copy_nonoverlapping(SP.sub(i_val1 as usize), a_val1, i_val1 as usize);
                SP = SP.sub(size_of::<*mut u8>().wrapping_add(i_val1 as usize));
                ip = (*ip).next;
            }
            Opcode::OSubD => {
                d_val1 = popd();
                d_val2 = popd();
                println!("SUB_D\t({}-{} -> {})", d_val2, d_val1, d_val2 - d_val1);
                pushd(d_val2 - d_val1);
                ip = (*ip).next;
            }
            Opcode::OAddI => {
                d_val1 = popi() as f64;
                d_val2 = popi() as f64;
                println!("ADD_I\t({}+{} -> {})", d_val2, d_val1, d_val2 + d_val1);
                pushd(d_val2 + d_val1);
                ip = (*ip).next;
            }
            _ => {
                err(&format!("invalid opcode: {:?}", (*ip).opcode));
            }
        }
    }
}
