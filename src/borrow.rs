fn main() {
    let important_data = "Hello, world".to_string();
    // borrow of moved value: `important_data`
    // calc_data(important_data);
    calc_data(&important_data);
    println!("{}", important_data);

    // x does not live long enough
    // let y;

    // {
    //     let x = 5;
    //     y = &x;
    //     println!("{}", x);
    // }
    // println!("{}", y);
}

fn calc_data(data: &String) {
    println!("{}", data);
}
