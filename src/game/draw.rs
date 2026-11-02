use super::{Game, Kind};
use crossterm::{queue, cursor::MoveTo, style::Print, terminal::{Clear, ClearType}};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::io::Write;

// Line sits at the TOP of the ground row (sextant pixels), so feet and obstacles touch it.
// Specks under the line are pebbles, so you can see the ground scroll.
const GROUND: &str = "🬂🬂🬂🬂🬂🬒🬂🬂🬂🬂🬂🬂🬂🬊🬂🬂🬒🬒🬂🬂🬂🬂🬂🬂🬂🬂🬂🬂🬂🬂🬡🬂🬂🬂🬂🬂🬂🬊🬂🬂🬂🬂🬂🬂🬂🬂🬡🬡🬂🬂🬂🬂🬂";

const CRAB_COLOR: Color = Color::Rgb { r: 247, g: 76, b: 0 };

const KELP_COLOR:   Color = Color::Rgb { r: 46,  g: 160, b: 67 };   // green
const CORAL_COLOR:  Color = Color::Rgb { r: 255, g: 110, b: 150 };  // pink
const URCHIN_COLOR: Color = Color::Rgb { r: 120, g: 70,  b: 200 };  // purple
const CLAM_COLOR:   Color = Color::Rgb { r: 230, g: 210, b: 170 };  // sand

// Crabs sextant art
const CRAB_A: [&str; 3] = [
    "🬪🬄🬞 🬏🬉🬜",   // claws open + eye stalks
    " 🬶🬻🬹🬺🬳 ",   // shell
    "🬖🬙🬂🬂🬂🬥🬢",   // legs out
];

const CRAB_B: [&str; 3] = [
    "🬫🬄🬞 🬏🬉🬛",   // claws closed
    " 🬶🬻🬹🬺🬳 ",
    " 🬥🬥🬂🬙🬙 ",   // legs in
];
// Obstacles Sextant Art
// tall (3 rows)
const KELP:   [&str; 3] = ["🬖🬞🬃", "🬠🬃🬗", "🬈🬯🬢"];          // 3 wide, wavy fronds
const CORAL:  [&str; 3] = ["🬏🬓🬞", "🬣▌🬜", "🬞█🬐"];          // 3 wide, branching
const URCHIN: [&str; 3] = ["🬏🬦🬓🬞", "🬈🬷🬲🬅", "🬡██🬒"];       // 4 wide, spiky ball

// low (2 rows)
const CLAM:   [&str; 2] = [" 🬵🬱 ", "🬻🬶🬳🬺"];               // 4 wide, shell

impl Game {
    pub fn draw(&self, out: &mut impl Write, x: u16, y: u16, w: u16, h: u16) -> std::io::Result<()>{ 
        queue!(out, Clear(ClearType::All))?;
        let pattern: Vec<char> = GROUND.chars().collect();
        let start = self.ground_offset as usize;
        let line: String = (0..w as usize)
            .map(|i| pattern[(start + i) % pattern.len()])
            .collect();

        let ground_row = y + h - 1;
        queue!(out, MoveTo(x, ground_row), Print(line))?;
        
        let sprite = if self.player.y > 0.0 {
            &CRAB_A                                                // airborne: no scuttling
        } else if (self.ground_offset as usize / 3) % 2 == 0 {
            &CRAB_A
        } else {
            &CRAB_B
        };

        // 3. draw it row by row, bottom row resting on the ground
        let bottom = ground_row as i32 - 1 - self.player.y.round() as i32;
        let top = bottom + 1 - sprite.len() as i32;
        for (i, row) in sprite.iter().enumerate() {
            let r = top + i as i32;
            if r >= y as i32 && r < ground_row as i32 {            // stay inside the rect
                queue!(out, MoveTo(x + super::PLAYER_X as u16, r as u16), SetForegroundColor(CRAB_COLOR), Print(row), ResetColor)?;
            }
        }

        for o in &self.obstacles {
            let (rows, color) = obstacle_sprite(o.kind);
            // obstacles and ground both snap with the same floor, so they step on the same frame
            let col = o.x as i32 - self.ground_offset as i32;
            let width = o.kind.size().0;
            // clip to the screen so obstacles slide in and out instead of popping
            let skip = (-col).max(0);                      // cells hidden past the left edge
            let take = (w as i32 - col).min(width) - skip; // cells left that fit on screen
            if take <= 0 {
                continue;
            }
            let top = ground_row as i32 - rows.len() as i32;   // bottom row sits on the ground
            for (i, row) in rows.iter().enumerate() {
                let visible: String = row.chars().skip(skip as usize).take(take as usize).collect();
                queue!(out,
                    MoveTo(x + (col + skip) as u16, (top + i as i32) as u16),
                    SetForegroundColor(color),
                    Print(visible),
                    ResetColor,
                )?;
            }
        }



        Ok(())
    }
}

fn obstacle_sprite(kind: Kind) -> (&'static [&'static str], Color) {
    match kind {
        Kind::Kelp   => (&KELP,   KELP_COLOR),
        Kind::Coral  => (&CORAL,  CORAL_COLOR),
        Kind::Urchin => (&URCHIN, URCHIN_COLOR),
        Kind::Clam   => (&CLAM,   CLAM_COLOR),
    }
}
