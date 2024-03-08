use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::Params;
use rusqlite::ToSql;

use super::BDEResult;

pub fn get_database_connection() -> BDEResult<Connection> {
    let database_path = init_database()?;
    Ok(Connection::open(database_path)?)
}

fn init_database() -> BDEResult<PathBuf> {
    let data_path = PathBuf::from("./data");

    if !data_path.exists() {
        fs::create_dir(data_path.clone());
    }

    let database_path = data_path.join("data.db");

    if !database_path.exists() {
        let conn = Connection::open(&database_path)?;

        let create_table_sql = fs::read_to_string("./sql/create_table.sql")?;
        let tables_sql = create_table_sql.split(';');
        for table_sql in tables_sql {
            if !table_sql.trim().is_empty() {
                conn.execute(table_sql, ())?;
            }
        }
    }

    Ok(database_path)
}

pub fn generate_insert_sql(table_name: &str, args_n: u16) -> String {
    let args_str_vec: Vec<&str> = vec!["?"; args_n.into()];
    format!(
        "INSERT INTO {} VALUES ({})",
        table_name,
        args_str_vec.join(", ")
    )
}

pub fn database_insert<T: Params>(
    table_name: &str,
    keywords: Vec<&str>,
    params: T,
) -> BDEResult<u64> {
    let database_path = init_database()?;
    let conn = Connection::open(database_path)?;

    let args_str_vec: Vec<&str> = vec!["?"; keywords.len()];
    let sql_command = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        keywords.join(", "),
        args_str_vec.join(", ")
    );

    conn.execute(sql_command.as_str(), params)?;

    let mut stmt = conn.prepare(
        format!(
            "select seq from sqlite_sequence where name='{}';",
            table_name
        )
        .as_str(),
    )?;

    let mut data_iter = serde_rusqlite::from_rows::<u64>(stmt.query([])?);

    let id = data_iter.next().unwrap()?;

    Ok(id)
}

pub fn database_insert_no_id<T: Params>(
    table_name: &str,
    keywords: Vec<&str>,
    params: T,
) -> BDEResult<()> {
    let database_path = init_database()?;
    let conn = Connection::open(database_path)?;

    let args_str_vec: Vec<&str> = vec!["?"; keywords.len()];
    let sql_command = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        keywords.join(", "),
        args_str_vec.join(", ")
    );

    conn.execute(sql_command.as_str(), params)?;

    Ok(())
}

pub fn database_select<T: serde::de::DeserializeOwned>(
    table_name: &str,
    where_args: Option<String>,
) -> BDEResult<Vec<T>> {
    let mut all_data: Vec<T> = Vec::new();
    let database_path = init_database()?;
    let conn = Connection::open(database_path)?;

    let sql_command = format!(
        "SELECT * FROM {} {}",
        table_name,
        match where_args {
            Some(where_args) => {
                format!("WHERE {}", where_args)
            }
            None => {
                String::default()
            }
        }
    );

    let mut stmt = conn.prepare(sql_command.as_str())?;

    let data_iter = serde_rusqlite::from_rows::<T>(stmt.query([])?);

    for data in data_iter {
        all_data.push(data?);
    }

    Ok(all_data)
}

pub fn database_select_single_name<T: serde::de::DeserializeOwned>(
    table_name: &str,
    item_id: u64,
    primary_key_name: &str,
) -> BDEResult<Option<T>> {
    let where_args = format!("{} == {}", primary_key_name, item_id);
    let data_iter = database_select::<T>(table_name, Some(where_args))?;

    Ok(data_iter.into_iter().next())
}

pub fn database_select_single<T: serde::de::DeserializeOwned>(
    table_name: &str,
    item_id: u64,
) -> BDEResult<Option<T>> {
    let res = database_select_single_name(table_name, item_id, "id")?;
    Ok(res)
}

pub fn database_update<T: Params>(
    table_name: &str,
    set_keywords: Vec<&str>,
    set_params: T,
    where_args: String,
) -> BDEResult<()> {
    let database_path = init_database()?;
    let conn = Connection::open(database_path)?;

    let keywords = set_keywords
        .into_iter()
        .map(|keyword| format!("{} = ?", keyword))
        .collect::<Vec<String>>()
        .join(", ");

    let sql_command = format!(
        "UPDATE {} SET {} = ?1 WHERE {}",
        table_name, keywords, where_args
    );

    conn.execute(sql_command.as_str(), set_params)?;

    Ok(())
}

pub fn database_update_single_set_where<P: ToSql>(
    table_name: &str,
    keyword: &str,
    item_id: u64,
    data: P,
) -> BDEResult<()> {
    let database_path = init_database()?;
    let conn = Connection::open(database_path)?;

    let sql_command = format!("UPDATE {} SET {} = ?1 WHERE id = ?2", table_name, keyword);

    conn.execute(sql_command.as_str(), (data, item_id))?;

    Ok(())
}

pub fn database_delete(table_name: &str, where_args: String) -> BDEResult<()> {
    let database_path = init_database()?;
    let conn = Connection::open(database_path)?;

    let sql_command = format!("DELETE FROM {} WHERE {}", table_name, where_args);

    conn.execute(sql_command.as_str(), ())?;

    Ok(())
}
