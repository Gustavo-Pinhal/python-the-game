use piston_window::*;

use std::collections::LinkedList;

use rand::{thread_rng, Rng};

use crate::cobra::{Direcao, Cobra, Bloco};
use crate::draw::{desenhar_bloco, desenhar_retangulo};
use crate::constantes;

//Estrutura do jogo
pub struct Game {
    snake: Cobra,

    fruta_existe: bool,
    fruta_x: i32,
    fruta_y: i32,

    largura: i32,
    altura: i32,

    game_over: bool,
    tempo_espera: f64,
}

impl Game {
    //Método construtor, cira o jogo
    pub fn new(largura: i32, altura: i32) -> Game {
        Game {
            snake: Cobra::new(2, 2),
            tempo_espera: 0.0,
            fruta_existe: true,
            fruta_x: 6,
            fruta_y: 4,
            largura,
            altura,
            game_over: false
        }
    }

    //Verifica a direção do botão e update a direção da cobra
    pub fn tecla_pressionada(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        //Identificar a direção
        let dir: Option<Direcao> = match key {
            constantes::MOV_UP =>Some(Direcao::Up),
            constantes::MOV_DOWN =>Some(Direcao::Down),
            constantes::MOV_LEFT =>Some(Direcao::Left),
            constantes::MOV_RIGHT =>Some(Direcao::Right),
            _=> None
        };
        //Caso a tecla da direção oposta seja pressionada, não fazer nada
        if dir.unwrap() == self.snake.get_direcao_cabeca().oposto() {
            return;
        }
        //update da cobra passando uma direção válida
        self.update_cobra(dir);
    }

    //Função que coordena os desenhos na tela
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        //Chama função que desenha a cobra
        self.snake.desenhar(con, g);
        //Caso a fruta exista chama função de desenhar bloco
        if self.fruta_existe {
            desenhar_bloco(constantes::COR_FRUTA, self.fruta_x, self.fruta_y, con, g);
        }
        //Chama função de desenhar retangulo para criar as bordas do mapa
        desenhar_retangulo(constantes::COR_BORDA, 0, 0, self.largura, 1, con, g);
        desenhar_retangulo(constantes::COR_BORDA, 0, self.altura - 1, self.largura, 1, con, g);
        desenhar_retangulo(constantes::COR_BORDA, 0, 0, 1, self.altura, con, g);
        desenhar_retangulo(constantes::COR_BORDA, self.largura - 1, 0, 1, self.altura, con, g);
        //Caso game over desenha a tela de game over
        if self.game_over{
            desenhar_retangulo(constantes::COR_GAMEOVER, 0, 0, self.largura, self.altura, con, g);
        }
    }

    //Update no estado do jogo
    pub fn update(&mut self, delta_tempo: f64) {
        self.tempo_espera += delta_tempo;
        //Restartar o jogo caso game_over
        if self.game_over {
            if self.tempo_espera > constantes::TEMPO_RESTART {
                self.restart();
            }
            return;
        }
        //Adicionar fruta caso ela não exista
        if !self.fruta_existe {
            self.adicionar_fruta();
        }
        //Update com direção nula na direção da cobra caso o tempo tenha sido concluído
        if self.tempo_espera > constantes::PERIODO_MOVIMENTO {
            self.update_cobra(None);
        }
    }

    //Verifica se a cobra comeu a fruta e chama restaurar_cauda caso true
    fn checar_comendo(&mut self) {
        //obter as coordenadas da cabeça
        let(cabeca_x, cabeca_y): (i32, i32) = self.snake.posicao_cabeca();
        //verificar se cabeça == fruta
        if self.fruta_existe && self.fruta_x == cabeca_x && self.fruta_y == cabeca_y {
            self.fruta_existe = false;
            self.snake.restaurar_cauda();
        }
    }

    //Verifica se a cobra colidiu com ela mesma ou com a parede
    fn checar_cobra_viva(&self, dir: Option<Direcao>) -> bool {
        let (prox_x, prox_y) = self.snake.next_head(dir);
        //Verifica colisão com sigo mesma e retorna false caso sim
        if self.snake.sobrepoe_cauda(prox_x, prox_y) {
            return false;
        }
        //Verifica colisão com a borda e retorna true ou false
        prox_x > 0 && prox_y > 0 && prox_x < self.largura - 1 && prox_y < self.altura - 1
    }

    //Retorna uma tupla aleatória
    fn get_tupla_rand(&self) -> (i32, i32) {
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let x: i32 = rng.gen_range(1..=self.largura - 2);
        let y: i32 = rng.gen_range(1..=self.altura - 2);
        return (x, y);
    }

    //Adiciona a fruta ao jogo
    fn adicionar_fruta(&mut self) {
        //Validade da posição da fruta
        let mut is_valido: bool;
        //Procurar uma posição válida para spawnar a fruta
        loop {
            //Obtém coordenadas aleatórias
            let (novo_x, novo_y) = self.get_tupla_rand();
            let corpo: LinkedList<Bloco> = Cobra::get_corpo(&self.snake);
            is_valido = true;
            //Iterar pelo corpo da cobra
            for bloco in &corpo {
                //Verifica se as coordenadas da fruta se igualam aos blocos do corpo da cobra
                if (novo_x, novo_y) == (bloco.x, bloco.y) {
                    //validade se torna falsa neste caso
                    is_valido = false;
                }
            }
            //Caso validade seja true, spawna a fruta
            if is_valido {
                self.fruta_x = novo_x;
                self.fruta_y = novo_y;
                self.fruta_existe = true;
                return;
            }
        }
    }

    //Update da cobra
    fn update_cobra(&mut self, dir: Option<Direcao>) {
        //Caso true mover e checar_comendo, caso false game_over
        if self.checar_cobra_viva(dir) {
            self.snake.mover_cobra(dir);
            self.checar_comendo();
        } else {
            self.game_over = true;
        }
        self.tempo_espera = 0.0;
    }

    //Restart do jogo
    fn restart(&mut self) {
        //Re-spawnar cobra e fruta, gamer_over false
        self.snake = Cobra::new(2, 2);
        self.tempo_espera = 0.0;
        self.fruta_existe = true;
        self.fruta_x = 6;
        self.fruta_y = 4;
        self.game_over = false;
    }
}