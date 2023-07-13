extern crate rand;
extern crate piston_window;

mod constantes;
mod draw;
mod cobra;
mod game;

use draw::to_coord_u32;
use piston_window::*;

use crate::game::Game;

fn main() {
    //Criar a tela
    let mut janela: PistonWindow = WindowSettings::new(
        "Python The Game 0.2",
        [
            to_coord_u32(constantes::LARGURA_TELA),
            to_coord_u32(constantes::ALTURA_TELA)
        ]
    ).exit_on_esc(true)
    .build()
    .unwrap();
    
    //Criar jogo
    let mut game = Game::new(constantes::LARGURA_TELA, constantes::ALTURA_TELA);

    //Enquanto a tela não for fechada
    while let Some(evento) = janela.next() {
        //Identificar a tecla pressionada
        if let Some(Button::Keyboard(key)) = evento.press_args() {
            game.tecla_pressionada(key);
        }
        //Desenhar a janela
        janela.draw_2d(&evento, |c, g, _device| {
            clear(constantes::COR_FUNDO, g);
            game.draw(&c, g);
            
        });
        //Update na tela e no jogo
        evento.update(|arg| {
            game.update(arg.dt);
        });
    }
}
