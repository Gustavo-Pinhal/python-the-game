use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use crate::constantes;

//Retorna a coordenada f64 do mapeamento gráfico
pub fn to_coord_f64(coordenada: i32) -> f64 {
    (coordenada as f64) * constantes::TAMANHO_BLOCO
}

//Retorna a coordenada u32 do mapeamento gráfico
pub fn to_coord_u32(coordenada: i32) -> u32 {
    (coordenada as u32) * (constantes::TAMANHO_BLOCO as u32)
}

//Desenha blocos na tela
pub fn desenhar_bloco(cor: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    //Converter coordenadas de jogo para coordenadas gráficas
    let gui_x: f64 = to_coord_f64(x);
    let gui_y: f64 = to_coord_f64(y);

    //Chamar função rectangle de piston_window
    rectangle(
        cor,
        [gui_x, gui_y, constantes::TAMANHO_BLOCO, constantes::TAMANHO_BLOCO],
        con.transform,
        g,
    );
}

//Desenha retangulo na tela
pub fn desenhar_retangulo(
    cor: Color,
    x: i32,
    y: i32,
    largura: i32,
    altura: i32,
    con: &Context,
    g: &mut G2d,
) {
    //Converter coordenadas de jogo para coordenadas gráficas
    let x = to_coord_f64(x);
    let y = to_coord_f64(y);

    //Chamar função rectangle de piston_window
    rectangle(
        cor,
        [
            x,
            y,
            to_coord_f64(largura),
            to_coord_f64(altura),
        ],
        con.transform,
        g,
    );
}