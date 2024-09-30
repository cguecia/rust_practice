fn main() {
    let name: &str = "Goku";

    let a: i32 = 8_9999;
    let b: i32 = 1;

    {
        let inner: String = String::from("I am Inner scoped");
        println!("Hello from inner scope! {}", inner)
    }

    let result: i32 = add_numbers(a, b);

    println!("Hello, rust world! My Name is {}!", name);
    println!("My power level is over {} !!!", result);

    let mut message: String = String::from("Hi there!");
    message.push_str(" How are you?");
    println!("{}", message); // Output: Hi there! How are you?

    let last_number: i32 = looper();
    println!("the last num={}", last_number); // 12

    vector_practice();

    let name_to_send: String = String::from("Bluey");

    let retuned_name: String = str_practice(name_to_send);

    println!("{}", retuned_name);

    let mut dbz_character = Hero::new(
        String::from("Gohan"),
        1_000_000_000,
        String::from("Sayian"),
        true,
    );

    dbz_character.transform(String::from("Super Saiyan"));
    println!(
        "{} is a {}",
        dbz_character.name, dbz_character.transformation_state
    );

    dbz_character.transform(String::from("Super Saiyan 2"));
    println!(
        "{} is a {}",
        dbz_character.name, dbz_character.transformation_state
    );

    dbz_character.transform(String::from("Super Saiyan 2"));
    println!(
        "{} is a {}",
        dbz_character.name, dbz_character.transformation_state
    );

    dbz_character.transform(String::from("Super Saiyan 3"));
    println!(
        "{} is a {}",
        dbz_character.name, dbz_character.transformation_state
    );

    dbz_character.transform(String::from("Beast"));
    println!(
        "{} is a {}",
        dbz_character.name, dbz_character.transformation_state
    );

    dbz_character.show_transformation_state();

    dbz_character.show_transformation_state_histroy();

    let mut bill = Hero::new(
        String::from("Bill"),
        1_000_000_000,
        String::from("Human"),
        false,
    );

    bill.transform(String::from("Pickle Mode"));
    bill.transform(String::from("Beast Mode"));

    bill.show_transformation_state();

    println!("Bill build current state is {:#?}", &bill);

    let result = longest(
        &bill.transformation_state_history,
        &dbz_character.transformation_state_history,
    );
    println!("The longest transformation state ={:#?}", result);

    let v1 = vec![String::from("hello"), String::from("there")];
    let v2 = vec![String::from("same"), String::from("length")];

    let same_length = longest(&v1, &v2);
    println!("The longest vec = {:#?}", same_length);

    if let Some(longest_vec) = same_length {
        println!("The longest vector is: {:?}", longest_vec);
    } else {
        println!("The vectors have equal length");
    }
}

// simple function practice
fn add_numbers(number_1: i32, number_2: i32) -> i32 {
    let result: i32 = number_1 + number_2;
    result
}

// for loop practice
fn looper() -> i32 {
    let mut last: i32 = 0;
    for item in 0..100 {
        println!("The num={}", item);
        if item >= 100 {
            break;
        };
        last = item;
    }

    assert_eq!(last, 99);
    last
}

// vector practice
fn vector_practice() {
    let mut nums: Vec<i32> = vec![1, 2, 3]; // mut nums: Vec<i32>

    nums.push(4);
    nums.push(5);
    nums.push(6);

    println!("The length of nums is now {}", nums.len());
}

fn str_practice(mut name_to_change: String) -> String {
    println!("{}", name_to_change);

    let ref_name: &String = &name_to_change;

    println!(
        "ref_name={} is a reference to name={}",
        ref_name, name_to_change
    );

    name_to_change.push_str(" Greeney");

    name_to_change
}

// struct practice
#[derive(Debug)]
struct Hero {
    name: String,
    power_level: i32,
    race: String,
    transformation_state: String,
    is_god_tier: bool,
    transformation_state_history: Vec<String>,
}

// implement practice
impl Hero {
    fn new(name: String, power_level: i32, race: String, is_god_tier: bool) -> Hero {
        Hero {
            name,
            power_level,
            race,
            transformation_state: String::from("Base"),
            transformation_state_history: vec![String::from("Base")],
            is_god_tier,
        }
    }

    fn transform(&mut self, transformation_state: String) {
        if self.transformation_state != transformation_state {
            self.transformation_state = transformation_state;

            self.transformation_state_history
                .push(self.transformation_state.clone());

            println!("{} transformed to {}", self.name, self.transformation_state);
        } else {
            println!(
                "{} is already in {} state",
                self.name, self.transformation_state
            );
        }
    }

    fn show_transformation_state(&self) {
        println!(
            "The current transformation state: {}",
            &self.transformation_state
        )
    }

    fn show_transformation_state_histroy(&self) {
        println!(
            "The current transformation state hisrtory: {:#?}",
            &self.transformation_state_history
        )
    }
}

// generic lifetime annotations practice with Some
fn longest<'a>(x: &'a Vec<String>, y: &'a Vec<String>) -> Option<&'a Vec<String>> {
    if x.len() == y.len() {
        None
    } else if x.len() > y.len() {
        Some(x)
    } else {
        Some(y)
    }
}
