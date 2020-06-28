use spsheet::{Book,Sheet,Cell,ods};
use std::path::Path;

// Create the requested 'ods' type file with required info
// FIXME-CREATE: Creatable?
pub fn assignment_spreadsheet() {
	let mut book = spsheet::Book::new();
	let mut sheet = spsheet::Sheet::new("assignment");

	// FIXME: The table has a 1px black border
	// - Spsheet doesn't support 'Cell Border'

	// Crete the Introduction
	sheet.add_cell(Cell::str("Skok do dálky", ""), 0, 0);

	// Create the header
	sheet.add_cell(Cell::str("Jméno", ""), 3, 0);
	sheet.add_cell(Cell::str("Pokus 1", ""), 3, 1);
	sheet.add_cell(Cell::str("Pokus 2", ""), 3, 2);
	sheet.add_cell(Cell::str("Pokus 3", ""), 3, 3);
	sheet.add_cell(Cell::str("Maximum", ""), 3, 4);
	sheet.add_cell(Cell::str("Pořadí", ""), 3, 5);
	sheet.add_cell(Cell::str("DORT?", ""), 3, 6);

	// Add Contendors
	// - Baumgartner
  sheet.add_cell(Cell::str("Baumgartner", ""), 4, 0);
  sheet.add_cell(Cell::float(7.84, ""), 4, 1);
	sheet.add_cell(Cell::float(7.77, ""), 4, 2);
	sheet.add_cell(Cell::float(7.92, ""), 4, 3);
	sheet.add_cell(Cell::float(7.92, ""), 4, 4);
	// FIXME: Library doesn't have int
	//sheet.add_cell(Cell::int(6), 4, 5);
	sheet.add_cell(Cell::str("6", ""), 4, 5);
	sheet.add_cell(Cell::str("dort nebude", ""), 4, 6);

	// - de Oliveira
	sheet.add_cell(Cell::str("de Oliveira", ""), 5, 0);
	sheet.add_cell(Cell::float(8.00, ""), 5, 1);
	sheet.add_cell(Cell::float(7.98, ""), 5, 2);
	sheet.add_cell(Cell::float(8.04, ""), 5, 3);
	sheet.add_cell(Cell::float(8.04, ""), 5, 4);
	sheet.add_cell(Cell::str("4", ""), 5, 5);
	sheet.add_cell(Cell::str("dort nebude", ""), 5, 6);

	// - Podlužnyj
	sheet.add_cell(Cell::str("de Oliveira", ""), 6, 0);
	sheet.add_cell(Cell::float(7.88, ""), 6, 1);
	sheet.add_cell(Cell::float(7.89, ""), 6, 2);
	sheet.add_cell(Cell::str("-", ""), 6, 3);
	sheet.add_cell(Cell::float(7.89, ""), 6, 4);
	sheet.add_cell(Cell::str("7", ""), 6, 5);
	sheet.add_cell(Cell::str("dort nebude", ""), 6, 6);

	// - Robinson
	// FIXME: Add gold background
	sheet.add_cell(Cell::str("Robinson", ""), 7, 0);
	sheet.add_cell(Cell::float(8.65, ""), 7, 1);
	sheet.add_cell(Cell::float(8.44, ""), 7, 2);
	sheet.add_cell(Cell::float(8.45, ""), 7, 3);
	sheet.add_cell(Cell::float(8.65, ""), 7, 4);
	sheet.add_cell(Cell::str("1", ""), 7, 5);
	sheet.add_cell(Cell::str("dort bude", ""), 7, 6);

	// - Rousseau
	// FIXME: Add bronze background
	sheet.add_cell(Cell::str("Rousseau", ""), 8, 0);
	sheet.add_cell(Cell::float(8.00, ""), 8, 1);
	sheet.add_cell(Cell::float(7.99, ""), 8, 2);
	sheet.add_cell(Cell::float(8.16, ""), 8, 3);
	sheet.add_cell(Cell::float(8.16, ""), 8, 4);
	sheet.add_cell(Cell::str("3", ""), 8, 5);
	sheet.add_cell(Cell::str("dort bude", ""), 8, 6);

	// - Stekič
	sheet.add_cell(Cell::str("Stekič", ""), 9, 0);
	sheet.add_cell(Cell::float(7.89, ""), 9, 1);
	sheet.add_cell(Cell::str("-", ""), 9, 2);
	sheet.add_cell(Cell::str("-", ""), 9, 3);
	sheet.add_cell(Cell::float(7.89, ""), 9, 4);
	sheet.add_cell(Cell::str("7", ""), 9, 5);
	sheet.add_cell(Cell::str("dort nebude", ""), 9, 6);

	// - Wartenberg
	sheet.add_cell(Cell::str("Wartenberg", ""), 10, 0);
	sheet.add_cell(Cell::float(8.02, ""), 10, 1);
	sheet.add_cell(Cell::float(8.02, ""), 10, 2);
	sheet.add_cell(Cell::float(8.02, ""), 10, 3);
	sheet.add_cell(Cell::float(8.02, ""), 10, 4);
	sheet.add_cell(Cell::str("5", ""), 10, 5);
	sheet.add_cell(Cell::str("dort nebude", ""), 10, 6);

	// - Williams
	// FIXME: Add silver background
	sheet.add_cell(Cell::str("Williams", ""), 11, 0);
	sheet.add_cell(Cell::float(8.11, ""), 11, 1);
	sheet.add_cell(Cell::float(8.05, ""), 11, 2);
	sheet.add_cell(Cell::float(8.22, ""), 11, 3);
	sheet.add_cell(Cell::float(8.22, ""), 11, 4);
	sheet.add_cell(Cell::str("2", ""), 11, 5);
	sheet.add_cell(Cell::str("dort bude", ""), 11, 6);

	// Add the task
	sheet.add_cell(Cell::str("Zadání", ""), 14, 0);
	sheet.add_cell(Cell::str("Osm závodníků ve skoku do dálky dosáhlo ve třech pokusech výsledky podle obrázku výše. Určete ve sloupci E nejdelší skok závodníka, ve sloupci F pořadí závodníka a ve sloupci G zda dostane nebo nedostane dort (jen první tři). Řádkům se závodníky, kteří se umístili na prvních třech místech přidělte barevní pozadí (první tři místa barevně odlišená).", ""), 15, 0);

	// Add sheet to the book
	book.add_sheet(sheet);

	ods::write(&book, Path::new("assignment.ods"));
}