mod transactions {

    use mysql::prelude::FromRow;
    use mysql::*;
    pub struct TransactionQuerys<'a> {
        transaction: Transaction<'a>,
    }

    impl<'a> TransactionQuerys<'a> {
        pub fn new(pool: &Pool) -> Result<TransactionQuerys<'a>, mysql::Error> {
            let tr = pool.start_transaction(TxOpts::default())?;

            Ok(TransactionQuerys { transaction: tr })
        }

        pub fn use_transaction<T>(&mut self, data: &str) -> Result<QueryResult<T>, mysql::Error>
        where
            T: mysql::prelude::Protocol,
        {
            self.transaction.prep(data)?.execute(())
        }

        pub fn commit(&self) -> Result<(), mysql::Error> {
            self.transaction.commit()
        }
    }
}
