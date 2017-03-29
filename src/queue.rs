

#[derive(Debug)]
pub struct queueData{
    pub data: Vec<String>,
    pub size: usize
}

impl queueData{

    pub fn new(size: usize) -> queueData{
        queueData{
            size: size,
            data: vec![],
        }  
    }

    pub fn put(&mut self, value: String) -> bool{
        let mut flag = false;
        if self.data.len() < self.size{
            self.data.push(value);
            flag = true;   
        }
        return flag
    }

    pub fn get(&mut self) -> Option<String>{
        self.data.pop()
    }
}

pub fn base_test(){
    let mut queue =  queueData::new(2);
    let world = String::from("world");
    for row in 0..4{
        if queue.put(world.clone()){
            println!("put success!")
        }else {
            println!("put false ! queue size is {}, queue len is {} ", queue.size, queue.data.len());
        }
    }
    for row in 0..queue.size{
        let get_world = queue.get();
        println!("{:?}", get_world);
    }   
}

pub fn thread_test(){
    use std::thread;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};
    // use std::sync::mpsc;
    // let mut  'a base_queue = Box::new(queueData::new(2));
    let mut queue =  Arc::new(Mutex::new( queueData::new(2)));
    let (tx, rx) = channel();
    
    for row in 0..3{
        // 需要调用lock之后，相当于对T做了lock，这个时候就可以调用T的方法了。
        let (queue, tx) = (queue.clone(), tx.clone());
        thread::spawn(move || {
            let world = String::from("world");
            let mut queue = queue.lock().unwrap();
            tx.send(queue.put(world)).unwrap();
        //    queue.get();
        });
    }
    for row in 0..3{
        // rx.recv().unwrap();
        // 同样的操纵，需要调用lock之后，相当于对T做了lock，这个时候就可以调用T的方法了，
        //如果不做lock，那样T就不是我们定义的struct。
        println!("{}", rx.recv().unwrap());
        let (queue, tx) = (queue.clone(), tx.clone());
        let mut queue = queue.lock().unwrap();
        queue.get();
    }
    // base_queue.get();
    // let mut queue: = 
    // let mut queue =  rx.recv(queue.get()).unwrap();
    // println!("{:?}");
    // let get_world = queue.get();
    // println!("{:?}", get_world);
}