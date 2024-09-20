#![allow(dead_code)]

use std::{collections::HashMap, error::Error, fmt, io};

#[derive(Debug)]
enum ParsePaymentInfoError {
    ParseError(String),
    Other(String),
}

impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&format!(
            "Parsing payment error: invalid payment info:\n{self:?}"
        ))
    }
}

impl Error for ParsePaymentInfoError {}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse().map_err(|err| {
                // \n does not work
                ParsePaymentInfoError::ParseError(format!(
                    "Failed to parse input `{card}` as numbers <- `{s}` could not be parsed as u32 <- {err}"
                ))
            })
        })
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError::Other(
            format!("Incorrect number of elements parsed <- expected `{expected_len}` but got `{len}` <- elements: `{numbers:?}`")
        ));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv,
    })
}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other(String),
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt.write_str("Credit card error: Could not retrieve credit card.")
        fmt.write_str(&format!(
            "Credit card error: Could not retrieve credit card:\n{self:?}"
        ))
    }
}

impl Error for CreditCardError {}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card was found for {name}.");
        CreditCardError::InvalidInput(msg.clone())
    })?;

    // parse_card could pop 2 variants of ParsePaymentInfoError
    // - ParseError
    // - Other
    let card = parse_card(card_string).map_err(|err| match &err {
        ParsePaymentInfoError::ParseError(s) => {
            CreditCardError::Other(format!("{name}'s card could not be parsed <- {s}"))
        }
        ParsePaymentInfoError::Other(s) => {
            CreditCardError::Other(format!("{name}'s card could not be parsed <- {s}"))
        }
    })?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => {
            println!("\nCredit Card Info: {card:?}");
        }
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("\n{msg}"),
                CreditCardError::Other(_) => println!("\nSomething went wrong! Try again!"),
            }

            // debug - not our target
            // log::error!("\n{err:?}");
            // debug but pretty print- not our target
            // log::error!("\n{err:#?}");
            // dispay
            log::error!("\n{err}");
        }
    }
}
