// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE



// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(&vec0);

//     println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }


fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

    let vec1 = &mut vec0;
    // println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
    fill_vec(vec1);
    vec1.push(88);

    //要是注掉上面的一行，执行下面的一行，会报错。说明同一时间，只有一个可变存储空间，无论是否借用。还有就是借用不会被销毁
    //vec0.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>){
    vec.push(22);
    vec.push(44);
    vec.push(66);
}