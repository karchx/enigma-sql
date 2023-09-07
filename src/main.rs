use mysql::*;
use mysql::prelude::*;

fn main() {
    let url = "mysql://root:root@192.168.101.8:3306/seguros";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn
        .query_iter("SELECT idcotizacion FROM seguros.cotizacion LIMIT 10")
        .unwrap()
        .for_each(|row| {
            let r: i32 = from_row(row.unwrap());
            println!("{}", r);
        });
}
