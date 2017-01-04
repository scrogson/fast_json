use std::thread;
use std::mem;
use erlang_nif_sys::{self, ErlNifPid};
use rustler::{NifEnv, NifTerm};

pub fn spawn_thread_and_send_value_back_to_self<'a, F>(env: &'a NifEnv, thread_fn: F)
    where F: for<'b> FnOnce(&'b NifEnv) -> NifTerm<'b> + Send + 'static
{
    let mut pid: ErlNifPid = unsafe { mem::uninitialized() };
    unsafe {
        erlang_nif_sys::enif_self(env.as_c_arg(), &mut pid);
    }

    thread::spawn(move || {
        let env = NifEnv::new(unsafe { erlang_nif_sys::enif_alloc_env() });
        let msg = thread_fn(&env);
        unsafe {
            erlang_nif_sys::enif_send(env.as_c_arg(),
                                      &pid,
                                      env.as_c_arg(),
                                      msg.as_c_arg());
        }
    });
}
