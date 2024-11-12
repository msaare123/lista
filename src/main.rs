
#[derive(Debug)]
enum Error {
    NotLongEnough,
    InvalidInput,
}

/// Struct which represents one fixed length molding
#[derive(Debug, PartialEq, PartialOrd)]
struct Molding {
    /// Length of the full molding
    length: u16,
    /// Pieces cut from this particular molding
    pieces: Vec<u16>,
}

impl Molding {
    /// Returns remaining usable length of molding
    fn length_remaining(&self) -> u16 {
        let mut length = 0;
        self.pieces.iter().for_each(|p| length += p);
        self.length - length
    }

    /// Try to cut piece from molding
    fn add_piece(&mut self, length: u16) -> Result<(), Error> {
        if self.length_remaining() < length {
            Err(Error::NotLongEnough)
        } else {
            self.pieces.push(length);
            Ok(())
        }
    }
}

#[derive(Debug)]
struct MoldingSet {
    /// Length of the fixed starting piece from which parts are cut from
    fixed_piece: u16,
    /// Set of Moldings
    set: Vec<Molding>,
}

impl MoldingSet {
    /// Create new molding set
    fn new(fixed_piece: u16) -> Self {
        Self {
            fixed_piece,
            set: Vec::new(),
        }
    }

    /// Add one full sized molding to set
    fn add_full(&mut self) {
        self.set.push(Molding {
            length: self.fixed_piece,
            pieces: vec![self.fixed_piece],
        })
    }

    /// Add part that is known to be equal or less than fixed size molding
    fn add_partial(&mut self, length: u16) -> Result<(), Error> {
        if length > self.fixed_piece {
            return Err(Error::InvalidInput);
        }

        // Find room from existing parts
        let suitable = self
            .set
            .iter_mut()
            .filter(|x| x.length_remaining() >= length)
            .reduce(|a, b| if a < b { a } else { b });
        if let Some(suitable) = suitable {
            // Found some suitable
            suitable.add_piece(length)?;
        } else {
            // Need to add new
            self.set.push(Molding {
                length: self.fixed_piece,
                pieces: vec![length],
            })
        }

        Ok(())
    }

    fn add(&mut self, length: u16) -> Result<(), Error> {
        let mut len = length;
        while len > 0 {
            if len >= self.fixed_piece {
                self.add_full();
                len -= self.fixed_piece;
            } else {
                self.add_partial(len)?;
                len = 0;
            }
        }

        Ok(())
    }
}

fn main() {
    const FIXED_PIECE: u16 = 2200;
    let mut desired_pieces: Vec<u16> = vec![2110, 2110, 2110, 2110, 940, 850];
    let mut molding_pieces = MoldingSet::new(FIXED_PIECE);

    desired_pieces.sort_by(|a, b| (b).cmp(a));
    for piece in desired_pieces {
        molding_pieces.add(piece).unwrap();
    }

    println!("Number of fixed length pieces: {}", molding_pieces.set.len());
    for (i, piece) in molding_pieces.set.iter().enumerate() {
        println!("Piece: {}, {:?}", i + 1, piece);
    }
}