//Using a hash map and vectors, create a text interface to allow a user to add
//employee names to a department in a company. For example,
//Add Sally to Engineering or Add Amir to Sales. Then let the user
//retrieve a list of all people in a department or all people in the company
//by department, sorted alphabetically.

//WITHOUT ANY HELP FROM ChatGPT !!!

fn main() {
	use std::collections::HashMap;
	let mut company = HashMap::new();
	let _dept_names: Vec<String> = Vec::new();

	loop {
	    println!("What do you wish to do?");
	    println!("Add new person to department? = 1");
	    println!("Show all people of a department? = 2");
	    println!("Show all people of the company? = 3");

	    let mut input = String::new();
	    std::io::stdin().read_line(&mut input).expect("failed to read line");

	    let input: u32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) =>continue,
		};

	    match input {
		    1 => {
				println!("What's the person's name?");

	            let mut name = String::new();
	            std::io::stdin().read_line(&mut name).expect("not a name");

				let name = name.split_whitespace().map(|name| {
					let _ = name.chars();
					format!("{}", name)
				}).collect::<Vec<String>>().join(" ");

				let vec_name: Vec<String> = vec![name.clone()];

	            println!("In which department is this person working?");

	            let mut dept = String::new();
	            std::io::stdin().read_line(&mut dept).expect("not a dept");

		        company.entry(dept).and_modify(|dept_names: &mut Vec<String>| {dept_names.push(name)}).or_insert(vec_name);
			}
		    2 => {
				println!("Which department do you wish to see?");
				for key in company.keys() {
					println!("{}?", key);
				}

				let mut input = String::new();
				std::io::stdin().read_line(&mut input).expect("failed to read line");

				let show_ppl = company.get(&input).unwrap();

				println!("People in this department: {:?}", show_ppl);
			}
		    3 => {
				for (dept, name) in &company {
		            println!("Department: {}, Name: {:?}", dept, name);
				}
			}
		    _ => println!("not an option!"),
	    }
    }
}