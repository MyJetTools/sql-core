use rust_extensions::slice_of_u8_utils::*;

pub fn scan_sql_for_placeholders<'s>(sql: &'s str) -> Vec<SqlTransformToken<'s>> {
    let mut pos_from = 0usize;

    let as_bytes = sql.as_bytes();

    let mut tokens = Vec::new();

    while let Some(place_holder_start_position) =
        as_bytes.find_sequence_pos("${".as_bytes(), pos_from)
    {
        let content =
            std::str::from_utf8(&as_bytes[pos_from..place_holder_start_position]).unwrap();

        tokens.push(SqlTransformToken::RawContent(content));

        let place_holder_end_position =
            as_bytes.find_sequence_pos("}".as_bytes(), place_holder_start_position);

        if place_holder_end_position.is_none() {
            break;
        }

        let place_holder_end_position = place_holder_end_position.unwrap();

        let field_name = std::str::from_utf8(
            &as_bytes[place_holder_start_position + 2..place_holder_end_position],
        )
        .unwrap();

        tokens.push(SqlTransformToken::PlaceHolder(field_name));

        pos_from = place_holder_end_position + 1;
    }

    if pos_from < sql.len() {
        let content = std::str::from_utf8(&as_bytes[pos_from..sql.len()]).unwrap();

        tokens.push(SqlTransformToken::RawContent(content))
    }

    tokens
}

#[derive(Debug)]
pub enum SqlTransformToken<'s> {
    RawContent(&'s str),
    PlaceHolder(&'s str),
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_from_prod_code() {
        let sql = "moment >= ${from_date} AND moment <= ${to_date} AND (message LIKE '%${phrase}%' OR context LIKE '%${phrase}%')";

        let scan_sql_for_placeholders = super::scan_sql_for_placeholders(sql);

        println!("{:#?}", scan_sql_for_placeholders);
    }
}
