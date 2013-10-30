
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
use std::hashmap::HashMap;

struct sched_msg {
    
    ip: IpAddr,
    filesize: Option<uint>, //filesize added to store file size
}

fn main() {

let mut opt1 : Option<uint> = Some(10u);
let mut opt2 : Option<uint> = Some(3u);
let mut opt3 : Option<uint> = Some(20u);

let mut local: IpAddr = std::rt::io::net::ip::Ipv4Addr(128, 143, -1, -1);
let mut local2: IpAddr = std::rt::io::net::ip::Ipv4Addr(137, 54, -1, -1);
let mut other: IpAddr = std::rt::io::net::ip::Ipv4Addr(-1, -1, -1, -1);

let msg: sched_msg = sched_msg{ip: local, filesize : opt1 };
let msg2: sched_msg = sched_msg{ ip: other, filesize: opt2};
let msg3: sched_msg = sched_msg{ip: local2, filesize: opt3};

let mut req_vec : PriorityQueue<sched_msg> = PriorityQueue::new();

req_vec.push(msg);
req_vec.push(msg2);
req_vec.push(msg3);

println(req_vec.pop().filesize.to_str());
println(req_vec.pop().filesize.to_str());
println(req_vec.pop().filesize.to_str());




}


impl Ord for sched_msg {

	fn lt(&self, other: &sched_msg) -> bool {
	
		let selfIP: IpAddr = self.ip;
		let otherIP: IpAddr = other.ip;

		let selfSize : Option<uint> = self.filesize;
		let mut sSize: uint = 0;
		match selfSize{
			Some(i) => {
				sSize=i;},
			None =>  {return true;}
		}

		let otherSize : Option<uint> = other.filesize;
		let mut oSize: uint = 0;
		match otherSize{
			Some(k) => {
				oSize=k;},
			None =>  {return true;}
		}

		let mut sIP : bool = false;
		let mut oIP : bool = false;

		match selfIP {
			Ipv4Addr(a , b, c, d) => {
				if ((a == 128 && b == 143) || (a == 137 && b == 54)){
					sIP = true;
				}
			},
			Ipv6Addr(a, b, c, d, e, f, g, h) => {
				if ((a == 128 && b == 143) || (a == 137 && b == 54)){
					sIP = true;
				}
				
			}
		}
		
		match otherIP {
			Ipv4Addr(a , b, c, d) => {
				if ((a == 128 && b == 143) || (a == 137 && b == 54)){
					oIP = true;
				}
			},
			Ipv6Addr(a, b, c, d, e, f, g, h) => {
				if ((a == 128 && b == 143) || (a == 137 && b == 54)){
					oIP = true;
				}
				
			}
		}
		if(sIP && oIP){
			if(sSize < oSize){
				return false;
			}else {
				return true;
			}
		}else if(sIP){
			return false;
		}else if (oIP){
			return true;
		}else if(sSize < oSize){
			return false;
		}else {
			return true;
		}
		
	}
}
