pub mod newtypes;

#[macro_export]
macro_rules! hello_world {
    ($something:tt) => {
        impl $something {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };
}

#[macro_export]
macro_rules! my_vec {
    () => [
        Vec::new()
    ];
    {$x: expr} => {
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };
    {$($x: expr),+ $(,)?} => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    }
}
