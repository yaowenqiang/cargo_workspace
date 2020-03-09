use mysql::*;
use mysql::prelude::*;
#[derive[Debug, PartialEq, Eq]]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    let url = "mysql://root:password@localhost:3306/db_name";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"CREATE temporary table payment (
            customer_id int not null,
            amount int not null,
            account_name test
        )")?;

        let payments = vec![
            Payment{customer_id: 1, amount: 2, account_name:None}
        ];

        conn.exec_batch(
            r"insert into payment (customer_id, amount, account_name)
            values(:customer_id, :amount, :account_name)",
            Payments.iter().map(|p| params | {
                "customer_id" => p.customer_id,
                "amount" =>p.amount,
                "account_name" => &p.account_name,
            }),
        );

        let selected_payments = conn.
            query_map(
                "select customer_id, amount, account_name from payment",
                |(customer_id, amount, account_name)| {
                    Payment{customer_id, amount, account_name}
                }
                );
        assert_eq(payments, selected_payments);
        println!("Yay!");
}
