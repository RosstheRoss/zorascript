/**
 * The region for the game
 * 
 * @remarks
 * 
 * In most cases NA and EU are the same, but there are some small differences mainly involving name encoding.
 */
const enum GameRegion {
    Japan = "JP",
    NorthAmerica = "NA",
    Europe = "EU"
}

/**
 * Specifies the specific Oracle game
 */
const enum Game {
    Ages = "Oracle of Ages",
    Seasons = "Oracle of Seasons"
}

const enum Animal {
    Ricky = 0x0b,
    Dimitri = 0x0c,
    Moosh = 0x0d   
}

const enum ChildQuestion {
    NoEgg = 0,
    YesChicken = 0x2,
}