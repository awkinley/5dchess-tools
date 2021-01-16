use super::*;

/** A wrapper around GenMove to cache the moves generated by an iterator.
    You should consider using this if you wish to re-use a `GenMoves`-iterator.
**/
pub struct CacheMoves<'a, B: Clone + AsRef<Board> + 'a, G: GenMoves<'a, B>> {
    pub iterator: G::Iter,
    pub cache: Vec<Move>,
}

impl<'a, B: Clone + AsRef<Board> + 'a, G: GenMoves<'a, B>> CacheMoves<'a, B, G> {
    /**
        Creates a new CacheMoves iterator out of a `GenMoves`-implementor and the required information for `generate_moves`.
    **/
    pub fn new(generator: G, game: &'a Game, partial_game: &'a PartialGame<'a, B>) -> Option<Self> {
        Some(Self {
            iterator: generator.generate_moves(game, partial_game)?,
            cache: vec![],
        })
    }

    /**
        Validates a move by looking through the cached moves. This function will not query
        the move iterator.

        Since not every move is looked through, if this function returns false,
        then it doesn't mean that the move is invalid. For a more thorough but expensive move
        validation function, you should use `validate_move`.
    **/
    pub fn validate_move_cached(&self, mv: &Move) -> bool {
        for cached_mv in self.cache.iter() {
            if cached_mv == mv {
                return true;
            }
        }

        false
    }

    /**
        Validaes a move by first traversing the cached moves and then consuming the iterator
        until the move is found. You should prefer using `G::validate_move` over this function
        for its potential speed benefit, unless you want to validate many moves from the same `CacheMoves` instance.
    **/
    pub fn validate_move(&mut self, mv: &Move) -> bool {
        if self.validate_move_cached(mv) {
            return true;
        }

        while let Some(m) = self.next() {
            if m == *mv {
                return true;
            }
        }

        false
    }

    /**
        Returns the n-th move from the cache. This function will not query the move iterator.
        You should use `get` if you also wish to query the move iterator.

        Returns `None` if the move isn't in the cache.
    **/
    pub fn get_cached(&mut self, n: usize) -> Option<Move> {
        if n < self.cache.len() {
            Some(self.cache[n])
        } else {
            None
        }
    }

    /**
        Returns the n-th move of the iterator. If that move already lies in the cache, then it is queried from the cache. Otherwise, the iterator is consumed up to the n-th move, if found.

        Returns `None` if the move is neither in the cache, nor could the iterator yield enough moves.
    **/
    pub fn get(&mut self, n: usize) -> Option<Move> {
        if n < self.cache.len() {
            Some(self.cache[n])
        } else {
            while let Some(m) = self.next() {
                if self.cache.len() == n + 1 {
                    return Some(m);
                }
            }

            None
        }
    }
}

impl<'a, B: Clone + AsRef<Board> + 'a, G: GenMoves<'a, B>> Iterator for CacheMoves<'a, B, G> {
    type Item = Move;

    /**
        Yields the next move, if present, and caches it.
    **/
    fn next(&mut self) -> Option<Move> {
        match self.iterator.next() {
            Some(m) => {
                self.cache.push(m);
                Some(m)
            }
            None => None,
        }
    }
}
