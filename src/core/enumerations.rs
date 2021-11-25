use std::fmt::Display;

use enum_flags::EnumFlags;
#[derive(Copy, Clone, PartialEq)]
pub enum Axis { None, X, Y }

#[derive(Copy, Clone, PartialEq)]
pub enum ScreenShotFormat { None, Jpg, Bmp, Png }

#[derive(Copy, Clone, PartialEq)]
pub enum ScreenType { None, Storyboard, Title, Select, Versus, Combat, Replay, Options }

#[derive(Copy, Clone, PartialEq)]
pub enum FadeDirection { None, In, Out }

#[derive(Copy, Clone, PartialEq)]
pub enum Assertion { None, Intro, Invisible, RoundNotOver, NoBarDisplay, NoBackground, NoForeground, NoStandGuard, NoAirGuard, NoCrouchGuard, NoAutoturn, NoJuggleCheck, NoKOSound, NoKOSlow, NoShadow, GlobalNoShadow, NoMusic, NoWalk, TimerFreeze, Unguardable, NoKO }

#[derive(Copy, Clone, PartialEq)]
pub enum BindToTargetPostion { None, Foot, Mid, Head }

#[derive(Copy, Clone, PartialEq)]
pub enum Victory { None, Normal, Special, Hyper, NormalThrow, Cheese, Time, Suicude, TeamKill }

#[derive(Copy, Clone, PartialEq)]
pub enum TeamSide { None, Left, Right }

#[derive(Copy, Clone, PartialEq)]
pub enum TeamMode { None, Single, Simul, Turns }

#[derive(Copy, Clone, PartialEq)]
pub enum GameSpeed { Normal, Slow }

#[derive(Copy, Clone, PartialEq)]
pub enum DrawMode { None, Normal, Font, OutlinedRectangle, FilledRectangle, Lines }

#[derive(Copy, Clone, PartialEq)]
pub enum CollisionType { None, PlayerPush, CharacterHit, ProjectileHit, ProjectileCollision }

#[repr(u8)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum AttackStateType { None = 0, Standing = 1, Crouching = 2, Air = 4 }

#[derive(Copy, Clone, PartialEq)]
pub enum AttackPower { None = 0, Normal, Special, Hyper, All }

#[derive(Copy, Clone, PartialEq)]
pub enum AttackClass { None = 0, Normal, Throw, Projectile, All }

#[derive(Copy, Clone, PartialEq)]
pub enum HitFlagCombo { No = 0, Yes, DontCare }

#[repr(u8)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum AffectTeam { None = 0, Enemy = 1, Friendly = 2, Both = 1 | 2 }

#[derive(Copy, Clone, PartialEq)]
pub enum HitAnimationType { None = 0, Light, Medium, Hard, Back, Up, DiagUp }

#[derive(Copy, Clone, PartialEq)]
pub enum PriorityType { None, Hit, Dodge, Miss }

#[derive(Copy, Clone, PartialEq)]
pub enum AttackEffect { None = 0, High, Low, Trip }

#[derive(Copy, Clone, PartialEq)]
pub enum HelperType { Normal = 0, Player, Projectile }

#[derive(Copy, Clone, PartialEq)]
pub enum PositionType { None = 0, P1, P2, Front, Back, Left, Right }

#[derive(Copy, Clone, PartialEq)]
pub enum ClsnType { None, Type1Attack, Type2Normal }

