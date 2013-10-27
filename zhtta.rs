//
// zhtta.rs
//
// Running on Rust 0.8
//
// Starting code for PS3
// 
// Note: it would be very unwise to run this server on a machine that is
// on the Internet and contains any sensitive files!
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.3

extern mod extra;

use std::rt::io::*;
use std::rt::io::net::ip::SocketAddr;
use std::io::println;
use std::cell::Cell;
use std::{os, str, io, run, uint};
use extra::arc;
use std::comm::*;
use extra::priority_queue::PriorityQueue;
use std::rt::io::net::ip::*;

static PORT:    int = 4414;
static IP: &'static str = "127.0.0.1";

struct sched_msg {
    stream: Option<std::rt::io::net::tcp::TcpStream>,
    filepath: ~std::path::PosixPath,
    ip: IpAddr,
    filesize: Option<uint>, //filesize added to store file size
}

fn main() {
    let mut req_vec : PriorityQueue<sched_msg> = PriorityQueue::new();
    let shared_req_vec = arc::RWArc::new(req_vec);
    let add_vec = shared_req_vec.clone();
    let take_vec = shared_req_vec.clone();
    
    let (port, chan) = stream();
    let chan = SharedChan::new(chan);
    
    // dequeue file requests, and send responses.
    // FIFO
    do spawn {
        let (sm_port, sm_chan) = stream();
        
        // a task for sending responses.
        do spawn {
            loop {
                let mut tf: sched_msg = sm_port.recv(); // wait for the dequeued request to handle
                match io::read_whole_file(tf.filepath) { // killed if file size is larger than memory size.
                    Ok(file_data) => {
                        println(fmt!("begin serving file [%?]", tf.filepath));
                        // A web server should always reply a HTTP header for any legal HTTP request.
                        tf.stream.write("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream; charset=UTF-8\r\n\r\n".as_bytes());
                        tf.stream.write(file_data);
                        println(fmt!("finish file [%?]", tf.filepath));
                    }
                    Err(err) => {
                        println(err);
                    }
                }
            }
        }
        
        loop {
            port.recv(); // wait for arrving notification
            do take_vec.write |vec| {
                if ((*vec).len() > 0) {
                    // LIFO didn't make sense in service scheduling, so we modify it as FIFO by using shift_opt() rather than pop().
                    let tf : sched_msg = (*vec).pop();
                    println(fmt!("shift from queue, size: %ud", (*vec).len()));
                    sm_chan.send(tf); // send the request to send-response-task to serve.
                }
            }
        }
    }

    let ip = match FromStr::from_str(IP) { Some(ip) => ip, 
                                           None => { println(fmt!("Error: Invalid IP address <%s>", IP));
                                                     return;},
                                         };
                                         
    let socket = net::tcp::TcpListener::bind(SocketAddr {ip: ip, port: PORT as u16});
    
    println(fmt!("Listening on %s:%d ...", ip.to_str(), PORT));
    let mut acceptor = socket.listen().unwrap();
    
    let mycount: uint = 0;
    let shared_mycount = arc::RWArc::new(mycount);

    for stream in acceptor.incoming() {
        let stream = Cell::new(stream);

        // Start a new task to handle the each connection
        let child_chan = chan.clone();
        let child_add_vec = add_vec.clone();
        let update_count = shared_mycount.clone();
        do spawn {
            let mut visitor_count = 0;
            do update_count.write |c| {
                *c += 1;
                visitor_count = *c;
             }
            
            let mut stream = stream.take();

            let mut visitor_ip: IpAddr = std::rt::io::net::ip::Ipv4Addr(-1, -1, -1, -1);

            stream.and_then_mut_ref(|x| {
                    match x.peer_name() {
                        Some(sock_addr) => {visitor_ip = sock_addr.ip;},
                        None => ()
                    }
                    Some(x)
                });

            println(fmt!("Visitor IP is %s", visitor_ip.to_str()));
		


            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            
            let req_group : ~[&str]= request_str.splitn_iter(' ', 3).collect();
            if req_group.len() > 2 {
                let path = req_group[1];
                println(fmt!("Request for path: \n%?", path));
                
                let file_path = ~os::getcwd().push(path.replace("/../", ""));
                if !os::path_exists(file_path) || os::path_is_dir(file_path) {
                    println(fmt!("Request received:\n%s", request_str));
                    let response: ~str = fmt!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>Hello, Rust!</title>
                         <style>body { background-color: #111; color: #FFEEAA }
                                h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                         </style></head>
                         <body>
                         <h1>Greetings, Krusty!</h1>
                         <h2>Visitor count: %u</h2>
                         </body></html>\r\n", visitor_count);

                    stream.write(response.as_bytes());
                }
                else {
                    // Requests scheduling

		    // Started Problem 3 code here
		    // Creates process that runs and parses file size command for problem 3
		    let mut args : ~[~str] = ~[~"-c", ~"<", file_path.to_str()];
		    let mut pr = run::Process::new("wc", args, run::ProcessOptions::new());
		    let poutput = pr.finish_with_output();
		    let mut poutputc = poutput.output;
		    let mut realstr = str::from_utf8(poutputc);
		    let mut strarray: ~[&str] = realstr.split_iter(' ').collect();
		    let mut formatfsize : Option<uint> = from_str(strarray[0]);
                    let msg: sched_msg = sched_msg{stream: stream, filepath: file_path.clone(), ip: visitor_ip, filesize : formatfsize };

		    println(fmt!("%?", realstr));//print file size served
		    println(fmt!("%?", strarray[0]));//print file size served
		    println(fmt!("%?", formatfsize));//print file size served

		    //Problem 3 Code finishes here

			
                    let (sm_port, sm_chan) = std::comm::stream();
                    sm_chan.send(msg);
                    
                    do child_add_vec.write |vec| {
                        let msg = sm_port.recv();
                        (*vec).push(msg); // enqueue new request.
                        println("add to queue");
                    }
                    child_chan.send(""); //notify the new arriving request.
                    println(fmt!("get file request: %?", file_path));
                }
            }
            println!("connection terminates")
        }
    }
}

impl Ord for sched_msg {

	fn lt(&self, other: &sched_msg) -> bool {
	
		let selfIP: IpAddr = self.ip;
		let otherIP: IpAddr = other.ip;

		match selfIP {
			Ipv4Addr(a , b, c, d) => {
				if ((a == 128 && b == 143) || (a == 137 && b == 54)){
					return false;
				}else{
					return true;;
				}
			},
			Ipv6Addr(a, b, c, d, e, f, g, h) => {
				if ((a == 128 && b == 143) || (a == 137 && b == 54)){
					return false;
				}else{
					return true;
				}
				
			}
		}

		
	}

}
