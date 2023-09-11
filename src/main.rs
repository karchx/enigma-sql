use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;

struct MysqlConnection {
    user: Option<String>,
    pass: Option<String>,
    ip_or_hostname: Option<String>,
    tcp_port: u16,
    db_name: Option<String>,
}

impl MysqlConnection {
    fn new() {
        OptsBuilder::from(MysqlConnection::config_db())
    }

    fn config_db() -> Self {
        let mysql_user = read_vars("MYSQL_USER");
        let mysql_pass = read_vars("MYSQL_PASS");
        let mysql_port: u16 = read_vars("MYSQL_PORT")
            .parse()
            .expect("please give me correct string number");
        let mysql_host = read_vars("MYSQL_HOST");
        let mysql_db = read_vars("MYSQL_DB");

        return MysqlConnection {
            user: Some(&mysql_user),
            pass: Some(&mysql_pass),
            ip_or_hostname: Some(&mysql_port),
            tcp_port: mysql_port,
            db_name: Some(&mysql_db),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ResultQuery {
    idcobertura: Option<String>,
    formula: Option<String>,
    elemento: Option<String>,
}

#[derive(Eq, PartialEq, Debug)]
struct UpdateQuery {
    idcobertura: String,
    formula: String,
    elemento: String,
}

impl UpdateQuery {
    fn new(idcobertura: &str, formula: &str, elemento: &str) -> UpdateQuery {
        UpdateQuery {
            idcobertura: idcobertura.to_string(),
            formula: formula.to_string(),
            elemento: elemento.to_string(),
        }
    }
}

fn main() {
    dotenv().ok();

    /*let opts = OptsBuilder::new()
    .user(Some(&mysql_user))
    .pass(Some(&mysql_pass))
    .ip_or_hostname(Some(&mysql_host))
    .tcp_port(mysql_port)
    .db_name(Some(&mysql_db));*/
    let opts = MysqlConnection::new();
    let pool = Pool::new(opts).unwrap();
    let args: Vec<String> = env::args().collect();
    let where_param = &args[1];

    let mut conn = pool.get_conn().unwrap();
    let mut update_formulas = Vec::new();

    let all_formulas: Vec<ResultQuery> = conn
        .query_iter("SELECT idcobertura, formula, elemento FROM seguros.plantilla_ramo_cobertura_tarifa WHERE idramo = 3")
        .map(|row| {
            row.map(|r| r.unwrap())
                .map(|r| {
                    let (idcobertura, formula, elemento) = mysql::from_row(r);

                    ResultQuery { idcobertura, formula, elemento }
                })
                .collect()
        })
        .unwrap();

    for result in all_formulas.iter() {
        match (&result.idcobertura, &result.formula, &result.elemento) {
            (Some(idcobertura), Some(formula), Some(elemento)) => {
                let formula_underscore = formula.replace(" ", "_");
                if formula_underscore.contains(where_param) {
                    let where_without_underscore = where_param.replace("_", " ");
                    let replace_formula = formula.replace(&where_without_underscore, "TASA");
                    update_formulas.push(UpdateQuery::new(idcobertura, &replace_formula, elemento));
                }
            }
            _ => {}
        }
    }

    for item in &update_formulas {
        println!("{}", item.formula);
    }
}

fn read_vars(var: &str) -> String {
    let error_expect = format!("{} must be set.", var);

    std::env::var(var).expect(&error_expect)
}
