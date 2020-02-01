/* rust
排序算法 - 选择排序
这种排序方法的主要逻辑为
外循环先指定一个数，通常是第一个数
接着在内循环中将这个外循环指定数与右侧数逐个比较，记录下右侧数当中最小的那位
左侧为排序好的元素，遂内循环中的比较是外循环指定数不停地与右侧元素进行比较
内循环时，每次发现比外循环指定数小的数就记录下来，循环结束后记录的数便是右侧最小数
在内循环结束后将外循环指定数与内循环得出的右侧最小数交换位置
以此类推，直到外层循环结束
vectors: [7, 21, 9, 13, 109, 9, 2, 50, 33, -1, 20, 11]
results: [-1, 2, 7, 9, 9, 11, 13, 20, 21, 33, 50, 109]
*/

fn selection_sort(vectors: &mut Vec<i32>) -> &Vec<i32>{
    for i in 0..vectors.len(){
        // 寻找未排序元素中的最小值，即[i, n)中的最小元素
        let mut index = i;
        let mut current = i;
        for j in i+1..vectors.len(){
            if vectors[j] < vectors[index]{
                index = j;
                current = j;
            }
        }
        let middle = vectors[current];
        vectors[current] = vectors[i];
        vectors[i] = middle;
    }
    vectors
}


fn main() {
    let mut vectors = vec![7, 21, 9, 13, 109, 9, 2, 50, 33, -1, 20, 11];
    println!("vectors: {:?}", vectors);
    let results = selection_sort(&mut vectors);
    println!("results: {:?}", results);

}




