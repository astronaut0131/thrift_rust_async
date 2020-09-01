use std::borrow::Borrow;

/// print time result in md table
pub fn format_result(mode: String, call_num: i64, total_time_in_ms: i64,
                     avg_time: i64, per_50_time: i64, per_90_time: i64,
                     per_95_time: i64, per_99_time: i64,
                     per_999_time: i64, max_time : i64) -> String {
    println!("time : {}", total_time_in_ms);
    format!("###{}
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    {} ms  |        {}        |    {} us   |    {} us   |     {} us    |    {} us    |   {} us   |   {}  us  |   {}  us  |"
            , mode, total_time_in_ms, (1000 * call_num / total_time_in_ms), avg_time / 1000, per_50_time / 1000,
            per_90_time / 1000, per_95_time / 1000, per_99_time / 1000, per_999_time / 1000, max_time / 1000)
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
pub fn print_welcome() {
    println!("******************************************");
    println!("*        E-01 benchmark for rust rpc     *");
    println!("*             Version : 0.1.0            *");
    println!("******************************************");
    println!("---------------------------   Benchmark Start! --------------------------");
}

/// print benchmark result
pub fn print_result(output: &Vec<String>) {
    println!();
    println!();
    println!("---------------------------   Benchmark Finished! --------------------------");
    for line in output {
        println!();
        println!("{}", line);
    }
}

pub fn handle_time(time_arrays: Vec<Box<Vec<i64>>>) -> Box<Vec<i64>> {
    let mut sum = 0;
    let mut count = 0;
    let mut times: Vec<i64> = Vec::new();
    for time_array_result in time_arrays {
        let time_array: &Vec<i64> = time_array_result.borrow();
        for time in time_array {
            times.push(*time);
            sum += time;
            count += 1;
        }
    }

    times.sort();
    let mut res = Vec::new();
    // avg
    res.push(sum / count);
    // per 50
    res.push(times[times.len() / 2]);
    // per 90
    res.push(times[(times.len() / 10) * 9]);
    // per 95
    res.push(times[(times.len() / 100) * 95]);
    // per 99
    res.push(times[(times.len() / 100) * 99]);
    // per 99.9
    res.push(times[(times.len() / 1000) * 999]);
    // max time
    res.push(times[times.len() - 1]);

    return Box::new(res);
}
