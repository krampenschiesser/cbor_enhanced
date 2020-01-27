impl<'de> crate::Deserialize<'de> for BlaEnum {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        let mut t_2: Option<usize> = Some(usize::default());
        let mut t_3: Option<usize> = Some(usize::default());
        let mut t_4: Option<BlaStruct> = None;
        let mut t_5: Option<BlaStruct> = None;
        let mut t_6: Option<usize> = Some(usize::default());
        let mut t_7: Option<BlaStruct> = None;
        let mut t_8: Option<String> = Some(String::default());
        let mut t_9: Option<usize> = Some(usize::default());
        let mut t_10: Option<BlaStruct> = None;
        let mut t_11: Option<String> = Some(String::default());
        let mut t_12: Option<Option<i32>> = None;
        let mut found_ids: Vec<usize> = Vec::new();
        let (map_def, data) = deserializer.take_map_def(data, true)?;
        let map_length = map_def.unwrap_or(0);
        let mut data = data;
        for i in 0..map_length {
            let (key, rem) = deserializer.take_unsigned(data, true)?;
            data = rem;
            match key {
                2u64 => {
                    let (val, rem) = usize::deserialize(deserializer, data)?;
                    data = rem;
                    t_2 = Some(val.into());
                    found_ids.push(2u64 as usize);
                }
                3u64 => {
                    let (val, rem) = usize::deserialize(deserializer, data)?;
                    data = rem;
                    t_3 = Some(val.into());
                    found_ids.push(3u64 as usize);
                }
                4u64 => {
                    let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
                    data = rem;
                    t_4 = Some(val.into());
                    found_ids.push(4u64 as usize);
                }
                5u64 => {
                    let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
                    data = rem;
                    t_5 = Some(val.into());
                    found_ids.push(5u64 as usize);
                }
                6u64 => {
                    let (val, rem) = usize::deserialize(deserializer, data)?;
                    data = rem;
                    t_6 = Some(val.into());
                    found_ids.push(6u64 as usize);
                }
                7u64 => {
                    let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
                    data = rem;
                    t_7 = Some(val.into());
                    found_ids.push(7u64 as usize);
                }
                8u64 => {
                    let (val, rem) = String::deserialize(deserializer, data)?;
                    data = rem;
                    t_8 = Some(val.into());
                    found_ids.push(8u64 as usize);
                }
                9u64 => {
                    let (val, rem) = usize::deserialize(deserializer, data)?;
                    data = rem;
                    t_9 = Some(val.into());
                    found_ids.push(9u64 as usize);
                }
                10u64 => {
                    let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
                    data = rem;
                    t_10 = Some(val.into());
                    found_ids.push(10u64 as usize);
                }
                11u64 => {
                    let (val, rem) = String::deserialize(deserializer, data)?;
                    data = rem;
                    t_11 = Some(val.into());
                    found_ids.push(11u64 as usize);
                }
                12u64 => {
                    let (val, rem) = Option::deserialize(deserializer, data)?;
                    data = rem;
                    t_12 = Some(val.into());
                    found_ids.push(12u64 as usize);
                }
                _ => {}
            }
        }
        let retval = if deserializer.found_contains_any(&found_ids, &[2u64]) {
            deserializer.check_is_some(&t_2, "BlaEnum::Val(0)")?;
            BlaEnum::Val(t_2.unwrap())
        } else if deserializer.found_contains_any(&found_ids, &[5u64]) {
            deserializer.check_is_some(&t_5, "BlaEnum::ValStructNamed{my}")?;
            BlaEnum::ValStructNamed { my: t_5.unwrap() }
        } else if deserializer.found_contains_any(&found_ids, &[12u64]) {
            deserializer.check_is_some(&t_12, "BlaEnum::ValOption(0)")?;
            BlaEnum::ValOption(t_12.unwrap())
        } else if deserializer.found_contains_any(&found_ids, &[9u64, 10u64, 11u64]) {
            deserializer.check_is_some(&t_9, "BlaEnum::ValMultipleName{id}")?;
            deserializer.check_is_some(&t_10, "BlaEnum::ValMultipleName{bla}")?;
            deserializer.check_is_some(&t_11, "BlaEnum::ValMultipleName{name}")?;
            BlaEnum::ValMultipleName {
                id: t_9.unwrap(),
                bla: t_10.unwrap(),
                name: t_11.unwrap(),
            }
        } else if deserializer.found_contains_any(&found_ids, &[4u64]) {
            deserializer.check_is_some(&t_4, "BlaEnum::ValStruct(0)")?;
            BlaEnum::ValStruct(t_4.unwrap())
        } else if deserializer.found_contains_any(&found_ids, &[6u64, 7u64, 8u64]) {
            deserializer.check_is_some(&t_6, "BlaEnum::ValMultipleTuple(0)")?;
            deserializer.check_is_some(&t_7, "BlaEnum::ValMultipleTuple(1)")?;
            deserializer.check_is_some(&t_8, "BlaEnum::ValMultipleTuple(2)")?;
            BlaEnum::ValMultipleTuple(t_6.unwrap(), t_7.unwrap(), t_8.unwrap())
        } else if deserializer.found_contains_any(&found_ids, &[3u64]) {
            deserializer.check_is_some(&t_3, "BlaEnum::ValNamed{name}")?;
            BlaEnum::ValNamed { name: t_3.unwrap() }
        } else {
            return Err(CborError::NoValueFound("Any variant of BlaEnum"));
        };
        Ok((retval, data))
    }
}
