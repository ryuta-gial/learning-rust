mod pizza_order {
    //パブリックな構造体
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        //パブリックな関数
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza) {
            println!(
                "The customer is erved a regular pizza with {}",
                cust_pizza.topping
            );
        }
    }
}

pub fn order_food(){
    crate::restaurant::pizza_order::help_customer::take_order();
}
