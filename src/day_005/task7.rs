#[allow(dead_code)]
enum Error {
    Errone(String),
    Errtwo(String),
}

fn return_err(a: f64, b: f64) -> Result<f64, Error> {
    if b != 0.0 {
        Ok(a / b)
    } else if b == 0.0 {
        Err(Error::Errone("Error, cannot divide by float".to_string()))
    } else {
        Err(Error::Errone("Error, cannot divide by zero".to_string()))
    }
}

pub fn task57() {

    let answer: Result<f64, Error> = return_err(1.0, 0.0);

    match answer {
        Ok(result) => println!("{}", result),
        Err(Error::Errone(e)) => println!("{}", e),
        Err(Error::Errtwo(e)) => println!(" {}", e),

    }


}