/// print time result in md table
pub fn format_result(mode: String, call_num: i32, time_in_ms: i64) -> String {
    format!("###{}
|  total time | time per call |
|  ---------  | ------------- |
|  {} ms  |  {} us  |", mode, time_in_ms, (time_in_ms as f64 / call_num as f64) * 1000 as f64)
}

/// print config result in md table
pub fn format_config(thread_num: i32, loop_num: i32) -> String {
    format!("###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      {}      |    {}    |    {}    |",
             format_i32(thread_num), format_i32(loop_num), format_i32(thread_num * loop_num))
}

/// format i32 for human
/// # Examples
///
/// ```no_run
/// let i = 1000000;
/// assert_eq!("1_00_000", format_i32(i))
/// ```
fn format_i32(mut i: i32) -> String {
    let mut res = String::new();
    let mut count = 0;
    while i > 0 {
        if count == 3 {
            res.insert_str(0, "_");
            count = 0;
        }
        count += 1;
        let last = i % 10;
        i /= 10;
        res.insert_str(0, last.to_string().as_str());
    }

    res
}

/// print welcome message
pub fn print_welcome(){
    println!("******************************************");
    println!("*        E-01 benchmark for rust rpc     *");
    println!("*             Version : 0.1.0            *");
    println!("******************************************");
    println!("---------------------------   Benchmark Start! --------------------------");
}

/// print benchmark result
pub fn print_result(output: &Vec<String>){
    println!();
    println!();
    println!("---------------------------   Benchmark Finished! --------------------------");
    for line in output {
        println!();
        println!("{}", line);
    }
}
