// use std::io;

// //***********crating a struct to hold our shops details*************//
// #[derive(Debug)]
// struct Shop {
//     store_name: String,
//     number_of_products: u32,
// }

// //creating an associated fuction for shop struct
// impl Shop {
//     fn create(name:String, products: u32)->Shop{
//         Shop{
//             store_name: name,
//             number_of_products: products
//         }
//     }
// }
// //creating a fuction to print shope details
// impl Shop {
//     fn print_shop_details (&mut self) {
//         println!("Name of shop is : {}",self.store_name);
//         println!("Number of products in a shop is : {}",self.number_of_products);

//     }
// }



// //************creating a struct to hold all the products*************** */
// #[derive(Debug)]
// struct ProductList {
//     list: Vec<Product>
// }

// //crating a associated function for aou list of products
// impl ProductList {
//     fn create()->ProductList{
//         ProductList{
//             list: Vec::new()
//         }
//     }
// }

// impl ProductList {
//     fn print_list(&self, num: Shop){
//         let mut index = 0;
//         //loop over all the product in a shop
//         for item in 0..num.number_of_products {

//             println!("product {} name is : {} ", item, self.list[index].name);
//             println!("product {} price is : {} ", item, self.list[index].price);
//             println!("product {} quantity is : {} ", item, self.list[index].qty);

//             //space
//             println!();
//             println!();
//             index = index + 1;
//         }
//     }
// }

// //********//creating a struct to hold our shop products details***/
// #[derive(Debug)]
// struct Product {
//     name: String,
//     price: u32,
//     qty: u32
// }

// //creating an associated fuction for product struct
// impl Product {
//     fn new(name: String, price: u32, qty: u32)->Product{
//         Product{
//             name,
//             price,
//             qty
//         }
//     }
// }

// fn main() {

//     //get store name from user
//     println!("Enter the name of your store:");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name);
//     let name: String = name.trim().parse().unwrap();

//     //get number of products from user
//     println!("Enter number of products in your stor :");
//     let mut num = String::new();
//     io::stdin().read_line(&mut num);
//     let num: u32 = num.trim().parse().unwrap();

//     let mut my_shop = Shop::create(name, num);

//     my_shop.print_shop_details();

//     //Creating an empty list to store all the products
//     let mut my_list_of_products = ProductList::create();

//     //collecting all the info of the products
//     for item in 0..my_shop.number_of_products {

//         //get product name from user
//         println!("Enter the name of product {}:",item);
//         let mut name = String::new();
//         io::stdin().read_line(&mut name);
//         let name: String = name.trim().parse().unwrap();

//         //get price of the protuct from user
//         println!("Enter price of products {} :",item);
//         let mut price = String::new();
//         io::stdin().read_line(&mut price);
//         let price: u32 = price.trim().parse().unwrap();

//         //get quantity of items in stoke from user
//         println!("Enter quantity in stoke :");
//         let mut qty = String::new();
//         io::stdin().read_line(&mut qty);
//         let qty: u32 = qty.trim().parse().unwrap();
        

//         //pushing all the products into a vector
//         my_list_of_products.list.push(Product::new(name, price, qty));

//     } 

//     //printing al the product with details in a terminal
//     my_list_of_products.print_list(my_shop);
   
    
// }

//*****************************TASK 6*********************************/
use std::io;

//***********crating a struct to hold our shops details*************//
#[derive(Debug)]
struct Shop {
    store_name: String,
    number_of_products: u32,
}

//creating an associated fuction for shop struct
impl Shop {
    fn create(name:String, products: u32)->Shop{
        Shop{
            store_name: name,
            number_of_products: products
        }
    }
}
//creating a fuction to print shope details
impl Shop {
    fn print_shop_details (&mut self) {
        println!("Name of shop is : {}",self.store_name);
        println!("Number of products in a shop is : {}",self.number_of_products);

    }
}



//************creating a struct to hold all the products*************** */
#[derive(Debug)]
struct ProductList {
    list: Vec<Product>
}

//crating a associated function for aou list of products
impl ProductList {
    fn create()->ProductList{
        ProductList{
            list: Vec::new()
        }
    }
}

impl ProductList {
    fn print_list(&self){
        for (index, item) in self.list.iter().enumerate() {
            println!("code of product : {}", self.list[index].code);
            println!("name of product : {}", self.list[index].name);
            println!("price of product : {}", self.list[index].price);
            println!("quantity in stock : {}", self.list[index].qty);
            println!("");
        }
    }

    fn add_product(&mut self){

        //get product name from user
        println!("Enter the code of product :");
        let mut code = String::new();
        io::stdin().read_line(&mut code);
        let code: String = code.trim().parse().unwrap();

        //get product name from user
        println!("Enter the name of product :");
        let mut name = String::new();
        io::stdin().read_line(&mut name);
        let name: String = name.trim().parse().unwrap();

        //get price of the protuct from user
        println!("Enter price of products :");
        let mut price = String::new();
        io::stdin().read_line(&mut price);
        let price: u32 = price.trim().parse().unwrap();

        //get quantity of items in stoke from user
        println!("Enter quantity in stoke :");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty);
        let qty: u32 = qty.trim().parse().unwrap();
        

        //pushing the product into a vector
        self.list.push(Product::new(code, name, price, qty));

    }
}

//********//creating a struct to hold our shop products details***/
#[derive(Debug)]
struct Product {
    code: String,
    name: String,
    price: u32,
    qty: u32
}

//creating an associated fuction for product struct
impl Product {
    fn new(code: String,name: String, price: u32, qty: u32)->Product{
        Product{
            code,
            name,
            price,
            qty
        }
    }
}

fn main() {

    //get store name from user
    println!("Enter the name of your store:");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    let name: String = name.trim().parse().unwrap();

    //get number of products from user
    println!("Enter number of products in your stor :");
    let mut num = String::new();
    io::stdin().read_line(&mut num);
    let num: u32 = num.trim().parse().unwrap();
  
    let mut my_shop = Shop::create(name, num);

    my_shop.print_shop_details(); 

    //creat an instance of empty list
    let mut my_list_of_products = ProductList::create();

    let mut flag = 0;
    while flag != 3 {
        //ucommand line user interface
        println!("");
        println!("*****Select from the menu******");
        println!("");
        println!("1. Add New Product");
        println!("2. Show All  Products");
        println!("3. Exit");
        let mut command = String::new();
        io::stdin().read_line(&mut command);
        let command: u32 = command.trim().parse().unwrap();

        match command {
            1 =>  my_list_of_products.add_product(),
            2 => my_list_of_products.print_list(),
            3 => {
                    println!("Exiting...Thank You");
                    flag=3; },
            _ => println!("please enter a valid command")
        }

    }
    
}