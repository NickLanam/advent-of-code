use advent_lib::{
  color::{RED, RESET, YELLOW},
  direction::CardinalDirection,
  runner::{Day, PartId},
};
use advent_of_code_2019::intcode::{Execution, execute};
use anyhow::Result;

type P1Out = String;
type P2Out = u64;
type Parsed = Vec<i64>;

enum Command<'a> {
  Move(CardinalDirection),
  Take(&'a str),
  Drop(&'a str),
  Inv,
}
impl<'a> From<&Command<'a>> for String {
  fn from(command: &Command) -> Self {
    match command {
      Command::Move(dir) => match dir {
        CardinalDirection::N => "north\n".to_string(),
        CardinalDirection::E => "east\n".to_string(),
        CardinalDirection::S => "south\n".to_string(),
        CardinalDirection::W => "west\n".to_string(),
      },
      Command::Take(item) => format!("take {item}\n"),
      Command::Drop(item) => format!("drop {item}\n"),
      Command::Inv => "inv\n".to_string(),
    }
  }
}
impl<'a> From<&Command<'a>> for Vec<i64> {
  fn from(command: &Command) -> Self {
    let as_string: String = command.into();
    as_string.chars().map(|c| c as u8 as i64).collect()
  }
}

fn print_output(result: &Execution) {
  for &n in result.outputs.iter() {
    if !(0..=255).contains(&n) {
      print!("{YELLOW}{n}{RESET}");
    } else {
      let nc = n as u8;
      if nc == 10 {
        println!();
      } else if nc.is_ascii_control() {
        print!("{RED}{n}{RESET}");
      } else {
        let c = nc as char;
        print!("{c}");
      }
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut result = execute(instructions, &[], Some(0), None)?;

    // It's a text-based dungeon game. I am greatly amused.
    // While I could write code to actually read and solve this, I chose to manually
    // play through it and write down the sequence of commands that reaches the password
    // for my input, then type that in manually.

    // If I return to this some day, I could try exploring how memory changes when picking up and dropping
    // items and when trying to go through the final door with the wrong combination,
    // then directly edit the memory addresses that are discovered to be the important ones,
    // then memory-edit inventory and position to go straight to the door and run the test,
    // then extract the answer from that output. It'd be fast, too.

    // Hull Breach (H_B)
    //  You got in through a hole in the floor here. To keep your ship from also freezing, the hole has been sealed.
    //  Items: None

    // Hot Chocolate Fountain (HCF)
    //  Somehow, it's still working
    //  Items: wreath

    // Observatory (OBS)
    //  There are a few telescopes; they're all bolted down, though.
    //  Items: food ration

    // Hallway (HLW)
    //  This area has been optimized for something; you're just not quite sure what.
    //  Items: giant electromagnet

    // Holodeck (HOD)
    //  Someone seems to have left it on the Giant Grid setting.
    //  Items: prime number

    // Stables (STB)
    //  Reindeer-sized. They're all empty.
    //  Items: astrolabe

    // Passages (PSG)
    //  They're a little twisty and starting to look all alike.
    //  Items: candy cane

    // Science Lab (SCN)
    //  You see evidence here of prototype polymer design work.
    //  Items: molten lava (WHICH KILLS YOU)

    // Kitchen (KCH)
    //  Items: None

    // Corridor (CRD)
    //  The metal walls and the metal floors are slightly different colors. Or are they?
    //  Items: escape pod (WHICH KILLS YOU)

    // Storage (STG)
    //  The boxes just contain more boxes. Recursively.
    //  Items: infinite loop (WHICH KILLS YOU)

    // Arcade (ACD)
    //  None of the cabinets seem to have power.
    //  Items: None

    // Warp Drive Maintenance (WDM)
    //  It appears to be working normally.
    //  Items: None

    // Sick Bay (SKB)
    //  Supports both Red-Nosed Reindeer medicine and regular reindeer medicine.
    //  Items: photons (WHICH KILL YOU)

    // Gift Wrapping Center (GWC)
    //  How else do you wrap presents on the go?
    //  Items: None

    // Navigation (NAV)
    //  Stranded. Please supply measurements from fifty stars to recalibrate.
    //  Items: weather machine

    // Engineering (ENG)
    //  You see a whiteboard with plans for Springdroid v2.
    //  Items: hypercube

    // Crew Quarters (CRW)
    //  The beds are all too small for you.
    //  Items: space law space brochure

    // Security Checkpoint
    //  In the next room, a pressure-sensitive floor will verify your identity.
    //  (Go west while carrying the correct combination of items to win)

    // Pressure-Sensitive Floor (END)
    //  (Text changes based on whether you found the solution or not)

    // Map for my input:
    //
    //       WDM
    //        |
    //       ACD - HCF - SKB - GWC - NAV
    //              |
    //             H_B - OBS - STG
    //              |     |
    // STB - HOD - HLW   SCN - CRD   END - SEC
    //              |     |     |           |
    //             PSG   KCH   ENG ------- CRW

    // Rooms to avoid taking items in:
    // - SCN (molten lava kills you)
    // - CRD (escape pod jettisons you into space)
    // - STG (game infinite loops lol)
    // - HLW (giant electromagnet blocks movement until dropped)
    // - SKB (photons make it dark and you are eaten by a Grue)
    // - H_B, ACD, KCN, WDM, GWC (nothing there)
    // DO take: candy cane, food ration, space law space brochure, astrolabe
    // Then go to the END room.

    let solution_inputs: Vec<Command> = vec![
      // Grab all eight safe items in a reasonably efficient order,
      // then go to security and drop the ones we need to drop to get through the door.
      // I might write a proper solver another day, but this was fun in the interim.

      // Get the wreath from HCF
      Command::Move(CardinalDirection::N),
      Command::Take("wreath"),
      // Get the weather machine from NAV
      Command::Move(CardinalDirection::E),
      Command::Move(CardinalDirection::E),
      Command::Move(CardinalDirection::E),
      Command::Take("weather machine"),
      // Back to Hull Breach
      Command::Move(CardinalDirection::W),
      Command::Move(CardinalDirection::W),
      Command::Move(CardinalDirection::W),
      Command::Move(CardinalDirection::S),
      // Get the candy cane from PSG
      Command::Move(CardinalDirection::S),
      Command::Move(CardinalDirection::S),
      Command::Take("candy cane"),
      // Get the prime number from HOD
      Command::Move(CardinalDirection::N),
      Command::Move(CardinalDirection::W),
      Command::Take("prime number"),
      // Get the astrolabe from STB
      Command::Move(CardinalDirection::W),
      Command::Take("astrolabe"),
      // Back to Hull Breach
      Command::Move(CardinalDirection::E),
      Command::Move(CardinalDirection::E),
      Command::Move(CardinalDirection::N),
      // Get the food ration from OBS
      Command::Move(CardinalDirection::E),
      Command::Take("food ration"),
      // Get the hypercube from ENG
      Command::Move(CardinalDirection::S),
      Command::Move(CardinalDirection::E),
      Command::Move(CardinalDirection::S),
      Command::Take("hypercube"),
      // Get the space law space brochure from CRW
      Command::Move(CardinalDirection::E),
      Command::Take("space law space brochure"),
      // Move to SEC
      Command::Move(CardinalDirection::E),
      Command::Move(CardinalDirection::N),
      // Alright... try it with all eight items. Which fails for being too heavy, of course.
      // Command::Move(CardinalDirection::W),
      // At this point, other people decided to watch memory values as the game changes and found the comparison,
      // found the weight values of the items, and found the solution that way.
      // Some started there, then figured out where to edit memory directly on any person's input to directly get the answer.
      Command::Inv,
      // I figured out experimentally which ones to drop. Got the password.
      Command::Drop("weather machine"),
      Command::Drop("hypercube"),
      Command::Drop("prime number"),
      Command::Drop("wreath"),
      Command::Inv,
      Command::Move(CardinalDirection::W),
    ];

    // print_output(&result);
    for input in solution_inputs.iter() {
      // println!("   INPUT: {}", String::from(input));
      let as_ints: Vec<i64> = input.into();
      result = execute(
        &result.final_tape,
        &as_ints,
        Some(result.pc),
        Some(result.ro),
      )?;
      // print_output(&result);
    }

    print_output(&result);

    Ok("Check the final output above.".to_string())
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 25)
}
