use bytestore::components::map::hashing::Hash;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::mem::MaybeUninit;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ngram<const N: usize>([char; N]);

impl<const N: usize> TryFrom<&String> for Ngram<N> {
    type Error = ();

    #[inline]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let value = value.as_str();
        Self::try_from(value)
    }
}

impl<const N: usize> TryFrom<String> for Ngram<N> {
    type Error = ();

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.as_str();
        Self::try_from(value)
    }
}

impl<const N: usize> TryFrom<&str> for Ngram<N> {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let n: [char; N] = value
            .chars()
            .take(N)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ())?;
        Ok(Self(n))
    }
}

impl<const N: usize> Serialize for Ngram<N> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple(N)?;
        for item in self.0.iter() {
            s.serialize_element(&item)?;
        }
        s.end()
    }
}

impl<'de, const N: usize> Deserialize<'de> for Ngram<N> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let res = deserializer.deserialize_tuple(N, CharArrayVisitor::<N>)?;
        Ok(Self(res))
    }
}

struct CharArrayVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for CharArrayVisitor<N> {
    type Value = [char; N];

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("char array of length {}", N))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut out: [MaybeUninit<char>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for item in out.iter_mut() {
            match seq.next_element::<char>()? {
                Some(v) => {
                    item.write(v);
                }
                None => {
                    return Err(serde::de::Error::invalid_length(N, &self));
                }
            }
        }

        let out: [char; N] = unsafe { *out.as_ptr().cast() };
        Ok(out)
    }
}

impl<const N: usize> Hash for Ngram<N> {
    #[inline]
    fn hash(&self) -> u64 {
        self.0.hash()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ngram_deser() {
        let data = ['あ', 'b', '漢'];
        let ngram = Ngram(data);
        let serialized = serde_json::to_string(&ngram).unwrap();
        assert_eq!(serialized, r#"["あ","b","漢"]"#);
        let deserialized: Ngram<3> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ngram);
    }
}
