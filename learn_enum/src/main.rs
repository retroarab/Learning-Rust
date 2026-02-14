use std::io::{self};
#[derive(Clone)]
enum Intrebare {
    Grila {
        text: String,
        variante: Vec<String>,
        corecta: usize,
    },
    Teorie {
        text: String,
    },
}
struct Grila {
    intrebari: Vec<Intrebare>,
}
enum Examene {
    Grila { examen: Grila },
    Combinat { intrebari: Vec<Intrebare> },
}

impl Examene {
    fn sustine_examen_grila(examen: &Grila) {
        let mut nr_corecte = 0;
        println!("Sustii acum examenul !");
        for (i, intrebare) in examen.intrebari.iter().enumerate() {
            if let Intrebare::Grila {
                text,
                variante,
                corecta,
            } = intrebare
            {
                println!("{}. {}", i + 1, text);
                for varianta in variante {
                    println!(" {}", varianta);
                }
                println!("Raspunsul tau este : ");
                let mut raspuns = String::new();
                io::stdin()
                    .read_line(&mut raspuns)
                    .expect("Nu se poate citi tot !");
                let raspuns: usize = raspuns.trim().parse().expect("Nu e numar !");
                if raspuns == *corecta {
                    nr_corecte += 1;
                };
            }
        }
        println!("Felicitari, ai rezolvat {} grile corecte !", nr_corecte)
    }
    fn sustine_examen_teorie(set_intrebari: &Vec<Intrebare>) {
        let mut corect = 0;
        for (i, intrebare) in set_intrebari.iter().enumerate() {
            if let Intrebare::Teorie { text } = intrebare {
                println!("{} \n\n Care este raspunsul tau ? : ", text);
                let mut raspuns = String::new();
                io::stdin()
                    .read_line(&mut raspuns)
                    .expect("Ooos ! S-a stricat");
                if raspuns.contains("Raspunsul Surpriza") {
                    corect += 1;
                }
            };
        }
        println!("Feclitiari, ai facut corect {} intrebari !", corect)
    }
    fn sustine_examen(&self) {
        if let Examene::Grila { examen } = &self {
            Self::sustine_examen_grila(&examen);
        } else if let Examene::Combinat { intrebari } = &self {
            Self::sustine_examen_teorie(intrebari);
        }
    }
}

fn main() {
    let q1 = Intrebare::Grila {
        text: "Ce este RUST ?".to_string(),
        variante: vec![
            String::from("1. Habar nu am boss"),
            String::from("2. Un limbaj de programre"),
            String::from("3. Corect by default"),
        ],
        corecta: (2),
    };
    let grila1 = Examene::Grila {
        examen: Grila {
            intrebari: vec![q1.clone()],
        },
    };
    grila1.sustine_examen();
    let q2 = Intrebare::Teorie {
        text: String::from("O intrebare complet random ?"),
    };
    let combinat1 = Examene::Combinat {
        intrebari: vec![q1, q2],
    };
    combinat1.sustine_examen();
}
