///
/// this project shows some basic rust syntax
/// enum, struct, function, generics, mut, errors
/// 

/// use external crates
extern crate dimensioned as dim;
use dim::si;
use dim::dimensions::Length;
use std::{thread, time, fmt};
use std::error::Error;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 

///
/// REQUIREMENT
/// 

/// enum
#[derive(Debug)]
enum Requirement {
    None,
    Depends,
    Mandatory
}

/// make it fun
impl Distribution<Requirement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Requirement {
        // pattern matching
        match rng.gen_range(0, 100) {
            0  ..= 24 => Requirement::None,
            25 ..= 50 => Requirement::Depends,
            _ => Requirement::Mandatory,
        }
    }
}

///
/// TAKE ACTION
/// 

/// struct
struct LocalInfrastructure
{
    // can we make tests?
    are_tests_available: bool,
}

/// custom error
#[derive(Debug)]
enum CovidTestError
{
    NotPossible,
    Unknown,
}

impl Error for CovidTestError {}
impl fmt::Display for CovidTestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        CovidTestError::NotPossible => write!(f, "It wasn't possible to carry out the test"),
        CovidTestError::Unknown => write!(f, "Something wrong happened in the covid test"),
      }
    }
  }
  

// this function may fail gracefully 
fn make_covid_test() -> Result<bool, CovidTestError> {

    // actually carry out the test (set to false to simulate an error)
    let infected:bool = true; 

    match infected {
      true => {
        println!("Test :)");
        Ok(true)
      },
      false => {
        println!("Test :(");
        Err(CovidTestError::Unknown)
      }
    }
}
  

/// trait
trait TakeAction {
    // simple function
    fn wear_mask(&self);

    // introducing generics (distance of type length)
    fn keep_distance<D>(&self, _dist:D) where D: Length;
    
    // introducing mutability
    // there's a lot more to be said
    fn wash_hands(&self, count:&mut usize);

    // introducting results and error handling 
    // there's a lot more to be said
    fn get_tested(&self) -> Result<bool, CovidTestError>;
}

/// impl actions against covid
impl TakeAction for LocalInfrastructure {

    fn wear_mask(&self) {
        // assigning a probability to the requirement
        let now: Requirement = rand::random();
        
        // match condition
        match now {
            Requirement::Mandatory => println!("I wear a mask, yeah"),
            _ => println!("I'm not wearing a mask, neah"),
        }
    }

    fn keep_distance<D>(&self, _dist:D)  
    where D: Length
    {
        // uncomment one of the macro if you want to generate an exception  
        // unimplemented!();
        // panic!("I'm panicking!")
    }

    fn wash_hands(&self, count:&mut usize) {
        // increment counter defined in main 
        *count+=1;
        println!("I already washed my hands {} times", count);
    }

    fn get_tested(&self) -> Result<bool, CovidTestError> { 

        // can we make tests
        if !self.are_tests_available {
            return Err(CovidTestError::NotPossible);
        }
        
        // ? operator is similar to unwrap but instead of panicking, it propagates the error to the calling function
        let result:bool = make_covid_test()?;
        Ok(result)
    }
}


/// main function = staying alive
fn main() {
    println!("Trying to alive during a pandemic!");

    // initialize current count (mutable)
    let mut wash_count:usize = 0;

    // initialize local parameters (setting to false generates an error)
    let conditions = LocalInfrastructure {
        are_tests_available: true
    };

    // infinite loop (unless there's an exception?)
    loop {
        conditions.wear_mask();
        conditions.keep_distance(1.0 * si::M);
        conditions.wash_hands(&mut wash_count);
        let _result = conditions.get_tested();

        // catch error 
        match _result {
            Ok(_) => {
              // just  continue with your life
            },
            Err(_) => {
              println!("Houston, we have a problem");
              break;
            }
          }

        // pause a sec (if we get to here)
        thread::sleep(time::Duration::from_secs(1));
            
    }

    println!("This program ends here");
}


