use clap::{App, Arg, ArgMatches};

fn get_address(matches: ArgMatches) {
  println!("You ordered for delivery.");
  if let Some(i) = matches.value_of("address") {
    match i {
      "moon" => println!("Delivery not available to: {}", i),
      _ => println!("Delivering to address: {}", i),
    }
  }
}

fn main() {
  let matches = App::new("Pizza Maker")
    .version("1.0")
    .author("Simranjeet Singh <smrnjeet222@gmail.com>")
    .about("Helps u make the best pizza")
    .arg(
      Arg::new("style")
        .long("style")
        .value_name("STYLE")
        .about("What type of pizza u want?")
        .takes_value(true),
    )
    .arg(
      Arg::new("toppings")
        .short('t')
        .long("toppings")
        .value_name("TOPPINGS")
        .multiple_occurrences(true)
        .about("What toppings of pizza u want?")
        .takes_value(true),
    )
    .arg(
      Arg::new("order_type")
        .long("order_type")
        .required(true)
        .value_name("ORDER_TYPE")
        .about("dine-in, delivery or pick-up?")
        .takes_value(true),
    )
    .arg(
      Arg::new("address")
        .long("adr")
        .required_if_eq("order_type", "delivery")
        .value_name("ADDRESS")
        .about("Where do u live?")
        .takes_value(true),
    )
    .get_matches();

  if let Some(i) = matches.value_of("style") {
    match i {
      "thin crust" => println!("{} is my favourite too", i),
      "pan" => println!("sucks!"),
      _ => println!("You asked {} pizza", i),
    }
  }

  if let Some(i) = matches.values_of("toppings") {
    println!("Toppings :");
    let vals: Vec<&str> = i.collect();

    for val in vals {
      match val {
        "corn" => println!(" - {}", val),
        "extra cheesee" => println!(" - {}", val),
        "pineapple" => println!(" - {}", val),
        _ => println!("{} not available", val),
      }
    }
  }

  if let Some(i) = matches.value_of("order_type") {
    match i {
      "delivery" => get_address(matches),
      _ => println!("You ordered for {}", i),
    }
  }
}
