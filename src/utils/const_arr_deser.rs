use std::{convert::TryInto, fmt::Debug, marker::PhantomData};

use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};

pub fn serialize<S: Serializer, T: Serialize + Clone, const N: usize>(
    data: &Vec<[T; N]>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    let mut s = ser.serialize_seq(Some(data.len() * N))?;
    for d in data.iter().flat_map(|i| i.iter()) {
        s.serialize_element(d)?;
    }
    s.end()
}

struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
where
    T: Deserialize<'de> + Debug,
{
    type Value = Vec<[T; N]>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an array of length {}", N))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let len = seq.size_hint().unwrap_or(10);
        let mut out = Vec::with_capacity(len);

        let next = seq.next_element::<Vec<T>>()?;

        if let Some(list) = next {
            let mut buf = Vec::with_capacity(N);
            for i in list {
                buf.push(i);
                if buf.len() == N {
                    let arr: [T; N] = std::mem::take(&mut buf).try_into().unwrap();
                    out.push(arr);
                }
            }
        }

        Ok(out)
    }
}

#[inline]
pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<Vec<[T; N]>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Debug,
{
    deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
}
