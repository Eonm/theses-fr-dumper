pub fn sequence(
    start: u32,
    step: u32,
    end: u32,
) -> Vec<(u32, u32)> {
    let mut seq = vec![];
    let distance = end - start;
    let number_of_step = distance / step;

    for i  in 0..number_of_step {
        seq.push(( i * step + start, step));
    }

    if (end - start) % step != 0 {
        seq.push((seq.last().unwrap_or(&(0, end)).0 + step, (end - start) % step));
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let seq = sequence(0, 2, 6);
        assert_eq!(seq, vec![(0,2), (2,2), (4,2)]);

        let seq = sequence(0, 3, 15);
        assert_eq!(seq, vec![(0,3), (3,3), (6,3), (9,3), (12,3)]);

        let seq = sequence(0, 2, 7);
        assert_eq!(seq, vec![(0,2), (2,2), (4,2), (6,1)]);

        let seq = sequence(2, 2, 6);
        assert_eq!(seq, vec![(2,2), (4,2)]);

        let seq = sequence(2, 2, 7);
        assert_eq!(seq, vec![(2,2), (4,2), (6,1)]);

        let seq = sequence(2, 3, 7);
        assert_eq!(seq, vec![(2,3), (5,2)]);
    }
}