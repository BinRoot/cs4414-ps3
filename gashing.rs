use std::{run, str};

pub fn gashify(html: ~str) -> ~str {
    println(html);
    let output: ~str = ~"";

    let mut quote_iter = html.matches_index_iter("\"");
    let mut more_exists = true;
    let mut qvec: ~[uint] = ~[];
    while more_exists {
        match quote_iter.next() {
            Some ( (quote_start, quote_end) ) => {
                if html.len()>quote_start && html.char_at(quote_start-1)!='\\' {
                    qvec.push(quote_start);
                }
            },
            None => {
                more_exists = false;
            }
        }
    }

    println("qvec: "+qvec.to_str());

    let mut m_iter = html.matches_index_iter("<!--#exec cmd=\"");

    more_exists = true;

    let mut cur_index: int = -1;

    while more_exists {
        match m_iter.next() {
            Some( (start, end) ) => {
                println("start: "+start.to_str());
                if(start as int > cur_index) {
                    println(start.to_str()+" "+end.to_str());
                    let mut end_quote = -1;
                    for qval in qvec.iter() {
                        if qval > &end {
                            end_quote = qval.clone();
                            break;
                        }
                    }
                    if end_quote != -1 {
                        let cmd = html.to_managed().slice_chars(end, end_quote).replace("\\\"", "\"").replace("\\\\","\\");
                        println("found cmd: " + cmd);
                        println("exec cmd: " + gash(cmd));
                        cur_index = end_quote as int;
                    }
                }
            },
            None => {
                more_exists = false;
            }
        }
    }

    output
}

fn gash(cmd: ~str) -> ~str {
    println("running "+cmd);
    let output = str::from_utf8(run::process_output(cmd, []).output);
    output
}