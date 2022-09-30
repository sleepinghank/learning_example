///
/// ### 接雨水问题
/// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
/// > 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// > 输出：6
/// > 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
///

fn main() {
    let t = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!("{}",trap01(t));
}

/// 解题思路一
/// 单调栈解法
pub fn trap01(height: Vec<i32>) -> i32{
    let mut stack = vec![];
    let mut result = 0;
    for i in 1..height.len() {
        // 栈顶有值，并且当前值比栈顶高
        while !stack.is_empty()&& (height[i] > height[stack[stack.len() -1]]) {
            let mid = stack.pop().unwrap();
            if !stack.is_empty() {
                let h = height[i].min(height[stack[stack.len() -1]]) - height[mid];
                let w = i - stack[stack.len() -1] - 1;
                result += h * (w as i32);
            }
        }
        stack.push(i);
    }
    return result;
}