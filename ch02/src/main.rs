use ch02::newtypes::{Age, FirstName, LastName, Pay};
use ch02::{self, hello_world, my_vec};
struct Example {}
hello_world!(Example);

fn calculate_raise(first_name: FirstName, last_name: LastName, age: Age, current_pay: Pay) -> Pay {
    if first_name.get_value() == "Sam" {
        Pay {
            value: current_pay.get_value() + 1000,
        }
    } else {
        Pay {
            value: *current_pay.get_value(),
        }
    }
}

fn main() {
    let e = Example {};
    e.hello_world();

    let empty: Vec<i32> = my_vec![];
    println!("{:?}", empty);
    let three_numbers = my_vec!(1, 2, 3);
    println!("{:?}", three_numbers);
    let trailing = my_vec!(5, 6, 7,);
    //let trailing = vec![5, 6, 7];
    println!("{:?}", trailing);

    let first_raise = calculate_raise(
        "Sam".try_into().expect("Can not convert to first name"),
        "Smith".try_into().expect("Can not convert to last name"),
        20.try_into().expect("Can not convert to age."),
        1000.try_into().expect("Can not convert to pay."),
    );
}