impl Default for ClsnType {
    fn default() -> Self { ClsnType::None }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Facing { Left, Right }

#[derive(Copy, Clone, PartialEq)]
pub enum BlendType { None, Add, Subtract }

impl Default for BlendType {
    fn default() -> Self { BlendType::None }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BackgroundLayer { Front, Back }

#[derive(Copy, Clone, PartialEq)]
pub enum NumberType { None, Int, Float }

#[derive(EnumFlags, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrintJustification { Left, Right, Center }

impl Default for PrintJustification {
    fn default() -> Self { PrintJustification::Center }
}

impl From<i16> for PrintJustification {
    fn from(just: i16) -> PrintJustification {
        let mut justification = PrintJustification::Center;

        if just > 0 {
            justification = PrintJustification::Left
        } else if just < 0 {
            justification = PrintJustification::Right;
        }

        return justification;
    }
}

#[repr(u16)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum PlayerButton { None = 0, Up = 1, Down = 2, Left = 4, Right = 8, A = 16, B = 32, C = 64, X = 128, Y = 256, Z = 512, Taunt = 1024, Pause = 2048 }

#[repr(u16)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum SystemButton {
    None = 0,
    Pause = 1,
    PauseStep = 2,
    Quit = 4,
    DebugDraw = 8,
    FullLifeAndPower = 16,
    TestCheat = 32,
    TakeScreenshot = 64,
    SetPlayer1LifeToZero = 128,
    SetPlayer2LifeToZero = 256,
    SetBothPlayersLifeToOne = 512,
    TimeOver = 1024
}

#[derive(Copy, Clone, PartialEq)]
pub enum RoundState { None, PreIntro, Intro, Fight, PreOver, Over }

#[derive(Copy, Clone, PartialEq)]
pub enum IntroState { None, Running, RoundNumber, Fight }

#[derive(Copy, Clone, PartialEq)]
pub enum CommandDirection { None = 0, B, DB, D, DF, F, UF, U, UB, B4Way, U4Way, F4Way, D4Way }

#[derive(Copy, Clone, PartialEq)]
pub enum StateType { None, Unchanged, Standing, Crouching, Airborne, Prone }

#[derive(Copy, Clone, PartialEq)]
pub enum MoveType { None, Idle, Attack, BeingHit, Unchanged }

#[derive(Copy, Clone, PartialEq)]
pub enum Physics { None, Unchanged, Standing, Crouching, Airborne }

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerControl { Unchanged, InControl, NoControl }

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerMode { Human, Ai }

#[repr(u8)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum CommandButton { None = 0, A = 1, B = 2, C = 4, X = 8, Y = 16, Z = 32, Taunt = 64 }

#[repr(u8)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum ForceFeedbackType { None = 0, Sine = 1, Square = 2 }

#[derive(Copy, Clone, PartialEq)]
pub enum ButtonState { Up, Down, Pressed, Released }

#[derive(Copy, Clone, PartialEq)]
pub enum ProjectileDataType { None, Hit, Guarded, Cancel }

#[derive(Copy, Clone, PartialEq)]
pub enum PauseState { Unpaused, Paused, PauseStep }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MainMenuOption { Arcade = 0, Versus = 1, TeamArcade = 2, TeamVersus = 3, TeamCoop = 4, Survival = 5, SurvivalCoop = 6, Training = 7, Watch = 8, Options = 9, Quit = 10 }

#[derive(Copy, Clone, PartialEq)]
pub enum EntityUpdateOrder { Character, Projectile, Explod }

#[derive(Copy, Clone, PartialEq)]
pub enum ProjectileState { Normal, Removing, Canceling, Kill }

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerSelectType { Profile, Random }

#[derive(Copy, Clone, PartialEq)]
pub enum CursorDirection { Up, Down, Left, Right }

#[derive(Copy, Clone, PartialEq)]
pub enum ElementType { None, Static, Animation, Text }

#[derive(Copy, Clone, PartialEq)]
pub enum CombatMode { None, Arcade, Versus, TeamArcade, TeamVersus, TeamCoop, Survival, SurvivalCoop, Training }

#[repr(u16)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
pub enum SpriteEffects {
    None = 0b0,
    FlipHorizontally = 0b01,
    FlipVertically = 0b10
}

impl Display for SpriteEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}{}",
            if self.contains(SpriteEffects::FlipHorizontally) { "H" } else { "" },
            if self.contains(SpriteEffects::FlipVertically) { "V" } else { "" }
        ))
    }
}
