use ansi_term::Style; 

const PI:f32 = 3.14;
static mut GLOBAL:u8 = 109;

fn calc(a:i32, b:i32) -> i32 {
    if a > 500 {
        a+b+a
    } else {
        a+b
    }
}

fn shadown() {
    let a = 900;

    {
        let b = 321;
        println!("Print to me B = {} and A = {}", b, a);

        //shadowing
        let a = 123;
        println!("Value = {}", calc(a,b));
    }

    println!("Print to me A = {}", a);
}

fn scope() {
    println!("PI = {}", PI);

    unsafe {
        println!("Global value = {}", GLOBAL);
    }

    let v:i32 = 300;
    println!("Show me a number = {}, size = {}", v, std::mem::size_of_val(&v));

    let decimal:f32 = 2.5;
    println!("Show me another number = {}", decimal);

    let boolean:bool = true;
    println!("Show me a boolean value = {}, boolean size = {}", boolean, std::mem::size_of_val(&boolean));

    let c:char = 'C';
    println!("Size of char = {}", std::mem::size_of_val(&c));
}

fn conditions() {

    let age:u8 = 16;
    let let_me_know = age >= 18;

    let condition = if let_me_know { "Let him in!!!" } else {"You should not pass!!!"};

    println!("{}", condition);

    let lang = "Python";
    let mean = match lang {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        "JavaScript" => "All f***ing",
        &_ => "Unknown"
    };
    
    println!("We use {} for {} applications", lang, mean);
    
}

fn repeat() {
    let mult:u8 = 7;

    println!("------------");
    
    let mut count = 0;
    
    let bold = Style::new().bold();
    
    while count < 10 {
        count += 1;
        
        if count == 5 {
            println!("{} {} x {} = {}", 
            bold.paint("Question:"),
                                    bold.paint(mult.to_string()), 
                                    bold.paint(count.to_string()),
                                    bold.paint("???")
                                );
                                continue;
        }
        
        println!("{} x {} = {}", mult, count, (mult*count));
    }
   
    println!("------------");
    
    count = 0;
    loop {
        count +=1 ;
        
        if count == 8 {
            println!("{} {} x {} = {}", 
                                    bold.paint("Question:"),
                                    bold.paint(mult.to_string()), 
                                    bold.paint(count.to_string()),
                                    bold.paint("???")
                                );
            continue;
        }

        println!("{} x {} = {}", mult, count, (mult*count));
        
        if count == 10 { break; }
    }

    println!("------------");

    for i in 1..11 {
        //OR for i in 1..=10
        println!("{} x {} = {}", mult, i, (mult*i));
    }
}

fn ownership() {
    let mut string = String::from("Christian");
    
    take_owner(&mut string); //Borrowing: gives a ref for the function, do not copy and paste pointers

    println!("{}", string);
}

fn take_owner(string : &mut String) {
    string.push_str(" Rodrigues");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Almost nothing",
            2 | 3 => "Just a little bit",
            4..=10 => "Little",
            _ if x % 2 == 0 => "A good amount",
            _ => "Too much"
        });
    }
}

fn err() {
    //panic!("Don't do that! please...");

    match res() {
        Ok(s) => println!("{}", s),
        Err(n) => print!("{}", n),
    }
}

fn res() -> Result<String, u8> {
    Ok(String::from("Everything is right here!"))
}

fn main () {
    scope();
    shadown();
    conditions();
    repeat();
    ownership();
    pattern_matching();
    err();
}

//Ownership, Borrowing e Reference