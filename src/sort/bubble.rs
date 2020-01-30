/* rust
排序算法 - 冒泡排序 Bubble Sort
这种排序方法的主要逻辑是相邻位元素的大小比对和交换
一轮比对结束后比对第下一轮，遂两层for循环
vectors: [7, 21, 9, 13, 109, 9, 2, 50, 33, 39, 20, 11]
results: [2, 7, 9, 9, 11, 13, 20, 21, 33, 39, 50, 109]
*/


fn bubble_sort(vectors: &mut Vec<i32>) -> &Vec<i32>{
    // 多少个元素就比对多少轮
    for i in 0..vectors.len(){
        // 划定边界
        for j in 0..vectors.len()-i-1{
            // 第i位与下一位进行比较
            if vectors[j] > vectors[j+1]{
                // 符合条件时借助第三个元素交换值
                let middle = vectors[j];
                vectors[j] = vectors[j+1];
                vectors[j+1] = middle;
            }
        }
    }
    vectors
}


fn main() {
    let mut vectors = vec![7, 21, 9, 13, 109, 9, 2, 50, 33, 39, 20, 11];
    println!("vectors: {:?}", vectors);
    let results = bubble_sort(&mut vectors);
    println!("results: {:?}", results);

}




