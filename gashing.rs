use std::{run, str};
use std::hashmap::HashMap;

pub fn gashify(html: ~str) -> ~str {

    // Record all indices of unescaped quotation marks
    let mut quote_iter = html.matches_index_iter("\"");
    let mut more_exists = true;
    let mut qvec: ~[uint] = ~[];
    while more_exists {
        match quote_iter.next() {
            Some ( (quote_start, _) ) => {
                if html.len()>quote_start && html.char_at(quote_start-1)!='\\' {
                    qvec.push(quote_start);
                }
            },
            None => more_exists = false
        }
    }

    // Record all indices of "-->" tokens
    let mut endtoken_iter = html.matches_index_iter("-->");
    more_exists = true;
    let mut evec: ~[uint] = ~[];
    while more_exists {
        match endtoken_iter.next() {
            Some ( (_, e_end) ) => {
                evec.push(e_end);
            },
            None => {
                more_exists = false;
            }
        }
    }

    // Iterate though all matches of the starting token
    let mut m_iter = html.matches_index_iter("<!--#exec cmd=\"");

    more_exists = true;
    let mut cur_index: int = -1;
    let mut replace_map: HashMap<~str, ~str> = HashMap::new();

    while more_exists {
        match m_iter.next() {
            Some( (start, end) ) => {
//                println("start: "+start.to_str());
                if(start as int > cur_index) {
//                    println(start.to_str()+" "+end.to_str());
                    let mut end_quote = -1;
                    for qval in qvec.iter() {
                        if qval > &end {
                            end_quote = qval.clone();
                            break;
                        }
                    }
                    if end_quote != -1 {
                        let cmd = html.to_managed()
                            .slice_chars(end, end_quote)
                            .replace("\\\"", "\"").replace("\\\\","\\");
                        let exec_token = html.slice(start, end_quote);
                        
                        let mut gash_comment: ~str = exec_token.to_owned();

                        // Find next index of a "-->" token
                        for endtoken_end_val in evec.iter() {
                            if endtoken_end_val > &end_quote {
                                gash_comment.push_str( 
                                    html.slice(
                                        end_quote.clone(), 
                                        endtoken_end_val.clone()) );
                                break;
                            }
                        }

                        // Insert new key/val pair to a hashmap
                        replace_map.find_or_insert(gash_comment, gash(cmd));
                        cur_index = end_quote as int;
                    }
                }
            },
            None => more_exists = false
        }
    }

    // Go thought the hashmap, and replace each key with its value
    let mut output = html.clone();
    for keyval in replace_map.iter() {
        match keyval { 
            (a,b) => output = output.replace(*a, *b) 
        }
    }
    
    output
}

// Invoke gash
fn gash(cmd: ~str) -> ~str {
    let argv: ~[~str] = cmd.split_iter(' ').filter(|&x| x != "")
            .map(|x| x.to_owned()).collect();
    let mut pr = run::Process::new("./gash", argv, run::ProcessOptions::new());
    let poutput = pr.finish_with_output();
    let poutputc = poutput.output;
    let realstr: ~str = str::from_utf8(poutputc);

    realstr.trim().to_owned()
}