pub fn is_multiple_of(nbr: u32) -> (bool, bool) {


    let multiple_de_3 = nbr % 3 == 0;
    let multiple_de_5 = nbr % 5 == 0;

    (multiple_de_3, multiple_de_5)
}

pub fn fizzbuzz(n: u32) -> String {

    let tuple: (bool, bool) = is_multiple_of(n);

    match tuple {
        (true, false) => String::from("Fizz"), //si c'est un multiple de 3.
        (false, true) => String::from("Buzz"), //si c'est un multiple de 5.
        (true, true) => String::from("FizzBuzz"), //si c'est un multiple de 3 et 5.
        (false, false) => n.to_string(),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]//9
    fn check_15_is_renvoie_bien_fizzbuzz(){
        assert_eq!(String::from("FizzBuzz"),fizzbuzz(15));
    }

    #[test]//9
    fn check_235_is_renvoie_bien_buzz(){
        assert_eq!(String::from("Buzz"),fizzbuzz(235));
    }

    #[test]//8
    fn check_is_multiple_of_220_renvoie_bien_false_true(){
        assert_eq!((false,true),is_multiple_of(220));
    }

    #[test]//7
    fn check_is_multiple_of_0_renvoie_bien_true_true(){
        assert_eq!((true,true),is_multiple_of(0));
    }

    #[test]//6
    fn check_fizzbuzz_884_renvoie_bien_884(){
        let str: String = "884".into();
        assert_eq!(str, fizzbuzz(884));
    }

    #[test]//5
    fn check_fizzbuzz_324_renvoie_bien_la_string_fizz(){
        let str: String = "Fizz".into();
        assert_eq!(str, fizzbuzz(324));
    }

    #[test]//4
    fn check_fizzbuzz_78_renvoie_bien_fizz(){
        let str: String = "Fizz".into();
        assert_eq!(str, fizzbuzz(78));
    }
}

/*
Tu vas écrire une fonction fizzbuzz(n: u32) -> String

Selon les règles classiques : multiple de 3 → "Fizz", multiple de 5 → "Buzz",
multiple des deux → "FizzBuzz", sinon le nombre lui-même.
*/
