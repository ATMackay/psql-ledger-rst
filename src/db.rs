use crate::{
    errors::MyError,
    model::{Account, Transaction},
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn ping_db(client: &Client) -> Result<(), MyError> {
    let _ = client.query_one("SELECT NOW()", &[]).await?;

    Ok(())
}

pub async fn get_accounts(client: &Client) -> Result<Vec<Account>, MyError> {
    let stmt = "SELECT * FROM accounts ORDER BY id";
    let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>();

    Ok(results)
}

pub async fn get_account_by_id(client: &Client, account_id: i64) -> Result<Account, MyError> {
    let stmt = "SELECT * FROM accounts WHERE id = $1 LIMIT 1";
    let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    client
        .query(&stmt, &[&account_id])
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_transaction_by_id(
    client: &Client,
    account_id: i64,
) -> Result<Transaction, MyError> {
    let stmt = "SELECT * FROM transactions WHERE id = $1 LIMIT 1";
    let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    client
        .query(&stmt, &[&account_id])
        .await?
        .iter()
        .map(|row| Transaction::from_row_ref(row).unwrap())
        .collect::<Vec<Transaction>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn create_account(client: &Client, account_info: Account) -> Result<Account, MyError> {
    let _stmt = "INSERT INTO accounts (
            username, balance, email
        ) VALUES (
            $1, $2, $3
        )
        RETURNING *";
    let _stmt = _stmt.replace("$table_fields", &Account::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &account_info.username,
                &account_info.balance,
                &account_info.email,
            ],
        )
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_transactions(client: &Client) -> Result<Vec<Transaction>, MyError> {
    let stmt = "SELECT * FROM transactions ORDER BY id";
    let stmt = stmt.replace("$table_fields", &Transaction::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Transaction::from_row_ref(row).unwrap())
        .collect::<Vec<Transaction>>();

    Ok(results)
}

pub async fn create_transaction(
    client: &Client,
    transaction_info: Transaction,
) -> Result<Transaction, MyError> {
    let _stmt = "INSERT INTO transactions (
            from_account, to_account, amount
        ) VALUES (
            $1, $2, $3
        )
        RETURNING *";
    let _stmt = _stmt.replace("$table_fields", &Transaction::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &transaction_info.from_account,
                &transaction_info.to_account,
                &transaction_info.amount,
            ],
        )
        .await?
        .iter()
        .map(|row| Transaction::from_row_ref(row).unwrap())
        .collect::<Vec<Transaction>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}
