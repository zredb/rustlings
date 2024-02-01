// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: std::option::Option<i32> = Some(3);

    if !Option::is_none(&my_option) {
        match my_option {
            Some(x) => x,
            None => todo!(),
        };
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    //vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", vec![1, 2, 3, 4, 5].resize(0, 5));

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
