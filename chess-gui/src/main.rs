use jblomlof_chess::{GameState, Game};

use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use std::{collections::HashMap, env, path};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 10;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (32 * 4, 32 * 4);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = 
(
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color = graphics::Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: graphics::Color = graphics::Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);
const HIGHLIGHT: graphics::Color = graphics::Color::new(230.0 / 255.0, 200.0 / 255.0, 50.0 / 255.0, 0.5);
    
/// GUI logic and event implementation structure.
struct AppState 
{
    sprites: HashMap<char, graphics::Image>,
    // Imported game representation.
    game: Game,

    selectedPiece: Option<(u32, u32)>,
    possibleMoves: Vec<(u32, u32)>
}

impl AppState 
{
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> 
    {
        let state = AppState 
        {
            sprites: AppState::load_sprites(ctx),

            game: Game::new(),

            selectedPiece: None,
            
            possibleMoves: vec![]
        };

        Ok(state)
    }
    #[rustfmt::skip] // Skips formatting on this function (not recommended)
    /// Loads chess piese images into hashmap, for ease of use.
    fn load_sprites(ctx: &mut Context) -> HashMap<char, graphics::Image> 
    {
        [
            ('k', "/black_king.png".to_string()),
            ('q', "/black_queen.png".to_string()),
            ('r', "/black_rook.png".to_string()),
            ('p', "/black_pawn.png".to_string()),
            ('b', "/black_bishop.png".to_string()),
            ('n', "/black_knight.png".to_string()),
            ('K', "/white_king.png".to_string()),
            ('Q', "/white_queen.png".to_string()),
            ('R', "/white_rook.png".to_string()),
            ('P', "/white_pawn.png".to_string()),
            ('B', "/white_bishop.png".to_string()),
            ('N', "/white_knight.png".to_string()),
            ('s', "/shadow.png".to_string())
        ]
            .iter()
            .map(|(piece, path)| 
            {
                (*piece, graphics::Image::new(ctx, path).unwrap())
            })
            .collect::<HashMap<char, graphics::Image>>()
    }
}

fn transform_input(input_pos: &str) -> (u32, u32) 
{
    let mut chars_iter = input_pos.chars();
    (
        chars_iter
            .next()
            .unwrap()
            .to_digit(18)
            .unwrap()
            - 10,
        chars_iter.next().unwrap().to_digit(10).unwrap() - 1,
    )
}

fn transform_back(file_input: u32, rank_input: u32) -> String 
{
    return ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
        .get(file_input as usize)
        .expect("Error on file_rank")
        .to_string()
        + &(rank_input + 1).to_string();
}

