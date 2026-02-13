//! ASCII art constants for all game visuals.

// ─── Title Screen ───────────────────────────────────────────────────────────

pub const TITLE_ART: &str = r#"
    ______ _     _       ____        _   _
   |  ____(_)   | |     |  _ \      | | (_)
   | |__   _ ___| |__   | | | | __ _| |_ _ _ __   __ _
   |  __| | / __| '_ \  | | | |/ _` | __| | '_ \ / _` |
   | |    | \__ \ | | | | |_| | (_| | |_| | | | | (_| |
   |_|    |_|___/_| |_| |____/ \__,_|\__|_|_| |_|\__, |
      _____ _                 _       _            __/ |
     / ____(_)               | |     | |          |___/
    | (___  _ _ __ ___  _   _| | __ _| |_ ___  _ __
     \___ \| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
     ____) | | | | | | | |_| | | (_| | || (_) | |
    |_____/|_|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
"#;

pub const SUBTITLE: &str = "~ Catch fish. Date fish. Find love. ~";

pub const MENU_FISH: &str = r#"
           o  o
      ___/|    |
 ><> /    |    |  ><>
     \____|    |
          |    |
     ><>  |    |  o  o
    ~~~~~~|~~~~|~~~~~~~~~
   ~~~~~~~|~~~~|~~~~~~~~~~
  ~~~~~~~~~\~~/ ~~~~~~~~~~~
"#;

// ─── Fish Characters ────────────────────────────────────────────────────────

pub const BUBBLES_SMALL: &str = r#"  ><(((o>"#;
pub const MARINA_SMALL: &str = r#" --====>"#;
pub const GILL_SMALL: &str = r#"  <o))><"#;

pub const BUBBLES_ART: &str = r#"
       .----.
      / o  o \
     |  .__.  |
     |  |  |  |
      \ '--' /
       '----'
       /|||||\
"#;

pub const BUBBLES_HAPPY: &str = r#"
       .----.
      / ^  ^ \
     |  .__.  |
     |  |><|  |
      \ '--' /
       '----'
       /|||||\
"#;

pub const BUBBLES_SHY: &str = r#"
       .----.
      / -  - \
     |  .__.  |
     |  |..|  |
      \ '--' /
       '----'
       /|||||\
"#;

pub const MARINA_ART: &str = r#"
            _____
    ,------'     '-----.
   /  o               /
  |    ___     ___   <
   \      '---'    \  \
    '-------.------'   |
             \________/
"#;

pub const MARINA_HAPPY: &str = r#"
            _____
    ,------'     '-----.
   /  ^               /
  |    ___     ___   <
   \      '---'    \  \
    '-------.------'   |
             \________/
"#;

pub const MARINA_ANGRY: &str = r#"
            _____
    ,------'     '-----.
   / >/               /
  |    ___     ___   <
   \      '---'    \  \
    '-------.------'   |
             \________/
"#;

pub const GILL_ART: &str = r#"
      .---.
     / o o \
    |   ~   |
     \ ___ /
      '---'
"#;

pub const GILL_PUFFED: &str = r#"
    .--------.
   /  O    O  \
  |            |
  |     ~~     |
  |            |
   \  .----.  /
    '--------'
"#;

pub const GILL_SHY: &str = r#"
      .---.
     / - - \
    |   ~   |
     \ ___ /
      '---'
"#;

// ─── Fishing Scenes ─────────────────────────────────────────────────────────

pub const POND_SCENE: &str = r#"
     __|__
    /     \           .
   | () () |    *         .
    \_____/
       |               .
       |     .
  _____|_____________________
 /                           \
|  ~~~  ><>  ~~~ ~~  ><>  ~~  |
|  ~~ ~~  ~~ ><> ~~ ~~  ~~ ~~ |
|  ~~~ ~~  ~~ ~~  ~~~ ><>  ~~ |
 \___________________________/
"#;

pub const CASTING_ART: &str = r#"
    O
   /|\  ~*
   / \    \
          |
  ~~~~~~~~|~~~~~~~~~
  ~~ ~~ ~~|~~ ~~ ~~~
  ~~~ ~~ ~|~ ~~ ~~~~
"#;

pub const FISH_ON_LINE: &str = r#"
    O     !
   /|\ /--+
   / \    |
          |
  ~~~~~~~~|~~~~~~~~~
  ~~ ~~ ~~|~~ ~~ ~~~
  ~~~ ><(((*~ ~~ ~~~~
"#;

pub const CATCH_SUCCESS: &str = r#"
    \O/
     |  ><(((o>
    / \
  ~~~~~~~~~~~~~~~~~~~~~~
  ~~ ~~ ~~ ~~ ~~ ~~ ~~~~
"#;

pub const CATCH_FAIL: &str = r#"
    _O_
     |     ><>...
    / \
  ~~~~~~~~~~~~~~~~~~~~~~
  ~~ ~~ ~~ ~~ ~~ ~~ ~~~~
"#;

// ─── Fishing Minigame ───────────────────────────────────────────────────────

pub const HOOK_BAR_LEFT: &str = "[";
pub const HOOK_BAR_RIGHT: &str = "]";
pub const HOOK_BAR_FILL: &str = "=";
pub const HOOK_BAR_EMPTY: &str = "-";
pub const HOOK_CURSOR: &str = "|";
pub const HOOK_ZONE: &str = "#";

// ─── Date Scenes ────────────────────────────────────────────────────────────

pub const CORAL_CAFE: &str = r#"
  .=====================.
  |   CORAL CAFE        |
  |  ___          ___   |
  | |   |  {~~}  |   |  |
  | | c |  {~~}  | c |  |
  | |___|________|___|  |
  |   []    []    []    |
  '====================='
"#;

pub const MOONLIT_REEF: &str = r#"
        *  .  *     *  .
     .    *    .  *
   *   .      *     .  *
  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  ~~ /\  ~~  /\ ~~  /\  ~~~~~
  ~ /  \ ~~ /  \ ~ /  \ ~~~~~
  ~/    \~~/    \~/    \~~~~~~
"#;

pub const SUNKEN_SHIP: &str = r#"
  ~~~~~~~~~~~~~~~~~~~~~~~~
  ~~  _______________  ~~~
  ~~ /    |     |    \ ~~~
  ~ |     |  X  |     | ~~
  ~ |_____|_____|_____| ~~
  ~~~~~~~~~~~~~~~~~~~~~~~~
  ~~~ ~~ ~~ ~~ ~~ ~~ ~~~~~
"#;

// ─── UI Elements ────────────────────────────────────────────────────────────

pub const HEART_FULL: &str = "<3";
pub const HEART_EMPTY: &str = "</3";

pub const DIALOGUE_TOP: &str = ".----------------------------------------------------.";
pub const DIALOGUE_BOT: &str = "'----------------------------------------------------'";
pub const DIALOGUE_SIDE: &str = "|";

// ─── Pond Names ─────────────────────────────────────────────────────────────

pub const POND_NAMES: [&str; 3] = ["Sunny Shallows", "Misty Depths", "Crystal Cove"];

// ─── Fish Descriptions ──────────────────────────────────────────────────────

pub const BUBBLES_DESC: &str = "A cheerful clownfish who loves puns and always\nlooks on the bright side. Energetic and warm.";
pub const MARINA_DESC: &str = "An elegant swordfish with a sharp wit and a\ncompetitive streak. Beneath the edge, she cares.";
pub const GILL_DESC: &str = "A shy pufferfish who puffs up when nervous.\nQuiet on the surface, but deeply thoughtful.";
