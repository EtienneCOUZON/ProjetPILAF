/*use priority_queue::PriorityQueue;
fn main(){
    play_with_priority_queue();
    play_with_min_priority_queue();
}
fn play_with_priority_queue(){
    let mut my_priority_queue=PriorityQueue::new();
    my_priority_queue.push(3,3);
    my_priority_queue.push(1,1);
    my_priority_queue.push(2,2);
    my_priority_queue.push(4,4);
    for (i,_) in my_priority_queue.into_iter() {
        println!("{}",i);
    }
}
fn play_with_mine_priority_queue(){
    for (_,i) in my priority_queue

}*/





























fn play_with_min_priority_queue(){
    let mut my_priority_queue=PriorityQueue::new();
    my_priority_queue.push(3,-3);
    my_priority_queue.push(1,-1);
    my_priority_queue.push(2,-2);
    my_priority_queue.push(4,-4);
    while! my_priority_queue.is_empty(){
        if let Some((element,_)) = my_priority_queue.pop(){
            println!("{}",element);
        
        }
    }
}