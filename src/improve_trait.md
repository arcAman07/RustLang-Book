fn some_function<T: Display + Clone, U: Clone + Debug>(t:&T, u: &U) -> i32 {

fn some_function<T, U>(t:&T, u: &U) -> i32 
where
t: Display + Clone,
u: Clone + Debug,
{