impl event::EventHandler<GameError> for AppState 
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult 
    {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        graphics::clear(ctx, [0.27, 0.20, 0.24, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new
        (
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state()))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle
        (
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new
            (
                (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                text_dimensions.w as f32 + 16.0,
                text_dimensions.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default())
            .expect("Failed to draw background.");

        // draw tiles
        for row in 0..8 
        {
            for col in 0..8 
            {
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle
                (
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32
                    (
                        (col + 1) * GRID_CELL_SIZE.0 as i32,
                        (row + 1) * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match col % 2 
                    {
                        0 => {
                            if row % 2 == 0 
                            {
                                WHITE
                            } else {
                                BLACK
                            }
                        }
                        _ => {
                            if row % 2 == 0 
                            {
                                BLACK
                            } else {
                                WHITE
                            }
                        }
                    },
                )
                .expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");
            }
        }
        
        for i in 0..self.possibleMoves.len()
        {
            let highlight = self.possibleMoves[i];
            // draw tile
            let rectangle = graphics::Mesh::new_rectangle
            (
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32
                (
                    (highlight.0 + 1) as i32 * GRID_CELL_SIZE.0 as i32,
                    (highlight.1 + 1) as i32 * GRID_CELL_SIZE.1 as i32,
                    GRID_CELL_SIZE.0 as i32,
                    GRID_CELL_SIZE.1 as i32,
                ),
                HIGHLIGHT,
            )
            .expect("Failed to create tile.");
            graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                .expect("Failed to draw tiles.");
        }

        let board = self.game.get_board();

        let mut i = 0;
        for c in board.chars()
        {
            let x = i % 8;
            let y = 7 - (i / 8);
            if c != '\n' { i += 1; }
            
            // draw piece
            if self.sprites.contains_key(&c)
            {
                graphics::draw
                (
                    ctx,
                    self.sprites.get(&'s').unwrap(),
                    graphics::DrawParam::default()
                        .scale([4.0, 4.0]) 
                        .dest
                        ([
                            (x + 1) as f32 * GRID_CELL_SIZE.0 as f32,
                            (y + 1) as f32 * GRID_CELL_SIZE.1 as f32,
                        ]),
                )
                .expect("Failed to draw shadow.");
                graphics::draw
                (
                    ctx,
                    self.sprites.get(&c).unwrap(),
                    graphics::DrawParam::default()
                        .scale([4.0, 4.0]) 
                        .dest
                        ([
                            (x + 1) as f32 * GRID_CELL_SIZE.0 as f32,
                            ((y + 1) as f32 * GRID_CELL_SIZE.1 as f32) - 14.0 * 4.0,
                        ]),
                )
                .expect("Failed to draw piece.");
            }
        }


        // draw text with dark gray colouring and center position
        graphics::draw
        (
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([1.0, 0.95, 0.85, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                    y: (28.0 as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event
    (
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) 
    {
        if button == event::MouseButton::Left 
        {
            let anySquare = (x / GRID_CELL_SIZE.0 as f32, y / GRID_CELL_SIZE.1 as f32);
            if anySquare.0 <= 0.0 || anySquare.0 >= 9.0 || anySquare.1 <= 0.0 || anySquare.1 >= 9.0 { return; };

            let square = ((anySquare.0 - 1.0) as u32, (anySquare.1 - 1.0) as u32);

            for i in 0..self.possibleMoves.len()
            {
                let mv = self.possibleMoves[i];
                if mv.0 == square.0 && mv.1 == square.1
                {
                    self.game.make_move
                    (
                        &transform_back(self.selectedPiece.unwrap().0, self.selectedPiece.unwrap().1),
                        &transform_back(square.0, square.1)
                    );
                    self.selectedPiece = None;
                    self.possibleMoves.clear();
                    return;
                }
            }

            let clickedPiece: char = self.game.get_board().as_bytes()[(square.0 + (7 - square.1) * 9) as usize] as char;

            //println!("{}", clickedPiece);

            if clickedPiece == '*'
            {
                self.selectedPiece = None;
                self.possibleMoves.clear();
                return;
            }

            if clickedPiece.is_uppercase()
            {
                if !self.game.is_white_turn()
                { 
                    self.selectedPiece = None;
                    self.possibleMoves.clear();
                    return; 
                }
            }
            else 
            {
                if self.game.is_white_turn()
                { 
                    self.selectedPiece = None;
                    self.possibleMoves.clear();
                    return; 
                }
            }
            
            if self.selectedPiece.is_some()
            {
                if self.selectedPiece.unwrap().0 == square.0 && self.selectedPiece.unwrap().1 == square.1 
                { 
                    self.selectedPiece = None;
                    self.possibleMoves.clear();
                    return;
                }
            }

            self.selectedPiece = Some(square);

            self.possibleMoves.clear();

            let moves = self.game.get_possible_moves(&transform_back(square.0, square.1));
            if moves.is_some()
            {
                let m = moves.unwrap();
                for i in m
                {
                    self.possibleMoves.push(transform_input(&i));
                }
            }
        }
    }
}

pub fn main() -> GameResult 
{
    let resource_dir = path::PathBuf::from("./res");

    let context_builder = ContextBuilder::new("schack", "melvin")
        .add_resource_path(resource_dir) 
        .window_setup
        (
            conf::WindowSetup::default()
                .title("Schack med gulliga svampar") // Set window title "Schack"
                .icon("/icon.png"), // Set application icon
        )
        .window_mode
        (
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        );
    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");

    graphics::set_default_filter(&mut contex, graphics::FilterMode::Nearest);

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}