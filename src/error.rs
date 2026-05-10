use {
    serde_json::Error as SerdeError,
    thiserror::Error,
    std::io::Error as IoError
    };



/** **Technical part** - type to represent a point data parsing error. */
#[derive(Debug, Error)]
pub enum ParsePointError {
    /** Input was malformed. */
    #[error("Invalid formatting of the point data")]
    InvalidFormat,
    /** Parsing `id` failed. */
    #[error("Failed to parse the point's ID")]
    FailedParseId,
    /** Parsing `x` failed. */
    #[error("Failed to parse the point's x coordinate")]
    FailedParseXCoord,
    /** Parsing `y` failed. */
    #[error("Failed to parse the point's y coordinate")]
    FailedParseYCoord,
    /** Parsing `food` failed. */
    #[error("Failed to parse the point's food amount")]
    FailedParseFoodAmount,
    /** `x` is out of acceptable range. */
    #[error("Point's x coordinate is out of range")]
    XCoordOutOfRange,
    /** `y` is out of acceptable range. */
    #[error("Point's y coordinate is out of range")]
    YCoordOutOfRange
    }

/** **Technical part** - type to represent a action data parsing error. */
#[derive(Debug, Error)]
pub enum ParseActionError {
    /** Input was malformed. */
    #[error("Invalid formatting of the action data")]
    InvalidFormat,
    /** Parsing `cycle` failed. */
    #[error("Failed to parse the action's cycle")]
    FailedParseCycle,
    /** Parsing `id` failed. */
    #[error("Failed to parse the action's ID")]
    FailedParseId,
    /** Parsing `food` failed. */
    #[error("Failed to parse the action's food amount")]
    FailedParseFoodAmount
    }

/** **Technical part** - type to represent a possible application's runtime errors. */
#[derive(Debug, Error)]
pub enum RuntimeError {
    /** Error caused by failing input validity assertion. */
    #[error("Assertion failed: {0}")]
    Assert(#[from] AssertionError),
    /** Error caused by a shortage of food sources occured during simulation. */
    #[error(transparent)]
    NoFoodsource(#[from] NoFoodsourceError),
    /** Error caused by trying to save statistics to file. */
    #[error("An error occured while trying to save: {0}")]
    File(#[from] SaveError)
    } 

/** **Technical part** - type to represent a possible assertion failure causes. */
#[derive(Debug, Error)]
pub enum AssertionError {
    /** Error caused by points with identical IDs. */
    #[error("The context has non-unique point IDs")]
    NonUniquePointIds,
    /** Error caused by points with identical positions. */
    #[error("The context has non-unique point positions")]
    NonUniquePointPositions,
    /** Error caused by passing more decision points than real points. */
    #[error("The context has more decison points than true points")]
    InvalidDecisionPoints,
    /** Error caused by passing pheromone strength which is out of acceptable range. */
    #[error("The context has pheromone strength out of range")]
    PheromoneOutsideOfRange,
    /** Error caused by passing dispersion coefficient which is out of acceptable range. */
    #[error("The context has dispersion coefficient out of range")]
    InvalidDispersionCoefficient,
    /** Error caused by actions containing invalid IDs. */
    #[error("The context has action with non-existant IDs")]
    NonOverlappingActionIds,
    /** Error caused by anthill reciving any amount of food during simulation - it should remain empty. */
    #[error("The context has or sets food amount to the anthill")]
    NonEmptyAnthill
    }

/** **Technical part** - type to represent a possible food source shortage. */
#[derive(Debug, Error)]
#[error("Available food sources ran out")]
pub struct NoFoodsourceError;

/** **Technical part** - type to represent a possible statistics' saving errors. */
#[derive(Debug, Error)]
pub enum SaveError {
    /** Error caused by problems with file handling. */
    #[error(transparent)]
    IO(#[from] IoError),
    /** Error caused by problems with parsing/writing JSON values. */
    #[error(transparent)]
    Serde(#[from] SerdeError)
    }