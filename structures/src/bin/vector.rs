fn heapsort(vector:Vec<i32>) -> Vec<i32>{
    let mut vec = vector;
    vec.sort();
    for i in vec.iter(){
        println!("{}",i);
    }
    return vec;

}

fn main(){
let vec =vec![3,5,4];
heapsort(vec);
}











































/*fn main(){
play_with_vector();
}
fn play_with_vector(){
    let my_vec=vec![3,1,2,4];
    println!("{:?}",my_vec);
}*/