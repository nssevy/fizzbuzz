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

pub fn add_elements_tab(tab: &[u32]) -> u32 {
    tab.iter().sum()
}

pub fn table_de_multiplication(chiffre: u32) -> [u32; 10] {
    let mut tableau = [0; 10];
    for (i, valeur) in tableau.iter_mut().enumerate() {
        *valeur = ((i + 1) as u32) * chiffre;
    }
    tableau
}

pub fn is_multiple_de_3(nbr: u32) -> bool {

    for tab in table_de_multiplication(3) {
        if nbr == tab {
            return true
        }
    }

    false
}

pub fn is_multiple_de_5(nbr: u32) -> bool {

    for tab in table_de_multiplication(5) {
        if nbr == tab {
            return true
        }
    }

    false
}

pub fn fizzbuzz(n: u32) -> String {

    let somme: u32 = add_elements_tab(&converti_nbr_en_tab(n));
    let tuple: (bool, bool) = (is_multiple_de_3(somme), is_multiple_de_5(somme));

    match tuple {
        (true, false) => String::from("Fizz"), //si c'est un multiple de 3.
        (false, true) => String::from("Buzz"), //si c'est un multiple de 5.
        (true, true) => String::from("FizzBuzz"), //si c'est un multiple de 3 et 5.
        (false, false) => n.to_string(),
    }

}

pub fn fizz(n: u32) -> String {
    let somme: u32 = add_elements_tab(&converti_nbr_en_tab(n));

    match is_multiple_de_3(somme) {
        true => String::from("fizz"),
        false => String::from("n'est pas un multiple de 3")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /*Pour connaitre un multiple de 3, il faut que la somme des chiffres
    * d'un nombre soit dans la tableau de 3. De meme pour 5.
    */

    /*
    * Je dois faire une boucle qui compare un nbr avec tout les élements d'un
    * tableau, si à la fin de la boucle mon nbr n'apparait pas dans tableau
    * la boucle renvoi false.
    */

    #[test]//14
    fn check_fizzbuzz_884_renvoie_bien_la_string_buzz(){
        let str: String = "Buzz".into();
        assert_eq!(str, fizzbuzz(884));
    }

    #[test]//13
    fn check_fizzbuzz_324_renvoie_bien_la_string_fizz(){
        let str: String = "Fizz".into();
        assert_eq!(str, fizzbuzz(324));
    }

    #[test]//12
    fn check_fizzbuzz_77_renvoie_bien_la_string_du_nombre_entre(){
        let str: String = "77".into();
        assert_eq!(str, fizzbuzz(77));
    }

    #[test]//11
    fn check_fizzbuzz_78_renvoie_bien_la_string_fizzbuzz(){
        let str: String = "FizzBuzz".into();
        assert_eq!(str, fizzbuzz(78));
    }

    #[test]//10
    fn check_check_que_la_fn_tab_de_multiplication_de_5_renvoi_un_tab_de_mul_de_5(){
        let tab_de_cinq: [u32; 10] = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
        assert_eq!(table_de_multiplication(5), tab_de_cinq)
    }

    #[test]//9
    fn check_que_la_fn_tab_de_multiplication_de_3_renvoi_un_tab_de_mul_de_3(){
        let tab_de_trois: [u32; 10] = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30];
        assert_eq!(table_de_multiplication(3), tab_de_trois)
    }

    #[test]//8
    fn check_la_somme_de_1_2_3_est_un_multiple_de_3(){
        let string: String = "fizz".into();
        assert_eq!(string, fizz(123))
    }

    #[test]//7
    #[should_panic]
    fn check_que_un_20_n_est_pas_un_multiple_de_3() {
        assert!(is_multiple_de_3(20));
    }

    #[test]//6
    fn check_que_un_21_est_un_multiple_de_3() {
        assert!(is_multiple_de_3(21));
    }

    #[test]//5
    fn check_que_la_somme_dun_tableau_4_est_egale_a_4() {
        let vec: Vec<u32> = converti_nbr_en_tab(4); // <- un tableau.
        assert_eq!(4, add_elements_tab(&vec))
    }

    #[test]//4
    fn check_que_la_somme_dun_tableau_1_2_3_est_egale_a_6() {
        let vec: Vec<u32> = converti_nbr_en_tab(123); // <- un tableau.
        assert_eq!(6, add_elements_tab(&vec))
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
