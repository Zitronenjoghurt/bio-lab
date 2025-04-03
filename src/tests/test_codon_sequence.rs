use crate::lab::types::codon_sequence::CodonSequence;

#[test]
fn test_byte_parsing() {
    for length in 0..100 {
        let sequence = CodonSequence::random(length);
        let bytes = sequence.to_bytes();
        let initial_sequence = CodonSequence::from_bytes(&bytes);
        assert_eq!(sequence, initial_sequence);
    }
}
