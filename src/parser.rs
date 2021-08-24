use nom::character::complete::i32;
use nom::character::{is_digit, is_space};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    sequence::tuple,
    IResult,
};

fn parse_maxdist(i: &[u8]) -> IResult<&[u8], Command> {
    let (a, _) = tag("MaxDist")(i)?;
    Ok((a, Command::MaxDist))
}

fn parse_maxlink(i: &[u8]) -> IResult<&[u8], Command> {
    let (a, _) = tag("MaxLink")(i)?;
    Ok((a, Command::MaxLink))
}

fn parse_finddist(i: &[u8]) -> IResult<&[u8], Command> {
    let space = take_while(is_space);

    let (input, (_, a, _, b)) = tuple((
        tag("FindDist "),
        take_while(is_digit),
        space,
        take_while(is_digit),
    ))(i)?;

    let (_, a) = i32(a)?;
    let (_, b) = i32(b)?;

    Ok((input, Command::FindDist(a, b)))
}

fn parse_findneighbours(i: &[u8]) -> IResult<&[u8], Command> {
    let (input, (_, a)) = tuple((tag("FindNeighbour "), take_while(is_digit)))(i)?;

    let (_, a) = i32(a)?;

    Ok((input, Command::FindNeighbour(a)))
}

pub fn parse_command(input: &[u8]) -> IResult<&[u8], Command> {
    alt((
        parse_maxdist,
        parse_maxlink,
        parse_finddist,
        parse_findneighbours,
    ))(input)
}

pub fn parse_mode(input: &[u8]) -> IResult<&[u8], TravelMode> {
    alt((
        parse_foot, parse_bike, parse_rail, parse_car, parse_bus, parse_ship,
    ))(input)
}

#[derive(Debug)]
pub enum Command {
    MaxDist,
    MaxLink,
    FindDist(i32, i32),
    FindNeighbour(i32),
    Check(TravelMode, Vec<i32>),
    FindRoute(TravelMode, i32, i32),
    FindShortestRoute(TravelMode, i32, i32),
}
#[derive(Debug)]
pub enum TravelMode {
    Foot,
    Bike,
    Car,
    Bus,
    Ship,
    Rail,
}

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
