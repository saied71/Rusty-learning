use std::collections::HashMap;

fn main() {
    
    // let mut hm = HashMap::new();

    // hm.insert(String::from("asshole"), 13);
    // hm.insert(String::from("asdasdasdasd"), 12);

    // for (k, v) in &hm {
        
    //     println!("{}, {}", k, v)
    // }

    // match hm.get(&String::from("asshole")) {
    //     Some(&n) => println!("{}", n),
    //     _ => println!("mo match"),
    // }

    let mut s = Some(0);
    
    while let Some(i) = s {
        if i > 20 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }
}