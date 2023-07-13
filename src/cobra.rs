use std::collections::LinkedList;
use piston_window::{Context, G2d};

use crate::draw::desenhar_bloco;
use crate::constantes;

//Direções de movimentação
#[derive(Copy, Clone, PartialEq)]
pub enum Direcao {
    Up,
    Down,
    Left,
    Right,
}

impl Direcao {
    //Retorna a direção oposta
    pub fn oposto(&self) -> Direcao {
        match *self {
            Direcao::Up => Direcao::Down,
            Direcao::Down => Direcao::Up,
            Direcao::Left => Direcao::Right,
            Direcao::Right => Direcao::Left,
        }
    }
}

//Estrutura do bloco, com as coordenads x, y
#[derive(Debug, Clone, PartialEq)]
pub struct Bloco {
    pub x: i32,
    pub y: i32,
}

//Estrutra da cobra, contendo uma lista encadeada para o corpo e cauda
pub struct Cobra {
    direcao: Direcao,
    corpo: LinkedList<Bloco>,
    cauda: Option<Bloco>,
}

impl Cobra {
    //Função construtora, cria a cobra
    pub fn new(x: i32, y: i32) -> Cobra {
        let mut corpo: LinkedList<Bloco> = LinkedList::new();

        //Inicializa criando três blocos de tamanho
        corpo.push_back(Bloco {
            x: x+2,
            y,
        });
        corpo.push_back(Bloco {
            x: x + 1,
            y,
        });
        corpo.push_back(Bloco {
            x,
            y,
        });
        
        Cobra { 
            direcao: Direcao::Right,
            corpo,
            cauda: None
        }
    }

    //Retorna uma lista encadeada com o corpo da cobra
    pub fn get_corpo(&self) -> LinkedList<Bloco> {
        self.corpo.clone()
    }

    //Chama a função desenhar_bloco para cada bloco do corpo da cobra
    pub fn desenhar(&self, con: &Context, g: &mut G2d) {
        for bloco in &self.corpo {
            desenhar_bloco(constantes::COR_COBRA, bloco.x, bloco.y, con, g);
        }
    }

    //Retorna as coordenadas da cabeça da cobra
    pub fn posicao_cabeca(&self) -> (i32, i32) {
        let bloco_cabeca: &Bloco = self.corpo.front().unwrap();
        (bloco_cabeca.x, bloco_cabeca.y)
    }

    //Movimenta a cobra pelas coordenads do jogo
    pub fn mover_cobra(&mut self, dir: Option<Direcao>) {
        //Identifica se direção é válida ou nula (botão pressionado)
        match dir {
            Some(d) => self.direcao = d,
            None => (),
        }
        //Identifica a posição da cabeça da cobra
        let (ult_x, ult_y): (i32, i32) = self.posicao_cabeca();
        //Cria um novo bloco na posição desejada
        let novo_bloco = match self.direcao {
            Direcao::Up => Bloco {
                x: ult_x,
                y: ult_y - 1,
            },
            Direcao::Down => Bloco {
                x: ult_x,
                y: ult_y + 1,
            },
            Direcao::Left => Bloco {
                x: ult_x - 1,
                y: ult_y,
            },
            Direcao::Right => Bloco {
                x: ult_x + 1,
                y: ult_y,
            },
        };
        //Adiciona o novo bloco à lista e altera a cauda
        self.corpo.push_front(novo_bloco);
        let remover_bloco: Bloco = self.corpo.pop_back().unwrap();
        self.cauda = Some(remover_bloco);
    }

    //Retorna a direcao da cabeça da cobra
    pub fn get_direcao_cabeca(&self) -> Direcao {
        self.direcao
    }

    //Calcula a próxima direção da cabeça
    pub fn next_head(&self, dir: Option<Direcao>) -> (i32, i32) {
        let (cabeca_x, cabeca_y): (i32, i32) = self.posicao_cabeca();
        //Identifica se direção é válida ou nula (botão pressionado) e a atribui a direção de movimento
        let mut dir_mov = self.direcao;
        match dir {
            Some(d) => dir_mov = d,
            None => {},
        }
        //Identifica a próxima posição da cabeça conforme dir_mov
        match dir_mov {
            Direcao::Up => (cabeca_x, cabeca_y - 1),
            Direcao::Down => (cabeca_x, cabeca_y + 1),
            Direcao::Left => (cabeca_x - 1, cabeca_y),
            Direcao::Right => (cabeca_x + 1, cabeca_y),
        }
    }

    //Adicionar a cauda à cobra
    pub fn restaurar_cauda(&mut self) {
        //Cria um bloco cópia da cauda atual
        let blc = self.cauda.clone().unwrap();
        //adiciona esse bloco ao corpo da cobra
        self.corpo.push_back(blc);
    }

    //Retorna true caso a cabeça encontre a cauda, falso caso contrário
    pub fn sobrepoe_cauda(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        //Itera pelo corpo da cobra e identifica se a cabeça está no mesmo bloco
        for bloco in &self.corpo {
            if x == bloco.x && y == bloco.y {
                return  true;
            }
            //Previne bug cabeça colidindo com o final da cauda inesperadamente
            ch += 1;
            if ch == self.corpo.len() - 1 {
                break;
            }
        }
        return false;
    }
}