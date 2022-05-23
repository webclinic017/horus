use postgres::types::ToSql;

pub trait Strategy {
    fn get_name(&self) -> &'static str;
    fn get_params<'a>(&'a self) -> Vec<(String, &'a (dyn ToSql + Sync))>;
}

// pub trait ParameterizedStrategy {

// }

pub struct GoldenCrossStrategy {
    pub first_sma: u32,
    pub second_sma: u32,
}

impl Strategy for GoldenCrossStrategy {
    fn get_name(&self) -> &'static str {
        return "GoldenCross";
    }

    fn get_params<'a>(&'a self) -> Vec<(String, &'a (dyn ToSql + Sync))> {
        let params: Vec<(String, &'a (dyn ToSql + Sync))> = vec!((String::from("first_sma"), &self.first_sma), (String::from("second_sma"), &self.second_sma));
        params
    }
}
