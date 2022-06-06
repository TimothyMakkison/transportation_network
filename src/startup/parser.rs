use nom::character::complete::i32;
use nom::character::{is_digit, is_space};
use nom::multi::separated_list0;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    sequence::tuple,
    IResult,
};

use crate::models::{Command, TravelMode};

fn parse_int(bytes: &[u8]) -> IResult<&[u8], i32> {
    let (remainder, digits) = take_while(is_digit)(bytes)?;
    let (_, integer) = i32(digits)?;

    Ok((remainder, integer))
}

fn parse_space(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    let (remainder, spaces) = take_while(is_space)(bytes)?;

    Ok((remainder, spaces))
}

fn parse_maxdist(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (a, _) = tag("MaxDist")(bytes)?;
    Ok((a, Command::MaxDist))
}

fn parse_maxlink(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (a, _) = tag("MaxLink")(bytes)?;
    Ok((a, Command::MaxLink))
}

fn parse_finddist(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, a, _, b)) =
        tuple((tag("FindDist "), parse_int, parse_space, parse_int))(bytes)?;

    Ok((input, Command::FindDist(a, b)))
}

fn parse_findneighbours(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, a)) = tuple((tag("FindNeighbour "), parse_int))(bytes)?;

    Ok((input, Command::FindNeighbour(a)))
}

fn parse_places(bytes: &[u8]) -> IResult<&[u8], Vec<i32>> {
    let (a, b) = separated_list0(tag(" "), parse_int)(bytes)?;

    Ok((a, b))
}

fn parse_check(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, mode, _, integers)) =
        tuple((tag("Check "), parse_mode, parse_space, parse_places))(bytes)?;

    Ok((input, Command::Check(mode, integers)))
}

fn parse_findroute(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, mode, _, a, _, b)) = tuple((
        tag("FindRoute "),
        parse_mode,
        parse_space,
        parse_int,
        parse_space,
        parse_int,
    ))(bytes)?;

    Ok((input, Command::FindRoute(mode, a, b)))
}

fn parse_findfastestroute(bytes: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, mode, _, a, _, b)) = tuple((
        tag("FindShortestRoute "),
        parse_mode,
        parse_space,
        parse_int,
        parse_space,
        parse_int,
    ))(bytes)?;

    Ok((input, Command::FindShortestRoute(mode, a, b)))
}

pub fn parse_command(input: &[u8]) -> IResult<&[u8], Command> {
    alt((
        parse_maxdist,
        parse_maxlink,
        parse_finddist,
        parse_findneighbours,
        parse_check,
        parse_findroute,
        parse_findfastestroute,
    ))(input)
}

pub fn parse_mode(input: &[u8]) -> IResult<&[u8], TravelMode> {
    alt((
        parse_foot, parse_bike, parse_rail, parse_car, parse_bus, parse_ship,
    ))(input)
}

//TODO Replace with macros

fn parse_foot(bytes: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Foot")(bytes)?;
    Ok((a, TravelMode::Foot))
}
fn parse_bike(bytes: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Bike")(bytes)?;
    Ok((a, TravelMode::Bike))
}
fn parse_rail(bytes: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Rail")(bytes)?;
    Ok((a, TravelMode::Rail))
}
fn parse_car(bytes: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Car")(bytes)?;
    Ok((a, TravelMode::Car))
}
fn parse_bus(bytes: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Bus")(bytes)?;
    Ok((a, TravelMode::Bus))
}
fn parse_ship(bytes: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Ship")(bytes)?;
    Ok((a, TravelMode::Ship))
}

#[macro_export]
macro_rules! mag {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
// macro_rules! mode_parser {
//     ($($t:tt, $y:ty)+) => {
//         $(
//         pub fn $t(input: &[u8]) -> IResult<&[u8], $t>
//             {
//                 let (a, _) = tag($t)(input)?;
//                 Ok((a, $y))
//             }
//         )+
//     }
// }

// mode_parser! {TransportType::Foot}
