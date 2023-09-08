use mysql::prelude::*;
use mysql::*;
use std::env;

fn main() {
    let url = "mysql://root:root@192.168.101.8:3306/seguros";
    let pool = Pool::new(url).unwrap();
    let args: Vec<String> = env::args().collect();
    let where_param = &args[1];

    println!("Searching: {}", where_param);

    let mut conn = pool.get_conn().unwrap();

    let select_formulas = conn
        .query_iter("SELECT formula FROM seguros.plantilla_ramo_cobertura_tarifa WHERE idramo = 3")
        .get();

    //let filter = select_formulas.filter(|f| f.contains(where_param));
    println!("{:?}", select_formulas);

    //    .for_each(|row| {
    //        let r: String = from_row(row.unwrap());
    //        println!("{}", r);
    //    });
}
