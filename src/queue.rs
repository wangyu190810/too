
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

fn test(){
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
