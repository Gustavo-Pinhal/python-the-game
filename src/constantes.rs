use piston_window::{types::Color, Key};

//Constantes gameplay
pub const PERIODO_MOVIMENTO: f64 = 0.20 ;
pub const TEMPO_RESTART: f64 = 1.0; 

//Cores
pub const COR_COBRA: Color = [0.15, 0.35, 0.65, 1.0];
pub const COR_FUNDO: Color = [0.08, 0.20, 0.32, 1.0];
pub const COR_FRUTA: Color = [0.90, 0.76, 0.20, 1.0];
pub const COR_BORDA: Color = [0.04, 0.10, 0.16, 1.0];
pub const COR_GAMEOVER: Color = [0.90, 0.00, 0.00, 0.5];

//Tela
pub const LARGURA_TELA: i32 = 30;
pub const ALTURA_TELA: i32 = 30;
pub const TAMANHO_BLOCO: f64 = 20.0;

//Teclas de movimentação
pub const MOV_UP: Key = Key::W;
pub const MOV_DOWN: Key = Key::S;
pub const MOV_LEFT: Key = Key::A;
pub const MOV_RIGHT: Key = Key::D;