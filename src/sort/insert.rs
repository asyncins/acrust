/* Rust Programming Language
排序算法 - 插入排序
这种排序方法的主要逻辑为
外循环先指定一个数，通常是第一个数
接着在内循环中将这个外循环指定数与左侧数逐个比较，并根据比较结果将外循环指定数插入在内循环数的左边或不动
左侧为排序好的元素，遂内循环中的比较是外循环指定数不停地与左侧元素进行比较
当外循环指定数小于左侧元素时交换位置，否则不动
以此类推，直到外层循环结束
vectors: [7, 21, 9, 13, 109, 9, 2, 50, 33, -1, 20, 11]
results: [-1, 2, 7, 9, 9, 11, 13, 20, 21, 33, 50, 109]
*/

fn insert_sort(vectors: &mut Vec<i32>) -> &Vec<i32>{
    for i in 1..vectors.len(){
        let mut current = vectors[i];
        let mut j = i - 1;
        while j >= 0 && current < vectors[j]{
            let middle = vectors[j+1];
            vectors[j+1] = vectors[j];
            vectors[j] = middle;
            if j > 0{
                /* rust 不允许while j >=0 中 j = 0 时还减 1
                导致 j 在 while 中为负数这种危险写法*/
                j = j - 1;  // j 递减即不断地跟左边比较
            }
        }
    }
    vectors
}


fn main() {
    let mut vectors = vec![7, 21, 9, 13, 109, 9, 2, 50, 33, -1, 20, 11];
    println!("vectors: {:?}", vectors);
    let results = insert_sort(&mut vectors);
    println!("results: {:?}", results);
}




