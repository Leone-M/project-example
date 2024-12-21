use project_example::math_functions;

fn main() {
    println!("Running default.rs file!");
    const X: i32 = 5;
    const Y: i32 = 3;
    let z = math_functions::add::add_func(&X, &Y);
    println!("{} + {} = {}", X, Y, z);
    let z = math_functions::sub::sub_func(&X, &Y);
    println!("{} - {} = {}", X, Y, z);
}
