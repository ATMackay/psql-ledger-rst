--! account_by_id
SELECT * FROM accounts
WHERE id = $1 LIMIT 1;

--! accounts
SELECT * FROM accounts
ORDER BY id;

--! account_by_username
SELECT * FROM accounts
WHERE username = $1 LIMIT 1;

--! account_by_email
SELECT * FROM accounts
WHERE email = $1 LIMIT 1;

--! transaction_by_id
SELECT * FROM transactions
WHERE id = $1 LIMIT 1;

--! delete_account
DELETE FROM accounts
WHERE id = $1;

--! new_account
INSERT INTO accounts (
	username, balance, email
) VALUES (
	$1, $2, $3
)
RETURNING *;

--! new_transaction
INSERT INTO transactions (
	from_account, to_account, amount
) VALUES (
	$1, $2, $3
)
RETURNING *;

--! account_transactions
SELECT
    t.id AS transaction_id,
    t.from_account AS from_account_id,
    from_acc.username AS from_username,
    t.to_account AS to_account_id,
    to_acc.username AS to_username,
    t.amount,
    t.created_at AS transaction_created_at
FROM
    transactions t
JOIN
    accounts from_acc ON t.from_account = from_acc.id
JOIN
    accounts to_acc ON t.to_account = to_acc.id
WHERE
    from_acc.username = 'desired_username' OR to_acc.username = 'desired_username'
ORDER BY
    t.created_at DESC;
