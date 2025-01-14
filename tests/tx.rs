extern crate rex;
use rex::db::*;
use rusqlite::{Connection, Result as sqlResult};
use std::fs;
//use std::collections::HashMap;

fn create_test_db(file_name: &str) -> Connection {
    create_db(file_name, vec!["test1".to_string(), "test 2".to_string()]).unwrap();
    Connection::open(file_name).unwrap()
}

#[test]
fn check_last_tx_id_1() {
    let file_name = "last_tx_id_1.sqlite".to_string();
    let conn = create_test_db(&file_name);

    let data = get_last_tx_id(&conn);
    let expected_data: sqlResult<i32> = Err(rusqlite::Error::QueryReturnedNoRows);

    conn.close().unwrap();
    fs::remove_file(file_name).unwrap();

    assert_eq!(data, expected_data);
}

#[test]
fn check_last_tx_id_2() {
    let file_name = "last_tx_id_2.sqlite".to_string();
    let conn = create_test_db(&file_name);

    add_new_tx(
        "2022-09-19",
        "Testing transaction",
        "test1",
        "100.00",
        "Income",
        &file_name,
        None,
    )
    .unwrap();

    let data = get_last_tx_id(&conn);
    let expected_data: sqlResult<i32> = Ok(1);

    conn.close().unwrap();
    fs::remove_file(file_name).unwrap();

    assert_eq!(data, expected_data);
}

#[test]
fn check_getting_all_tx_1() {
    let file_name = "getting_tx_1.sqlite".to_string();
    let conn = create_test_db(&file_name);

    let data = get_all_txs(&conn, 6, 0);
    let expected_data = (Vec::new(), Vec::new(), Vec::new());

    conn.close().unwrap();
    fs::remove_file(file_name).unwrap();

    assert_eq!(data, expected_data);
}

#[test]
fn check_getting_all_tx_2() {
    let file_name = "getting_tx_2.sqlite".to_string();
    let conn = create_test_db(&file_name);

    add_new_tx(
        "2022-07-19",
        "Testing transaction",
        "test1",
        "100.00",
        "Expense",
        &file_name,
        None,
    )
    .unwrap();

    add_new_tx(
        "2022-07-19",
        "Testing transaction",
        "test 2",
        "100.00",
        "Expense",
        &file_name,
        None,
    )
    .unwrap();

    add_new_tx(
        "2022-05-15",
        "Testing transaction",
        "test 2",
        "100.00",
        "Expense",
        &file_name,
        None,
    )
    .unwrap();

    add_new_tx(
        "2022-05-20",
        "Testing transaction",
        "test 2",
        "100.00",
        "Income",
        &file_name,
        None,
    )
    .unwrap();

    let data = get_all_txs(&conn, 6, 0);
    let data_2 = get_all_txs(&conn, 4, 0);

    let expected_data = (
        vec![
            vec![
                "19-07-2022".to_string(),
                "Testing transaction".to_string(),
                "test1".to_string(),
                "100.00".to_string(),
                "Expense".to_string(),
            ],
            vec![
                "19-07-2022".to_string(),
                "Testing transaction".to_string(),
                "test 2".to_string(),
                "100.00".to_string(),
                "Expense".to_string(),
            ],
        ],
        vec![
            vec!["-100.00".to_string(), "0.00".to_string()],
            vec!["-100.00".to_string(), "-100.00".to_string()],
        ],
        vec!["1".to_string(), "2".to_string()],
    );

    let expected_data_2 = (
        vec![
            vec![
                "15-05-2022".to_string(),
                "Testing transaction".to_string(),
                "test 2".to_string(),
                "100.00".to_string(),
                "Expense".to_string(),
            ],
            vec![
                "20-05-2022".to_string(),
                "Testing transaction".to_string(),
                "test 2".to_string(),
                "100.00".to_string(),
                "Income".to_string(),
            ],
        ],
        vec![
            vec!["0.00".to_string(), "-100.00".to_string()],
            vec!["0.00".to_string(), "0.00".to_string()],
        ],
        vec!["3".to_string(), "4".to_string()],
    );

    conn.close().unwrap();
    fs::remove_file(file_name).unwrap();

    assert_eq!(data, expected_data);
    assert_eq!(data_2, expected_data_2);
}
