extern crate crossbeam;
extern crate reqwest;

use crossbeam::channel;
use std::thread;
use reqwest;
use std::collections::VecDeque;

struct 


struct ProxyMiddleware {
    new: Vec<reqwest::Proxy>,
    good: Vec<reqwest::Proxy>,
    bad: Vec<reqwest::Proxy>,
    hz: Vec<reqwest::Proxy>,
}

impl ProxyMiddleware {
    fn new(new: Vec<reqwest::Proxy>) -> ProxyMiddleware {
        
    }
}

type Callback = fn();

struct Rqwst {
    url: String,
    //client: Option<reqwest::Client>, // create empty and then add Client
    cb: Option<Callback>,
}

impl Rqwst {
    fn new(url: &str) -> Rqwst {
        Rqwst{
            url,
            //client: None,
            cb: None,
        }
    }

    fn add_callback(&mut self, cb: Callback) -> Rqwst {
        self.cb = cb;
        return self
 
   // fn add_client(&mut self, client: reqwest::Client) -> Rqwst {
   //     self.client = client;
   //     return self
   // }

}

struct Pool {
    pool: VecDeque<Rqwst>,
}

impl Pool {
    fn new(pool: VecDeque<Rqwst>) -> Pool {
        Pool {pool}
    }


    fn empty() -> Pool {
        let pool: VecDeque<Rqwst> = VecDeque::new();
        Pool {pool}
    }

}

struct Spider{
    pool: Pool,
}

impl Spider {
    fn new() -> Spider {
        Spider { pool: Pool::new() }
    }
}

fn main(){
    //let (tx, rx) = channel::unbounded();
    
    let (tx, rx) = channel::bounded(3);

    let work = thread::spawn(move || {
       println!("Starting thread"); 
       
       // This will print out:
       // 1
       // 2
       while let Some(msg) = rx.recv().unwrap() {
           println!("item: {:?}", msg);
       }
    }); 
       
    //procs.push(thread::spawn(move || {
    println!("Sending 1");
    tx.send(Some("rofl"));
    println!("Sending 2");
    tx.send(Some("lol"));
    //}));
    println!("starting counter");
    let mut c = 0;
    for i in 0..100000{
        c+=1;
    }
    println!("Sending Non3");
    tx.send(None);

    work.join();
//    
//    println!("stoping counter");
    // Explicitly `drop` this sender allowing the iterator to close.
    //drop(tx);
   // for proc in procs {
   //     let _ = proc.join();
   // }

    //println!("Closing");
}
