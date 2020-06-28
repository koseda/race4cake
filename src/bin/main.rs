///! This is a school assignment
///!
///! ASSIGNMENT (translated from czech): Eight long jump competitors achieved results in three attempts as shown above. Determine in column E the longest jump of the competitor, in column F the order of the competitor and in column G whether or not he will receive a cake (only the first three). Assign a colored background to the rows of competitors who placed in the first three places (the first three places are color-coded).
///!
///! See assignment.ods
///!
///! Analysis
///! - Find out longest jump from E row and append competetor's rank from row F
///! - Decide if he gets a cake or not (from row G)

// ABSTRACTING

use jump4cake;

fn main() {
	// Create the spreadsheet
	jump4cake::assignment_spreadsheet();
}