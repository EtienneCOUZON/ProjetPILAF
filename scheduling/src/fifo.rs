/*use std::collections::VecDeque;
use super::Task;
use super::SchedulingPolicy;
struct FifoScheduler{
   client:VecDeque<Task>
}
impl FifoScheduler{
   pub fn new()->Self{
       Self{
           client: VecDeque::new()
       }
   }
}
impl SchedulingPolicy for FifoScheduler{
   fn register(&mut self, task:Task){
       self.client.push_back(task);
   }
   fn take(&mut self) -> Option<Task>{
        
       return self.client.pop_front();
  
   }
}
#[cfg(test)]
mod tests {
   use super::FifoScheduler;
   use super::super::{SchedulingPolicy, Task};
 
   #[test]
   fn test_fifo(){
       let mut fifo_scheduler=FifoScheduler::new();
       let taskA=Task{
           id:"A",
           duration:5,
           deadline:10
       };
       let taskB=Task{
           id:"B",
           duration:3,
           deadline:9
       };
       let taskC=Task{
           id:"C",
           duration:2,
           deadline:8
       };
       let taskD=Task{
           id:"D",
           duration:1,
           deadline:12
       };
       fifo_scheduler.register(taskA.clone());
       assert_eq!(fifo_scheduler.take(),Some (taskA));
       fifo_scheduler.register(taskB.clone());
       fifo_scheduler.register(taskC.clone());
       fifo_scheduler.register(taskD.clone());
       assert_eq!(fifo_scheduler.take(), Some (taskB));
       assert_eq!(fifo_scheduler.take(), Some (taskC));
       assert_eq!(fifo_scheduler.take(), Some (taskD));
       assert_eq!(fifo_scheduler.take(), None);
 
 
 
 
   }
 
}
*/