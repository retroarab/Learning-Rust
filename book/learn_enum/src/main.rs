use std::io::{self};
#[derive(Clone)]

// no more courses today, let me refactor this for learning experience
enum Intrebare {
    Grila {
        text: &'static str,
        variante: Vec<&'static str>,
        corecta: usize,
    },
    Teorie {
        text: &'static str,
        contine: &'static str,
    },
}
// this should use match, right ? I think it's more optimized in compiler
impl Intrebare {
    fn evalueaza_raspuns(&self) -> bool {
        if let Intrebare::Grila {
            text,
            variante,
            corecta,
        } = &self
        {
            println!("{}", text);
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
                return true;
            }
            false
        } else if let Intrebare::Teorie { text, contine } = &self {
            println!("{} \n Raspunsul tau complex este : ", text);
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Nu merge !");
            if temp.contains(contine) {
                return true;
            };
            false
        } else {
            false
        }
    }
}
enum Examene {
    Grila { intrebari: Vec<Intrebare> },
    Combinat { intrebari: Vec<Intrebare> },
}

impl Examene {
    fn get_intrebari(&self) -> &Vec<Intrebare> {
        match self {
            Examene::Grila { intrebari } | Examene::Combinat { intrebari } => intrebari,
        }
    }
    fn sustine_examen(&self) {
        let mut nr_corecte = 0;
        for intrebare in self.get_intrebari() {
            if intrebare.evalueaza_raspuns() {
                nr_corecte += 1;
            }
        }
        println!("Scor: {}/{}", nr_corecte, self.get_intrebari().len());
    }
}

fn main() {
    let q1 = Intrebare::Grila {
        text: "Ce este RUST ?",
        variante: vec![
            "1. Habar nu am boss",
            "2. Un limbaj de programre",
            "3. Corect by default",
        ],
        corecta: (2),
    };
    let grila1 = Examene::Grila {
        intrebari: vec![q1.clone()],
    };
    println!("Primul exman !");
    grila1.sustine_examen();
    let q2 = Intrebare::Teorie {
        text: "O intrebare complet random ?",
        contine: "Cheie!",
    };
    let combinat1 = Examene::Combinat {
        intrebari: vec![q1, q2],
    };
    println!("Al doilea examen !");
    combinat1.sustine_examen();
}
