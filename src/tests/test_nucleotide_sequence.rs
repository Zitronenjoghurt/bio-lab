use crate::lab::types::nucleotide_sequence::NucleotideSequence;

#[test]
fn test_byte_parsing() {
    for length in 0..100 {
        let sequence = NucleotideSequence::random(length);
        let bytes = sequence.to_bytes();
        let initial_sequence = NucleotideSequence::from_bytes(&bytes);
        assert_eq!(sequence, initial_sequence);
    }
}
