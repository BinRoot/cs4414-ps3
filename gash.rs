//
// gash.rs
//
// Reference solution for PS2
// Running on Rust 0.8
//
// Special thanks to Kiet Tran for porting code from Rust 0.7 to Rust 0.8.
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu, Purnam Jantrania, David Evans
// Version 0.2
//

use std::{io, run, os, path, libc};
use std::task;
use std::os::args;

fn get_fd(fpath: &str, mode: &str) -> libc::c_int {
    #[fixed_stack_segment]; #[inline(never)];

    unsafe {
        let fpathbuf = fpath.to_c_str().unwrap();
        let modebuf = mode.to_c_str().unwrap();
        return libc::fileno(libc::fopen(fpathbuf, modebuf));
    }
}

fn exit(status: libc::c_int) {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe { libc::exit(status); }
}

fn handle_cmd(cmd_line: &str, pipe_in: libc::c_int, pipe_out: libc::c_int, pipe_err: libc::c_int) {
    let mut out_fd = pipe_out;
    let mut in_fd = pipe_in;
    let err_fd = pipe_err;
    
    let mut argv: ~[~str] =
        cmd_line.split_iter(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    let mut i = 0;
    // found problem on redirection
    // `ping google.com | grep 1 > ping.txt &` didn't work
    // because grep won't flush the buffer until terminated (only) by SIGINT.
    while (i < argv.len()) {
        if (argv[i] == ~">") {
            argv.remove(i);
            out_fd = get_fd(argv.remove(i), "w");
        } else if (argv[i] == ~"<") {
            argv.remove(i);
            in_fd = get_fd(argv.remove(i), "r");
        }
        i += 1;
    }
    
    if argv.len() > 0 {
        let program = argv.remove(0);
        match program {
            ~"help"     => {println("This is a new shell implemented in Rust!")}
            ~"cd"       => {if argv.len()>0 {os::change_dir(&path::PosixPath(argv[0]));}}
            //global variable?
            //~"history"  => {for i in range(0, history.len()) {println(fmt!("%5u %s", i+1, history[i]));}}
            ~"exit"     => {exit(0);}
            _           => {let mut prog = run::Process::new(program, argv, run::ProcessOptions {
                                                                                        env: None,
                                                                                        dir: None,
                                                                                        in_fd: Some(in_fd),
                                                                                        out_fd: Some(out_fd),
                                                                                        err_fd: Some(err_fd)
                                                                                    });
                             prog.finish();
                             // close the pipes after process terminates.
                             if in_fd != 0 {os::close(in_fd);}
                             if out_fd != 1 {os::close(out_fd);}
                             if err_fd != 2 {os::close(err_fd);}
                            }
        }//match 
    }//if
}

fn handle_cmdline(cmd_line:&str, bg_flag:bool)
{
    // handle pipes
    let progs: ~[~str] =
        cmd_line.split_iter('|').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    
    let mut pipes: ~[os::Pipe] = ~[];
    
    // create pipes
    if (progs.len() > 1) {
        for _ in range(0, progs.len()-1) {
            pipes.push(os::pipe());
        }
    }
        
    if progs.len() == 1 {
        if bg_flag == false { handle_cmd(progs[0], 0, 1, 2); }
        else {task::spawn_sched(task::SingleThreaded, ||{handle_cmd(progs[0], 0, 1, 2)});}
    } else {
        for i in range(0, progs.len()) {
            let prog = progs[i].to_owned();
            
            if i == 0 {
                let pipe_i = pipes[i];
                task::spawn_sched(task::SingleThreaded, ||{handle_cmd(prog, 0, pipe_i.out, 2)});
            } else if i == progs.len() - 1 {
                let pipe_i_1 = pipes[i-1];
                if bg_flag == true {
                    task::spawn_sched(task::SingleThreaded, ||{handle_cmd(prog, pipe_i_1.input, 1, 2)});
                } else {
                    handle_cmd(prog, pipe_i_1.input, 1, 2);
                }
            } else {
                let pipe_i = pipes[i];
                let pipe_i_1 = pipes[i-1];
                task::spawn_sched(task::SingleThreaded, ||{handle_cmd(prog, pipe_i_1.input, pipe_i.out, 2)});
            }
        }
    }
}

fn main() {

    if args().len() > 1 {
        let mut argv = args().clone();
        argv.remove(0);
        handle_cmdline(argv.connect(" "), false);
        return;
    }

    static CMD_PROMPT: &'static str = "gash > ";
    let mut history: ~[~str] = ~[];
    
    loop {
        print(CMD_PROMPT);
        
        let mut cmd_line = io::stdin().read_line();
        cmd_line = cmd_line.trim().to_owned();
        if cmd_line.len() > 0 {
            history.push(cmd_line.to_owned());
        }
        let mut bg_flag = false;
        if cmd_line.ends_with("&") {
            cmd_line = cmd_line.trim_right_chars(&'&').to_owned();
            bg_flag = true;
        }
        
        if cmd_line == ~"exit" {
            break;
        } else if cmd_line == ~"history" {
            for i in range(0, history.len()) {
                println(fmt!("%5u %s", i+1, history[i]));
            }
        } else {
            handle_cmdline(cmd_line, bg_flag);
        }
    }
}
