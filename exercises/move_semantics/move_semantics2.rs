// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


// fn main() {
//     let vec0 = Vec::new();

//     let vec_temp = &vec0;
//     let mut vec1 = fill_vec(vec_temp);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }


fn main() {
    let mut vec = Vec::new();
    let mut temp = &mut vec;
    // println!("{}", temp.len());
    // println!("{}", vec.len());
    test(temp);
    let temp1 = &vec;
    println!("{}", vec.len());
    // println!("{}", temp.len());
    

}

fn test(vec: &mut Vec<i32>) {
    vec.push(1);
}