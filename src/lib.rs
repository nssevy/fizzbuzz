pub fn converti_nbr_en_tab(n: u32) -> Vec<u32> {

    let mut tableau_n = Vec::new();

    const DIX: u32 = 10;

    let mut nombre = n; //123

    while nombre != 0 {
        tableau_n.push(nombre % DIX);
        nombre = nombre / DIX;
    }

    tableau_n
} 

pub fn is_multiple_of(nbr: u32) -> (bool, bool) {

    if nbr == 0 {
        return (true, true)
    }

    let tab: Vec<u32> = converti_nbr_en_tab(nbr);
    let mut multiple_de_5: bool = false;
    let mut multiple_de_3: bool = false;

    if nbr % 3 == 0 {
         multiple_de_3 = true;
    }

    if tab[0] == 0 || tab[0] == 5 {
        multiple_de_5 = true;
    }

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

    #[test]//3
    fn check_123_est_egale_a_un_tableau_de_3_2_1() {
        let vec: Vec<u32> = vec![3,2,1]; 
        assert_eq!(vec, converti_nbr_en_tab(123))
    }

    #[test]//2
    fn check_12_est_egale_a_un_tableau_de_2_1() {
        let vec: Vec<u32> = vec![2,1]; 
        assert_eq!(vec, converti_nbr_en_tab(12))
    }

    #[test]//1
    fn check_1_est_egale_a_un_tableau_de_1() {
        let vec: Vec<u32> = vec![1]; 
        assert_eq!(vec, converti_nbr_en_tab(1))
    }
}

/*
Tu vas écrire une fonction fizzbuzz(n: u32) -> String

Selon les règles classiques : multiple de 3 → "Fizz", multiple de 5 → "Buzz",
multiple des deux → "FizzBuzz", sinon le nombre lui-même.
*/
