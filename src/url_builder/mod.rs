pub fn csv_url_builder(step: &(u32, u32)) -> String {
    format!("https://www.theses.fr/?q=*:*&lng=fr&start={}&status=&access=&prevision=&filtrepersonne=&lng=&lng=&checkedfacets=&format=csv&type=avancee&rows={}", step.0, step.1 )
}

pub fn json_url_builder(step: &(u32, u32)) -> String {
    format!("https://www.theses.fr/?q=*:*&lng=fr&start={}&status=&access=&prevision=&filtrepersonne=&lng=&lng=&checkedfacets=&format=json&type=avancee&rows={}", step.0, step.1 )
}

mod tests {
    use super::*;

    #[test]
    fn test_csv_url_builder() {
        assert_eq!(csv_url_builder(&(0,2)), "https://www.theses.fr/?q=*:*&lng=fr&start=0&status=&access=&prevision=&filtrepersonne=&lng=&lng=&checkedfacets=&format=csv&type=avancee&rows=2")
    }

    #[test]
    fn test_json_url_builder() {
        assert_eq!(json_url_builder(&(0,2)), "https://www.theses.fr/?q=*:*&lng=fr&start=0&status=&access=&prevision=&filtrepersonne=&lng=&lng=&checkedfacets=&format=json&type=avancee&rows=2")
    }
}
