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

fn parse_int(i: &[u8]) -> IResult<&[u8], i32> {
    let (remainder, digits) = take_while(is_digit)(i)?;
    let (_, integer) = i32(digits)?;

    return Ok((remainder, integer));
}

fn parse_space(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (remainder, spaces) = take_while(is_space)(i)?;

    return Ok((remainder, spaces));
}

fn parse_maxdist(i: &[u8]) -> IResult<&[u8], Command> {
    let (a, _) = tag("MaxDist")(i)?;
    Ok((a, Command::MaxDist))
}

fn parse_maxlink(i: &[u8]) -> IResult<&[u8], Command> {
    let (a, _) = tag("MaxLink")(i)?;
    Ok((a, Command::MaxLink))
}

fn parse_finddist(i: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, a, _, b)) = tuple((tag("FindDist "), parse_int, parse_space, parse_int))(i)?;

    Ok((input, Command::FindDist(a, b)))
}

fn parse_findneighbours(i: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, a)) = tuple((tag("FindNeighbour "), parse_int))(i)?;

    Ok((input, Command::FindNeighbour(a)))
}

fn parse_places(i: &[u8]) -> IResult<&[u8], Vec<i32>> {
    let (a, b) = separated_list0(tag(" "), parse_int)(i)?;

    return Ok((a, b));
}

fn parse_check(i: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, mode, _, integers)) =
        tuple((tag("Check "), parse_mode, parse_space, parse_places))(i)?;

    Ok((input, Command::Check(mode, integers)))
}

fn parse_findroute(i: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, mode, _, a, _, b)) = tuple((
        tag("FindRoute "),
        parse_mode,
        parse_space,
        parse_int,
        parse_space,
        parse_int,
    ))(i)?;

    Ok((input, Command::FindRoute(mode, a, b)))
}

fn parse_findfastestroute(i: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, mode, _, a, _, b)) = tuple((
        tag("FindShortestRoute "),
        parse_mode,
        parse_space,
        parse_int,
        parse_space,
        parse_int,
    ))(i)?;

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

fn parse_foot(i: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Foot")(i)?;
    Ok((a, TravelMode::Foot))
}

fn parse_bike(i: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Bike")(i)?;
    Ok((a, TravelMode::Bike))
}
fn parse_rail(i: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Rail")(i)?;
    Ok((a, TravelMode::Rail))
}
fn parse_car(i: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Car")(i)?;
    Ok((a, TravelMode::Car))
}
fn parse_bus(i: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Bus")(i)?;
    Ok((a, TravelMode::Bus))
}
fn parse_ship(i: &[u8]) -> IResult<&[u8], TravelMode> {
    let (a, _) = tag("Ship")(i)?;
    Ok((a, TravelMode::Ship))
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
