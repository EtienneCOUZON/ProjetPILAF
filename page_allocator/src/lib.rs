use std::collections::VecDeque;
struct PageAllocator{
   page_libre:Vec<bool>,
   capacity:usize,
   truetab:VecDeque<usize>
}
impl PageAllocator {
   pub fn new(capacity: usize)-> Self{
       Self{
           page_libre: vec![true;capacity],
           capacity:capacity,
           truetab:(0..capacity).collect(),
           }
   }
   pub fn allocate(&mut self) -> Option<usize>{
       if let Some(i)= self.truetab.pop_front(){
           self.page_libre[i]=false;
           return Some(i);
       }
       else{
       return None;
       }
   }
   pub fn free(&mut self , page:usize){
       if page<self.capacity&&!self.page_libre[page]{
           self.page_libre[page]=true;   
       }   
   }
   fn to_string(&self)-> String{
       let mut chaine=String::new();           
       for i in 0..self.capacity{
           if self.page_libre[i]
           {
               chaine=chaine+"-";
           }
           else {
               chaine=chaine+"x";
           }
       }
       return chaine;
   }
#[cfg(test)]
mod tests {
    use super::PageAllocator;
    #[test]
    fn it_works() {
        let mut allocator=PageAllocator::new(3);
        assert_eq!(allocator.to_string(), "---");
        allocator.allocate();
        assert_eq!(allocator.to_string(), "x--");
        allocator.allocate();
        assert_eq!(allocator.to_string(), "xx-");
        allocator.free(0);
        assert_eq!(allocator.to_string(), "-x-");
        allocator.allocate();
        assert_eq!(allocator.to_string(), "xx-");
        allocator.allocate();
        assert_eq!(allocator.to_string(), "xxx");
        allocator.free(2);
        assert_eq!(allocator.to_string(), "xx-");
        allocator.allocate();
        allocator.allocate();
        allocator.allocate();
        assert_eq!(allocator.to_string(), "xxx");    
    }
}
}