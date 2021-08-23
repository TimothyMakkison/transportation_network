use nom::character::{is_digit, is_space};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    number::complete::be_i32,
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

    let (_, a) = be_i32(a)?;
    let (_, b) = be_i32(b)?;

    Ok((input, Command::FindDist(a, b)))
}

pub fn parser(input: &[u8]) -> IResult<&[u8], Command> {
    alt((parse_maxdist, parse_maxlink, parse_finddist))(input)
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
    Rail,
    Car,
    Bus,
    Ship,
}
