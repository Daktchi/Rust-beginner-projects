use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};

struct Contact{
    name : String,
    phone : i32,
}

fn create_carnet(contact: Contact) -> std::io::Result<()> {

    // let mut f = File::create_new("carnet.txt")?;
    //
    // f.write("Hello, world!".as_bytes())?;
    // Ok(())

    let mut f = File::options().append(true).open("carnet.txt")?;
    writeln!(&mut f,"{} : {}", contact.name, contact.phone)?;//Write and add new contact in the file
    Ok(())

}

fn read_carnet()->std::io::Result<()> {//Reading contacts from carnet list

    let mut file = File::open("carnet.txt")?;
    let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut count = 0;
        for line in contents.lines() {//Get line into the whole text
            count+= 1;
            println!("{count}- {line}");//Line by line
        }


        Ok(())
}

fn remove_contact(contact: i32, mut contacts :Vec<String>)->std::io::Result<()> {//Removing a contact by its ID number

    contacts.remove((contact-1) as usize);//Removing of the contact at its exact position
    fs::remove_file("carnet.txt")?; //The old carnet is removed

    let _ = File::create("carnet.txt");//Recreating a new contact
    let mut f = File::options().append(true).open("carnet.txt")?;
    for contact in contacts {//Updating the new carnet with the rest of contact
        writeln!(&mut f,"{contact}", )?;//Write and add new contact in the file
    }

    Ok(())

}

pub fn require_option() -> Option<i32> {//Control that the user has entered a valid number option required

    //TOP OPTION REQUIREMENT.
    let mut option = Default::default();
    io::stdin().read_line(&mut option).expect("Please enter a value");

    let option: i32 = match option.trim().parse() { //take the user option
        Ok(num) =>
            { //Control that the value is an option list
                if num >= 0 && num <= 2 {//Range of options
                    num
                } else {
                    println!("\n〜〜〜Please! The option value should be from the option list range of [0-2]〜〜〜\n\n");
                    return None
                }
            }
        ,
        Err(_) => {
            println!("\n \t \t〜〜〜Enter a valid number Option〜〜〜\n\n");
            return None
        },
    };

    Some(option)

}

fn main() {

    println!("\n################################################## \
               \n#\tWELCOME TO CARNET D'ADRESSE MANAGER    \t#\
               \n##################################################\n");


    loop {

        println!("\n-----------Lists of Contact------------\n");

       let _ = read_carnet();//Reading contacts from carnet list

        println!("\n1- Add a contact; \n2- Remove a contact \n0- Exit the Carnet Manager\
                \n{}:
                ", "Choose an option".to_uppercase());

        let option = match require_option() {// The user's input option
            Some(value) => value,
            None => continue,
        };

    if option == 1 { //Adding a contact

            println!("\nName : ");
            let mut name = Default::default();
            io::stdin().read_line(&mut name).expect("Enter a value");//Handle the case when there is no value
            let name = name.trim();
            println!("Phone number : ");

            let mut phone  = Default::default();
            io::stdin().read_line(&mut phone).expect("Enter a value");
            let phone : i32 = match phone.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Wrong number! Impossible to save it.");
                    continue
                }
            };

            let contact  = Contact{
                name : name.to_string(),
                phone
            };

            let _ = create_carnet(contact);


        }else if option == 2 {

            println!("-------Enter the number of the contact that you want to delete---------");


        let mut option = Default::default();
        io::stdin().read_line(&mut option).expect("Please enter a value");

        let option: i32 = match option.trim().parse() { //take the user option
            Ok(num) =>
               num
            ,
            Err(_) => {
                println!("\n \t \t〜〜〜Enter a valid number Option〜〜〜\n\n");
                continue
            }};

            // if op_list_element > 0  {

                let mut file = File::open("carnet.txt").unwrap();
                let mut contents = String::new();
                let mut contacts: Vec<String> = Vec::new();

                let _  = file.read_to_string(&mut contents);

                    for line in contents.lines() {//Get line into the whole text
                        contacts.push(line.into());
                    }

                    println!("{:?}", contacts);
                    // return;
                    let _ = remove_contact(option, contacts);

                println!("********Contact removed successfully**********\n\n");

            // }else {
            //     println!("********The list you want to delete doesn't exist*********\n\n");
            // };

        }else if option == 0 {

            println!("Carnet exited successfully !");
            break;

        }else {
            panic!("Error!");
        }

    }

}
